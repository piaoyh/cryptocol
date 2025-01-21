#[allow(unused_variables)]
#[allow(dead_code)]

use std::convert::From;
use std::str::FromStr;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };

use crate::number::{ SmallUInt, NumberErr };

/// big_uint.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_uint_arithmetic.rs.
pub struct BigUInt<T, const N: usize>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // Dummy struct for documentation
    #[allow(dead_code)] number: [T; N],
    #[allow(dead_code)] flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone + Display + Debug + ToString
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign
        + BitOr<Self, Output = Self> + BitOrAssign
        + BitXor<Self, Output = Self> + BitXorAssign
        + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]> + From<u32>
{
    /***** CONSTRUCTORS *****/

    // pub fn new() -> Self
    /// Constructs a new `BigUInt<T, N>`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>`.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let obj = U256::new();
    /// println!("obj = {}", obj);
    /// assert_eq!(obj.to_string(), "0");
    /// assert_eq!(obj.is_overflow(), false);
    /// assert_eq!(obj.is_underflow(), false);
    /// assert_eq!(obj.is_infinity(), false);
    /// assert_eq!(obj.is_divided_by_zero(), false);
    /// assert_eq!(obj.is_undefined(), false);
    /// assert_eq!(obj.is_left_carry(), false);
    /// assert_eq!(obj.is_right_carry(), false);
    /// ```
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn zero() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of `0`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents `0`.
    /// 
    /// # Features
    /// This function calls `BigUInt<T, N>::new()`, so it is
    /// virtually exactly the same as the function
    /// `BigUInt<T, N>::new()`.
    /// 
    /// # Benefit
    /// Your source code will be better readable if you use
    /// `BigUInt<T, N>::zero()` instead of
    /// `BigUInt<T, N>::new()` especially
    /// when you create the big number zero.
    ///
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let zero = U256::zero();
    /// println!("zero = {}", zero);
    /// assert_eq!(zero.to_string(), "0");
    /// assert_eq!(zero.is_overflow(), false);
    /// assert_eq!(zero.is_underflow(), false);
    /// assert_eq!(zero.is_infinity(), false);
    /// assert_eq!(zero.is_divided_by_zero(), false);
    /// assert_eq!(zero.is_undefined(), false);
    /// assert_eq!(zero.is_left_carry(), false);
    /// assert_eq!(zero.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn zero() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn one() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of `1`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents `1`.
    /// 
    /// # Benefit
    /// Your source code will be better readable if you use
    /// `BigUInt<T, N>::one()` instead of
    /// `BigUInt<T, N>::new()` and then
    /// `set_uint(1)` especially when you create the big number `1`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let one = U256::one();
    /// println!("one = {}", one);
    /// assert_eq!(one.to_string(), "1");
    /// assert_eq!(one.is_overflow(), false);
    /// assert_eq!(one.is_underflow(), false);
    /// assert_eq!(one.is_infinity(), false);
    /// assert_eq!(one.is_divided_by_zero(), false);
    /// assert_eq!(one.is_undefined(), false);
    /// assert_eq!(one.is_left_carry(), false);
    /// assert_eq!(one.is_right_carry(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn one() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn max() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of
    /// maximum.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents maximum value.
    /// 
    /// # Features
    /// All bits are set to be `1`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let maximum = U256::max();
    /// assert_eq!(maximum.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(maximum.is_overflow(), false);
    /// assert_eq!(maximum.is_underflow(), false);
    /// assert_eq!(maximum.is_infinity(), false);
    /// assert_eq!(maximum.is_divided_by_zero(), false);
    /// assert_eq!(maximum.is_undefined(), false);
    /// assert_eq!(maximum.is_left_carry(), false);
    /// assert_eq!(maximum.is_right_carry(), false);
    /// assert_eq!(maximum.wrapping_add_uint(1_u16), U256::zero());
    /// ```
    pub fn max() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn submax(_size_in_bits: usize) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object which has the
    /// value of `size_in_bits`-bit long maximum value in which all bits are
    /// set to be `1`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents
    /// `size_in_bits`-bit long maximum value.
    /// 
    /// # Features
    /// This method will make all the `size_in_bits` bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let half = U256::submax(128_usize);
    /// println!("half maximum = \t{}", half);
    /// println!("half maximum = \t{}", half.to_string_with_radix_and_stride(16, 4).unwrap());
    /// assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    /// assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(half.is_overflow(), false);
    /// assert_eq!(half.is_underflow(), false);
    /// assert_eq!(half.is_infinity(), false);
    /// assert_eq!(half.is_divided_by_zero(), false);
    /// assert_eq!(half.is_undefined(), false);
    /// assert_eq!(half.is_left_carry(), false);
    /// assert_eq!(half.is_right_carry(), false);
    /// ```
    pub fn submax(_size_in_bits: usize) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn halfmax() -> Self
    /// Constructs a new `BigUInt<T, N>`-type object which has the
    /// value of half-length maximum value in which all bits are set to be `1`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents
    /// a half-length maximum value.
    /// 
    /// # Features
    /// This method will make all the lower half bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let half = U256::halfmax();
    /// println!("half maximum = {0} = {0:#x}", half);
    /// assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    /// assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(half.is_overflow(), false);
    /// assert_eq!(half.is_underflow(), false);
    /// assert_eq!(half.is_infinity(), false);
    /// assert_eq!(half.is_divided_by_zero(), false);
    /// assert_eq!(half.is_undefined(), false);
    /// assert_eq!(half.is_left_carry(), false);
    /// assert_eq!(half.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn halfmax() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn from_uint<U>(_val: U) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object from an
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same
    /// value of `val`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with_u16;
    /// define_utypes_with_u16!();
    /// 
    /// let a_from_u8 = U512::from_uint(123_u8);
    /// let b_from_u16 = U512::from_uint(12345_u16);
    /// let c_from_u32 = U512::from_uint(1234567890_u32);
    /// let d_from_u64 = U512::from_uint(12345678901234567890_u64);
    /// let e_from_u128 = U512::from_uint(123456789012345678901234567890123456789_u128);
    /// let f_from_usize = U512::from_uint(123_usize);
    /// 
    /// println!("a_from_u8 = {}", a_from_u8);
    /// println!("b_from_u16 = {}", b_from_u16);
    /// println!("c_from_u32 = {}", c_from_u32);
    /// println!("d_from_u64 = {}", d_from_u64);
    /// println!("e_from_u128 = {}", e_from_u128);
    /// println!("f_from_usize = {}", f_from_usize);
    /// 
    /// assert_eq!(a_from_u8.into_u8(), 123_u8);
    /// assert_eq!(a_from_u8.is_overflow(), false);
    /// assert_eq!(a_from_u8.is_underflow(), false);
    /// assert_eq!(a_from_u8.is_infinity(), false);
    /// assert_eq!(a_from_u8.is_divided_by_zero(), false);
    /// assert_eq!(a_from_u8.is_undefined(), false);
    /// assert_eq!(a_from_u8.is_left_carry(), false);
    /// assert_eq!(a_from_u8.is_right_carry(), false);
    /// 
    /// assert_eq!(b_from_u16.into_u16(), 12345_u16);
    /// assert_eq!(b_from_u16.is_overflow(), false);
    /// assert_eq!(b_from_u16.is_underflow(), false);
    /// assert_eq!(b_from_u16.is_infinity(), false);
    /// assert_eq!(b_from_u16.is_divided_by_zero(), false);
    /// assert_eq!(b_from_u16.is_undefined(), false);
    /// assert_eq!(b_from_u16.is_left_carry(), false);
    /// assert_eq!(b_from_u16.is_right_carry(), false);
    /// 
    /// assert_eq!(c_from_u32.into_u32(), 1234567890_u32);
    /// assert_eq!(c_from_u32.is_underflow(), false);
    /// assert_eq!(c_from_u32.is_infinity(), false);
    /// assert_eq!(c_from_u32.is_divided_by_zero(), false);
    /// assert_eq!(c_from_u32.is_undefined(), false);
    /// assert_eq!(c_from_u32.is_left_carry(), false);
    /// assert_eq!(c_from_u32.is_right_carry(), false);
    /// 
    /// assert_eq!(d_from_u64.into_u64(), 12345678901234567890_u64);
    /// assert_eq!(d_from_u64.is_overflow(), false);
    /// assert_eq!(d_from_u64.is_underflow(), false);
    /// assert_eq!(d_from_u64.is_infinity(), false);
    /// assert_eq!(d_from_u64.is_divided_by_zero(), false);
    /// assert_eq!(d_from_u64.is_undefined(), false);
    /// assert_eq!(d_from_u64.is_left_carry(), false);
    /// assert_eq!(d_from_u64.is_right_carry(), false);
    /// 
    /// assert_eq!(e_from_u128.into_u128(), 123456789012345678901234567890123456789_u128);
    /// assert_eq!(e_from_u128.is_overflow(), false);
    /// assert_eq!(e_from_u128.is_underflow(), false);
    /// assert_eq!(e_from_u128.is_infinity(), false);
    /// assert_eq!(e_from_u128.is_divided_by_zero(), false);
    /// assert_eq!(e_from_u128.is_undefined(), false);
    /// assert_eq!(e_from_u128.is_left_carry(), false);
    /// assert_eq!(e_from_u128.is_right_carry(), false);
    /// 
    /// assert_eq!(f_from_usize.into_usize(), 123_usize);
    /// assert_eq!(f_from_usize.is_overflow(), false);
    /// assert_eq!(f_from_usize.is_underflow(), false);
    /// assert_eq!(f_from_usize.is_infinity(), false);
    /// assert_eq!(f_from_usize.is_divided_by_zero(), false);
    /// assert_eq!(f_from_usize.is_undefined(), false);
    /// assert_eq!(f_from_usize.is_left_carry(), false);
    /// assert_eq!(f_from_usize.is_right_carry(), false);
    /// ```
    pub fn from_uint<U>(_val: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn from_array(_val: [T; N]) -> Self
    /// Constructs a new `BigUInt<T, N>` from an array of type `T` with `N`
    /// elements.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of array `val`.
    /// 
    /// # Counterpart Method
    /// You can also use the method [from()](struct@BigUInt#impl-From<[T;+N]>-for-BigUInt<T,+N>)
    /// implemented by implementation of trait `From<[T;N]>`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let big_num = U256::from_array([10_u8; 32]);
    /// println!("big_num = {:X}", big_num);
    /// assert_eq!(big_num.to_string_with_radix(16).unwrap(), "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A");
    /// assert_eq!(big_num.is_overflow(), false);
    /// assert_eq!(big_num.is_underflow(), false);
    /// assert_eq!(big_num.is_infinity(), false);
    /// assert_eq!(big_num.is_divided_by_zero(), false);
    /// assert_eq!(big_num.is_undefined(), false);
    /// assert_eq!(big_num.is_left_carry(), false);
    /// assert_eq!(big_num.is_right_carry(), false);
    /// ```
    pub fn from_array(_val: [T; N]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn from_biguint<U, const M: usize>(_biguint: &BigUInt<U, M>) -> Self
    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Features
    /// It copies not only long-bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Example 1 for the same length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let mut a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// a_u512_with_u8.set_overflow();
    /// a_u512_with_u8.set_underflow();
    /// a_u512_with_u8.set_infinity();
    /// a_u512_with_u8.set_divided_by_zero();
    /// a_u512_with_u8.set_undefined();
    /// a_u512_with_u8.set_left_carry();
    /// a_u512_with_u8.set_right_carry();
    /// assert_eq!(a_u512_with_u8.is_overflow(), true);
    /// assert_eq!(a_u512_with_u8.is_underflow(), true);
    /// assert_eq!(a_u512_with_u8.is_infinity(), true);
    /// assert_eq!(a_u512_with_u8.is_divided_by_zero(), true);
    /// assert_eq!(a_u512_with_u8.is_undefined(), true);
    /// assert_eq!(a_u512_with_u8.is_left_carry(), true);
    /// assert_eq!(a_u512_with_u8.is_right_carry(), true);
    /// 
    /// let b_u512_with_u8 = U512_with_u8::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u512_with_u8 = {}", b_u512_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u512_with_u8.is_overflow(), false);
    /// assert_eq!(b_u512_with_u8.is_underflow(), false);
    /// assert_eq!(b_u512_with_u8.is_infinity(), false);
    /// assert_eq!(b_u512_with_u8.is_divided_by_zero(), false);
    /// assert_eq!(b_u512_with_u8.is_undefined(), false);
    /// assert_eq!(b_u512_with_u8.is_left_carry(), false);
    /// assert_eq!(b_u512_with_u8.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for the shorter length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let mut a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// a_u512_with_u8.set_overflow();
    /// a_u512_with_u8.set_underflow();
    /// a_u512_with_u8.set_infinity();
    /// a_u512_with_u8.set_divided_by_zero();
    /// a_u512_with_u8.set_undefined();
    /// a_u512_with_u8.set_left_carry();
    /// a_u512_with_u8.set_right_carry();
    /// assert_eq!(a_u512_with_u8.is_overflow(), true);
    /// assert_eq!(a_u512_with_u8.is_underflow(), true);
    /// assert_eq!(a_u512_with_u8.is_infinity(), true);
    /// assert_eq!(a_u512_with_u8.is_divided_by_zero(), true);
    /// assert_eq!(a_u512_with_u8.is_undefined(), true);
    /// assert_eq!(a_u512_with_u8.is_left_carry(), true);
    /// assert_eq!(a_u512_with_u8.is_right_carry(), true);
    /// 
    /// let b_u256_with_u8 = U256_with_u16::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u256_with_u8 = {}", b_u256_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u256_with_u8.to_string(), "98633800081229720571026865697976779988382011787853764870844783447569204535061");
    /// assert_eq!(b_u256_with_u8.is_overflow(), false);
    /// assert_eq!(b_u256_with_u8.is_underflow(), false);
    /// assert_eq!(b_u256_with_u8.is_infinity(), false);
    /// assert_eq!(b_u256_with_u8.is_divided_by_zero(), false);
    /// assert_eq!(b_u256_with_u8.is_undefined(), false);
    /// assert_eq!(b_u256_with_u8.is_left_carry(), false);
    /// assert_eq!(b_u256_with_u8.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the longer length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let mut a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// a_u512_with_u8.set_overflow();
    /// a_u512_with_u8.set_underflow();
    /// a_u512_with_u8.set_infinity();
    /// a_u512_with_u8.set_divided_by_zero();
    /// a_u512_with_u8.set_undefined();
    /// a_u512_with_u8.set_left_carry();
    /// a_u512_with_u8.set_right_carry();
    /// assert_eq!(a_u512_with_u8.is_overflow(), true);
    /// assert_eq!(a_u512_with_u8.is_underflow(), true);
    /// assert_eq!(a_u512_with_u8.is_infinity(), true);
    /// assert_eq!(a_u512_with_u8.is_divided_by_zero(), true);
    /// assert_eq!(a_u512_with_u8.is_undefined(), true);
    /// assert_eq!(a_u512_with_u8.is_left_carry(), true);
    /// assert_eq!(a_u512_with_u8.is_right_carry(), true);
    /// 
    /// let b_u1024_with_u8 = U1024_with_u16::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u1024_with_u8 = {}", b_u1024_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u1024_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u1024_with_u8.is_overflow(), false);
    /// assert_eq!(b_u1024_with_u8.is_underflow(), false);
    /// assert_eq!(b_u1024_with_u8.is_infinity(), false);
    /// assert_eq!(b_u1024_with_u8.is_divided_by_zero(), false);
    /// assert_eq!(b_u1024_with_u8.is_undefined(), false);
    /// assert_eq!(b_u1024_with_u8.is_left_carry(), false);
    /// assert_eq!(b_u1024_with_u8.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_biguint<U, const M: usize>(_biguint: &BigUInt<U, M>) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn from_be(_be: Self) -> Self
    /// Converts a big unsigned integer from big endian to the target’s
    /// endianness.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let be = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
    ///                             0x1122, 0x3344, 0x5566, 0x7788,
    ///                             0x9900, 0xaabb, 0xccdd, 0xeeff,
    ///                             0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    /// let le = U256::from_be(be.clone());
    /// println!("be = {:#x}", be);
    /// println!("le = {:#x}", le);
    /// #[cfg(target_endian = "little")]
    /// {
    ///     assert_eq!(be.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    ///     assert_eq!(le.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");        
    /// }
    /// #[cfg(target_endian = "big")]
    /// {
    ///     assert_eq!(be.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    ///     assert_eq!(le.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");        
    /// }
    /// assert_eq!(le.is_overflow(), false);
    /// assert_eq!(le.is_underflow(), false);
    /// assert_eq!(le.is_infinity(), false);
    /// assert_eq!(le.is_divided_by_zero(), false);
    /// assert_eq!(le.is_undefined(), false);
    /// assert_eq!(le.is_left_carry(), false);
    /// assert_eq!(le.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_be(_be: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn from_be_bytes(_be_bytes: [T; N]) -> Self
    /// Create a native endian unsigned integer value from its representation
    /// as a byte array in big endian.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let be_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
    ///                 0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    /// let le = U256::from_be_bytes(be_array.clone());
    /// print!("be_array = ");
    /// for elem in be_array
    ///     { print!("{:#8x} ", elem); }
    /// println!();
    /// println!("le = {#x}", le);
    /// #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "78563412_EFCDAB90_44332211_88776655_BBAA0099_FFEEDDCC_4C3D2E1F_89706A5B");
    /// #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    /// assert_eq!(le.is_overflow(), false);
    /// assert_eq!(le.is_underflow(), false);
    /// assert_eq!(le.is_infinity(), false);
    /// assert_eq!(le.is_divided_by_zero(), false);
    /// assert_eq!(le.is_undefined(), false);
    /// assert_eq!(le.is_left_carry(), false);
    /// assert_eq!(le.is_right_carry(), false);
    /// ```
    pub fn from_be_bytes(_be_bytes: [T; N]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn from_le(_le: Self) -> Self
    /// Converts a big unsigned integer from little endian to the target’s
    /// endianness.
    /// 
    /// # Features
    /// - On little endian this is a no-op.
    /// - On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let le1 = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
    ///                 0x1122, 0x3344, 0x5566, 0x7788,
    ///                 0x9900, 0xaabb, 0xccdd, 0xeeff,
    ///                 0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    /// let le2 = U256::from_le(le1.clone());
    /// println!("le1 = {:#x}", le1);
    /// println!("le2 = {:#x}", le2);
    /// #[cfg(target_endian = "little")]
    /// {
    ///     assert_eq!(le1.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    ///     assert_eq!(le2.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    /// }
    /// #[cfg(target_endian = "big")]
    /// {
    ///     assert_eq!(le1.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    ///     assert_eq!(le2.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");
    /// }
    /// assert_eq!(le2.is_overflow(), false);
    /// assert_eq!(le2.is_underflow(), false);
    /// assert_eq!(le2.is_infinity(), false);
    /// assert_eq!(le2.is_divided_by_zero(), false);
    /// assert_eq!(le2.is_undefined(), false);
    /// assert_eq!(le2.is_left_carry(), false);
    /// assert_eq!(le2.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_le(_le: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    // pub fn from_le_bytes(_le_bytes: [T; N]) -> Self
    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    /// 
    /// # Features
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let le_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
    ///                 0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    /// let le = U256::from_le_bytes(le_array.clone());
    /// print!("le_array = ");
    /// for elem in le_array
    ///     { print!("{:#8x} ", elem); }
    /// println!();
    /// println!("le = {#x}", le);
    /// #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "5B6A7089_1F2E3D4C_CCDDEEFF_9900AABB_55667788_11223344_90ABCDEF_12345678");
    /// #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    /// assert_eq!(le.is_overflow(), false);
    /// assert_eq!(le.is_underflow(), false);
    /// assert_eq!(le.is_infinity(), false);
    /// assert_eq!(le.is_divided_by_zero(), false);
    /// assert_eq!(le.is_undefined(), false);
    /// assert_eq!(le.is_left_carry(), false);
    /// assert_eq!(le.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_le_bytes(_le_bytes: [T; N]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    //  pub fn from_string(_txt: &str) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a string of decimal number.
    /// 
    /// # Output
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns one of
    /// `Err(NumberErr::NotAlphaNumeric)`, `Err(NumberErr::NotFitToRadix)`,
    /// and `Err(NumberErr::TooBigNumber)` according to its failure reason.
    /// 
    /// # Delimiter _
    /// In the number expression in a string, you can separate the digits with
    /// '_' in order to make it more readable. So, "1_0000" and "10_000" are all
    /// the same as"10000".
    /// 
    /// # Errors
    /// | priority | argument | value                                           | Caused Error                 |
    /// |----------|----------|-------------------------------------------------|------------------------------|
    /// | 1st      | `txt`    | contains any non-alphanumeric letter except '_' | `NumberErr::NotAlphaNumeric` |
    /// | 2nd      | `txt`    | contains any letter other than number           | `NumberErr::NotFitToRadix`   |
    /// | 3rd      | `txt`    | expresses bigger number than maximum value      | `NumberErr::TooBigNumber`    |
    /// 
    /// When multiple errors were caused, only the error with higher priority is
    /// issued. `1st` is higher than `2nd`, and so on.
    /// 
    /// # Example 1 for correct case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_correct = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    /// match a_correct
    /// {
    ///     Ok(n) => {
    ///         println!("a_correct = {}", n);
    ///         assert_eq!(n.to_string(), "1234567890123456789012345678901234567890123456789012345678901234567890");
    ///         assert_eq!(n.is_overflow(), false);
    ///         assert_eq!(n.is_underflow(), false);
    ///         assert_eq!(n.is_infinity(), false);
    ///         assert_eq!(n.is_divided_by_zero(), false);
    ///         assert_eq!(n.is_undefined(), false);
    ///         assert_eq!(n.is_left_carry(), false);
    ///         assert_eq!(n.is_right_carry(), false);
    ///     },
    ///     Err(e) => { println!("Failed: {}", e); },
    /// }
    /// ```
    /// 
    /// # Example 2 for NumberErr::NotAlphaNumeric case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let b_contains_non_alphanumeric = U256::from_string("12345+67890");
    /// match b_contains_non_alphanumeric
    /// {
    ///     Ok(n) =>  { println!("a_correct = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 3 for NumberErr::NotFitToRadix case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let c_constains_not_fit_to_radix = U256::from_string("1234567890a");
    /// match c_constains_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("c_constains_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::NotFitToRadix);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 4 for NumberErr::TooBigNumber case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let d_constains_too_big_number = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    /// match d_constains_too_big_number
    /// {
    ///     Ok(n) =>  { println!("c_constains_too_big_number = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::TooBigNumber);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 5 for NumberErr::NotAlphaNumeric and NumberErr::NotFitToRadix case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let e_contains_non_alphanumeric_not_fit_to_radix = U256::from_string("F12345+67890");
    /// match e_contains_non_alphanumeric_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("e_contains_non_alphanumeric_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::NotFitToRadix); // The first letter is 'F'.
    ///     },
    /// }
    /// ```
    #[inline]
    pub fn from_string(_txt: &str) -> Result<Self, NumberErr>
    {
        unimplemented!(); // Dummy code for documentation
    }

    //  pub fn from_str_radix(_txt: &str, _radix: u32) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a string with `radix`.
    /// 
    /// # Output
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns one of
    /// `Err(NumberErr::OutOfValidRadixRange)`, `Err(NumberErr::NotAlphaNumeric)`,
    /// and `Err(NumberErr::NotFitToRadix)` according to its failure reason.
    /// 
    /// # Errors
    /// - If the argument `txt` of this method includes any letters other than
    /// alphanumeric letter(s),
    /// it will return`Err(NumberErr::NotAlphaNumeric)`.
    /// - If the argument `radix` of this method is out of the valid range from
    /// `2` up to `62` inclusively,
    /// it will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// - If the argument `txt` of this method includes any letter(s) out of
    /// the valid letter range even if they are alphanumeric, it will return
    /// `Err(NumberErr::NotFitToRadix)`. For example, in the case of hexadecimal
    /// number system which means that the argument radix is `16`, if the
    /// argument `txt` includes 'g',
    /// it will return `Err(NumberErr::NotFitToRadix)`.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Delimiter _
    /// In the number expression in a string, you can separate the digits with
    /// '_' in order to make it more readable. So, "10000" is the same as
    /// "1_0000".
    /// 
    /// # Errors
    /// | priority | argument | value                                           | Caused Error                      |
    /// |----------|----------|-------------------------------------------------|-----------------------------------|
    /// | 1st      | `radix`  | less than `2` or greater than `62`              | `NumberErr::OutOfValidRadixRange` |
    /// | 2nd      | `txt`    | contains any non-alphanumeric letter except '_' | `NumberErr::NotAlphaNumeric`      |
    /// | 3rd      | `txt`    | contains any letter or number out of `radix`    | `NumberErr::NotFitToRadix`        |
    /// | 4th      | `txt`    | expresses bigger number than maximum value      | `NumberErr::TooBigNumber`         |
    /// 
    /// When multiple errors were caused, only the error with higher priority is
    /// issued. `1st` is higher than `2nd`, and so on.
    /// 
    /// # Example 1 for correct case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_correct = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    /// match a_correct
    /// {
    ///     Ok(n) => {
    ///         println!("a_correct = {}", n);
    ///         assert_eq!(n.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0");
    ///         assert_eq!(n.is_overflow(), false);
    ///         assert_eq!(n.is_underflow(), false);
    ///         assert_eq!(n.is_infinity(), false);
    ///         assert_eq!(n.is_divided_by_zero(), false);
    ///         assert_eq!(n.is_undefined(), false);
    ///         assert_eq!(n.is_left_carry(), false);
    ///         assert_eq!(n.is_right_carry(), false);
    ///     },
    ///     Err(e) => { println!("Failed: {}", e); },
    /// }
    /// ```
    /// 
    /// # Example 2 for NumberErr::OutOfValidRadixRange case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let b_contains_out_of_valid_radix_range = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 63);
    /// match b_contains_out_of_valid_radix_range
    /// {
    ///     Ok(n) =>  { println!("a_correct = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::OutOfValidRadixRange);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 3 for NumberErr::NotAlphaNumeric case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let c_contains_non_alphanumeric = U512::from_str_radix("1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0", 16);
    /// match c_contains_non_alphanumeric
    /// {
    ///     Ok(n) =>  { println!("a_correct = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 4 for NumberErr::NotFitToRadix case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let d_constains_not_fit_to_radix = U512::from_str_radix("1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG", 16);
    /// match d_constains_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("d_constains_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::NotFitToRadix);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 5 for NumberErr::TooBigNumber case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let e_constains_too_big_number = U512::from_str_radix("1_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    /// match e_constains_too_big_number
    /// {
    ///     Ok(n) =>  { println!("c_constains_too_big_number = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::TooBigNumber);
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 6 for NumberErr::NotAlphaNumeric, NumberErr::NotFitToRadix, and NumberErr::TooBigNumber case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let f_contains_non_alphanumeric_not_fit_to_radix = U512::from_str_radix("1,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG", 16);
    /// match f_contains_non_alphanumeric_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("f_contains_non_alphanumeric_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         println!("Failed: {}", e);
    ///         assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///     },
    /// }
    /// ```
    pub fn from_str_radix(_txt: &str, _radix: u32) -> Result<Self, NumberErr>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn generate_check_bits(_bit_pos: usize) -> Option<Self>
    /// Constucts a new `BigUInt<T, N>` which has the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Output
    /// It returns a big unsigned integer `BigUInt<T, N>` whose bit specified
    /// by the argument bit_posvalue is set to be 1, wrapped by enum
    /// `Some(self)` of `Option<Self>` if the bit positon `bit_pos` is less
    /// than `size_of::<T>() * N * 8`. It returns `None` if the bit positon
    /// `bit_pos` is greater than or equal to `size_of::<T>() * N * 8`.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = U256::generate_check_bits(0).unwrap();
    /// println!("a_0 = {:#b}", a_0);
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// assert_eq!(a_0.is_overflow(), false);
    /// assert_eq!(a_0.is_underflow(), false);
    /// assert_eq!(a_0.is_infinity(), false);
    /// assert_eq!(a_0.is_divided_by_zero(), false);
    /// assert_eq!(a_0.is_undefined(), false);
    /// assert_eq!(a_0.is_left_carry(), false);
    /// assert_eq!(a_0.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_12 = U256::generate_check_bits(12).unwrap();
    /// println!("a_12 = {:#b}", a_12);
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// assert_eq!(a_12.is_overflow(), false);
    /// assert_eq!(a_12.is_underflow(), false);
    /// assert_eq!(a_12.is_infinity(), false);
    /// assert_eq!(a_12.is_divided_by_zero(), false);
    /// assert_eq!(a_12.is_undefined(), false);
    /// assert_eq!(a_12.is_left_carry(), false);
    /// assert_eq!(a_12.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_255 = U256::generate_check_bits(255).unwrap();
    /// println!("a_255 = {:#b}", a_255);
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// assert_eq!(a_255.is_overflow(), false);
    /// assert_eq!(a_255.is_underflow(), false);
    /// assert_eq!(a_255.is_infinity(), false);
    /// assert_eq!(a_255.is_divided_by_zero(), false);
    /// assert_eq!(a_255.is_undefined(), false);
    /// assert_eq!(a_255.is_left_carry(), false);
    /// assert_eq!(a_255.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_256 = U256::generate_check_bits(256);
    /// match a_256
    /// {
    ///     Some(n) => { println!("a_256 = {:#b}", n); },
    ///     None => { assert_eq!(a_256, None); },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn generate_check_bits(_bit_pos: usize) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn generate_check_bits_(_bit_pos: usize) -> Self
    /// Constucts a new `BigUInt<T, N>` which has the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic.
    /// 
    /// # Output
    /// It returns a big unsigned integer `BigUInt<T, N>` whose bit specified
    /// by the argument bit_posvalue is set to be 1.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = U256::generate_check_bits_(0);
    /// println!("a_0 = {:#b}", a_0);
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// assert_eq!(a_0.is_overflow(), false);
    /// assert_eq!(a_0.is_underflow(), false);
    /// assert_eq!(a_0.is_infinity(), false);
    /// assert_eq!(a_0.is_divided_by_zero(), false);
    /// assert_eq!(a_0.is_undefined(), false);
    /// assert_eq!(a_0.is_left_carry(), false);
    /// assert_eq!(a_0.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_12 = U256::generate_check_bits_(12);
    /// println!("a_12 = {:#b}", a_12);
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// assert_eq!(a_12.is_overflow(), false);
    /// assert_eq!(a_12.is_underflow(), false);
    /// assert_eq!(a_12.is_infinity(), false);
    /// assert_eq!(a_12.is_divided_by_zero(), false);
    /// assert_eq!(a_12.is_undefined(), false);
    /// assert_eq!(a_12.is_left_carry(), false);
    /// assert_eq!(a_12.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_255 = U256::generate_check_bits_(255);
    /// println!("a_255 = {:#b}", a_255);
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// assert_eq!(a_255.is_overflow(), false);
    /// assert_eq!(a_255.is_underflow(), false);
    /// assert_eq!(a_255.is_infinity(), false);
    /// assert_eq!(a_255.is_divided_by_zero(), false);
    /// assert_eq!(a_255.is_undefined(), false);
    /// assert_eq!(a_255.is_left_carry(), false);
    /// assert_eq!(a_255.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// // It will panic!
    /// let a_256 = U256::generate_check_bits_(256);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn generate_check_bits_(_bit_pos: usize) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }


    
    /***** METHODS TO GET SIZE BOTH IN BYTES AND BITS *****/

    // pub fn size_in_bytes() -> usize
    /// Returns how many bytes long the number `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bytes.
    /// 
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// println!("U256 is {}-byte integer.", U256::size_in_bytes());
    /// assert_eq!(U256::size_in_bytes(), 32);
    /// ```
    #[inline]
    pub fn size_in_bytes() -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn size_in_bits() -> usize
    /// Returns how many bits long the number `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bits.
    /// 
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// println!("U256 is {}-bit integer.", U256::size_in_bits());
    /// assert_eq!(U256::size_in_bits(), 256);
    /// ```
    #[inline]
    pub fn size_in_bits() -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn length_in_bytes(&self) -> usize
    /// Returns how many bytes long the number i.e. the object of
    /// `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bytes.
    /// 
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = U256::from_str_radix("A16F", 16).unwrap();
    /// println!("a is {}-byte integer.", a.length_in_bytes());
    /// assert_eq!(a.length_in_bytes(), 32);
    /// ```
    #[inline]
    pub fn length_in_bytes(&self) -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn length_in_bits(&self) -> usize
    /// Returns how many bits long the number i.e. the object of
    /// `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bits.
    /// 
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_str_radix("A16F", 16).unwrap();
    /// println!("a is {}-bit integer.", a.length_in_bits());
    /// assert_eq!(a.length_in_bits(), 256);
    /// ```
    #[inline]
    pub fn length_in_bits(&self) -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** METHODS TO GET, SET, AND CHECK *****/

    // pub fn turn_check_bits(&mut self, _bit_pos: usize)
    /// Changes a `BigUInt<T, N>` to have the value zero and sets only
    /// the bit specified by the argument `bit_pos` to be 1.
    /// 
    /// # Argumentss
    /// The bit positon `bit_pos` is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the `bit_pos` is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_string("256487951236974125896345564889974258").unwrap();
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// a.turn_check_bits(102);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a, U256::from_str_radix("1000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000", 2).unwrap());
    /// ```
    /// 
    /// # Panic Example
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_string("256487951236974125896345564889974258").unwrap();
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // It will panic.
    /// a.turn_check_bits(256);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn turn_check_bits(&mut self, _bit_pos: usize)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_bit_set(&self, _bit_pos: usize) -> Option<bool>
    /// Check a `self` to know whether or not the bit specified by the argument
    /// `bit_pos` to be 1.
    /// 
    /// # Bit Position
    /// The bit positon `bit_pos` is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endianness. So, if the `bit_pos`
    /// is `0`, only LSB is set to be `1` and all the other bits will be set
    /// to `0`.
    /// 
    /// # Output
    /// If the bit specified by `bit_pos` is set to be one, this method returns
    /// `Some(true)` of enum `Option<bool>`. If the bit specified by `bit_pos`
    /// is set to be zero, this method returns `Some(true)` of enum
    /// `Option<bool>`. If the bit positon `bit_pos` is greater than or equal
    /// to `size_of::<T>() * N * 8`, this method returns `None`.
    /// 
    /// # Counterpart method
    /// If you are sure that `bit_pos` is less than `size_of::<T>() * N * 8`,
    /// you can use the method `is_bit_set_()` for better performance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    /// let mut res = a_biguint.is_bit_set(151);
    /// match res
    /// {
    ///     Some(r) => {
    ///         println!("The {}th bit is set: {}", 151, r);
    ///         assert_eq!(a_biguint.is_bit_set_(151), true);
    ///         assert_eq!(a_biguint.is_overflow(), false);
    ///         assert_eq!(a_biguint.is_underflow(), false);
    ///         assert_eq!(a_biguint.is_infinity(), false);
    ///         assert_eq!(a_biguint.is_undefined(), true);
    ///         assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    ///     },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a_biguint, 151);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    /// res = a_biguint.is_bit_set(200);
    /// match res
    /// {
    ///     Some(r) => {
    ///         println!("The {}th bit is set: {}", 200, r);
    ///         assert_eq!(a_biguint.is_bit_set_(200), false);
    ///         assert_eq!(a_biguint.is_overflow(), false);
    ///         assert_eq!(a_biguint.is_underflow(), false);
    ///         assert_eq!(a_biguint.is_infinity(), false);
    ///         assert_eq!(a_biguint.is_undefined(), true);
    ///         assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    ///     },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a_biguint, 200);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    /// res = a_biguint.is_bit_set(300);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}th bit is set: {}", 300, r); },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a_biguint, 300);
    ///         assert_eq!(a_biguint.is_bit_set(300), None);
    ///     }
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_bit_set(&self, _bit_pos: usize) -> Option<bool>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_bit_set_(&self, _bit_pos: usize) -> bool
    /// Check whether or not the bit specified by the argument
    /// `bit_pos` in `self` to be 1.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic. So, you are highly
    /// recommended to use only when you are sure that `bit_pos` is neither
    /// greater than nor equal to `size_of::<T>() * N * 8`. Otherwise, use
    /// the method `is_bit_set()`.
    /// 
    /// # Bit Position
    /// The bit positon `bit_pos` is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endianness. So, if the `bit_pos`
    ///  is `0`, only LSB is set to be `1` and all the other bits will be set
    /// to `0`.
    /// 
    /// # Output
    /// If the bit specified by `bit_pos` is set to be one, this method returns
    /// `true`. If the bit specified by `bit_pos` is set to be zero, this
    /// method returns `false`.
    /// 
    /// # Counterpart method
    /// If you are not sure that `bit_pos` is less than `size_of::<T>() * N * 8`,
    /// you are highly encouraged to use the method `is_bit_set()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a = {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    /// println!("The {}th bit is set: {}", 151, a.is_bit_set_(151));
    /// assert_eq!(a.is_bit_set_(151), true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("The {}th bit is set: {}", 200, a.is_bit_set_(200));
    /// assert_eq!(a.is_bit_set_(200), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// // It will panic!!!
    /// println!("The {}th bit is set: {}", 300, a.is_bit_set_(300));
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_bit_set_(&self, _bit_pos: usize) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_upper_portion(_portion: usize) -> Self
    /// Get the non-zero upper portion (high order part) from `self`.
    /// 
    /// # Argument
    /// The argument `portion` specifies the length of the high order part to
    /// take in bits.
    /// 
    /// # Output
    /// - If `portion` is bigger than or equal to the length of the non-zero
    /// part of `self`, this method returns `self`. Here, non-zero part of
    /// `00101100` is not `1011` but `101100` for example.
    /// - If `portion` is less than the length of the non-zero part of `self`,
    /// this method returns the high order part of `self` as many bits as
    /// specified by `portion`. Here, non-zero part of `00101100` is not `1011`
    /// but `101100` for example.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// let b = a.get_upper_portion(10);
    /// println!("The 10-bit upper portion of {}_U256 is {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101101001");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_upper_portion(&self, _portion: usize) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_lower_portion(_portion: usize) -> Self
    /// Get the lower portion (low order part) from `self`.
    /// 
    /// # Argument
    /// The argument `portion` specifies the length of the low order part to
    /// take in bits.
    /// 
    /// # Output
    /// - If `portion` is bigger than or equal to the length of the non-zero
    /// part of `self`, this method returns `self`. Here, non-zero part of
    /// `00101100` is not `1011` but `101100` for example.
    /// - If `portion` is less than the length of the non-zero part of `self`,
    /// this method returns the low order part of `self` as many bits as
    /// specified by `portion`. Here, non-zero part of `00101100` is not `1011`
    /// but `101100` for example.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912340").unwrap();
    /// let b = a.get_lower_portion(10);
    /// println!("The 10-bit lower portion of {}_U256 is {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101010100");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_lower_portion(&self, _portion: usize) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_num(&self, _i: usize) -> Option<T>
    /// Returns i-th element of its array of type `T` wrapped in Some
    /// of enum Option if `i` < `N`. Otherwise, it returns `None`.
    /// 
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it returns `None`.
    /// 
    /// # Counterpart Method
    /// When you are sure that `i` < `N`, you may want to use its counterpart
    /// method [get_num_()](struct@BigUInt#method.get_num_) for performance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// match a.get_num(3)
    /// {
    ///     Some(num) => {
    ///         println!("a.get_num(3).unwrap() = {}", num);
    ///         assert_eq!(num, 30);
    ///     },
    ///     None => {
    ///         println!("There is no third element.");
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let f = a.get_num(8);
    /// match f
    /// {
    ///     Some(num) => {
    ///         println!("a.get_num(3).unwrap() = {}", num);
    ///     },
    ///     None => {
    ///         println!("There is no third element.");
    ///         assert_eq!(f, None);
    ///     },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_num(&self, _i: usize) -> Option<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_num_(&self, _i: usize) -> T
    /// Returns i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This method is performance-oriented and does not care for safety.
    /// So, if `i` >= `N`, it will panic.
    /// 
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Counterpart Method
    /// Use this method only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method
    /// [get_num()](struct@BigUInt#method.get_num) for safety.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let b = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", b);
    /// assert_eq!(b, 30);
    /// ```
    /// 
    /// # Panic Example
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// // It will panic.
    /// let c = a.get_num_(8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn get_num_(&self, _i: usize) -> T
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_num(&mut self, _i: usize, _val: T) -> bool
    /// Sets i-th element of its array of type `T`, and return `true`
    /// if `i` < `N`. Otherwise, it sets none of the elements of its
    /// array of type `T`, and returns `false`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    ///  
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianess.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it will return `false`.
    /// 
    /// # Counterpart Method
    /// When you are sure that `i` < `N`, you may want to use its Counterpart
    /// method [set_num_()](struct@BigUInt#method.set_num_) for performance.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = U256::from([0_u64, 10, 20, 30]);
    /// let mut num = a.get_num_(3);
    /// println!("a.get_num(3).unwrap() = {}", num);
    /// let b = a.set_num(3, 0);
    /// assert!(b);
    /// num = a.get_num_(3);
    /// println!("a.get_num(3).unwrap() = {}", num);
    /// assert_eq!(num, 0);
    /// 
    /// let c = a.set_num(4, 0);
    /// if !b
    ///     { println!("There is no fourth element."); }
    /// assert!(!c);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_num(&mut self, _i: usize, _val: T) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_num_(&mut self, _i: usize, _val: T)
    /// Sets i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `i` >= `N`, it will panic.
    /// 
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Counterpart Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method set_num(). This method set_num_()
    /// is considered to be slightly faster than the method set_num().
    /// Use this method set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method
    /// [set_num()](struct@BigUInt#method.set_num).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from([10_u128, 20]);
    /// let mut num = a.get_num_(1);
    /// println!("a.get_num_(1) = {}", num);
    /// assert_eq!(num, 20);
    /// 
    /// a.set_num_(1, 300);
    /// num = a.get_num_(1);
    /// println!("a.get_num_(1) = {}", num);
    /// assert_eq!(num, 300);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from([10_u128, 20]);
    /// // It will panic.
    /// let c = a.set_num_(4, 0);
    /// ```
    #[inline]
    pub fn set_num_(&mut self, _i: usize, _val: T)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_number(&self) -> &[T; N]
    /// Returns the reference of its array of `T`-type for borrowing instead
    /// of giving its ownership. `BigUInt` has an array of `T` in order
    /// to present long-sized unsigned integers.
    /// 
    /// # Features
    /// The output of this method is immutable.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// if let Ok(a) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    /// {
    ///     let arr = a.get_number();
    ///     println!("arr = {:?}", arr);
    ///     assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    /// }
    /// ```
    #[inline]
    pub fn get_number(&self) -> &[T; N]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_number_mut(&self) -> &mut [T; N]
    /// Returns the reference of its array of `T`-type for borrowing instead
    /// of giving its ownership. `BigUInt` has an array of `T` in order
    /// to present long-sized unsigned integers.
    /// 
    /// # Features
    /// The output of this method is mutable.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// if let Ok(a) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    /// {
    ///     let arr = a.get_number_mut();
    ///     println!("arr = {:?}", arr);
    ///     assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// Only for big-endian compatible.
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_number_mut(&self) -> &mut [T; N]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_number(&mut self, _val: &[T; N])
    /// Sets the contents of its array of `T`-type. The argument val is the
    /// reference of array of type `T` with the length `N`. `BigUInt` have an
    /// array of `T` in order to present long-sized unsigned integer.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// println!("arr = {:?}", a);
    /// let arr = [1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    /// a.set_number(&arr);
    /// println!("arr = {:?}", a);
    /// assert_eq!(a.get_number(), &arr);
    /// ```
    #[inline]
    pub fn set_number(&mut self, _val: &[T; N])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_zero(&mut self)
    /// Sets `BigUInt` to be zero.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_zero();
    /// println!("a = {}", a);
    /// assert_eq!(a, U256::zero());
    /// ```
    pub fn set_zero(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_zero(&self) -> bool
    /// Checks whether `BigUInt` to be zero and returns true if it is
    /// zero and returns false if it is not zero.
    /// 
    /// # Output
    /// It returns true if it is zero. Otherwise, it returns false.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U1024::zero();
    /// let mut b_zero = a.is_zero();
    /// if b_zero
    /// {
    ///     println!("a is Zero");
    ///     assert_eq!(b_zero, true);
    /// }
    /// else
    /// {
    ///     println!("a is Not Zero");
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U1024::zero();
    /// let mut b_zero = a.is_zero();
    /// a.set_one();
    /// b_zero = a.is_zero();
    /// if b_zero
    /// {
    ///     println!("a is Zero");
    /// }
    /// else
    /// {
    ///     println!("a is Not Zero");
    ///     assert_eq!(b_zero, false);
    /// }
    /// ```
    pub fn is_zero(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_one(&mut self)
    /// Sets `BigUInt` to be one.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_one();
    /// println!("a = {}", a);
    /// assert_eq!(a, U256::one());
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_one(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_one(&self) -> bool
    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one.
    /// 
    /// # Output
    /// It returns `true` if it is one. Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::one();
    /// let mut b_one = a.is_one();
    /// if b_one
    /// {
    ///     println!("a is One");
    ///     assert_eq!(b_one, true);
    /// }
    /// else
    /// {
    ///     println!("a is Not One");
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::one();
    /// let mut b_one = a.is_one();
    /// 
    /// a.set_max();
    /// b_one = a.is_one();
    /// if b_one
    /// {
    ///     println!("a is One");
    /// }
    /// else
    /// {
    ///     println!("a is Not One");
    ///     assert_eq!(b_one, false);
    /// }
    /// ```
    pub fn is_one(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_zero_or_one(&self) -> bool
    /// Checks whether `BigUInt` to be either zero or one and returns true if it
    /// is either zero or one. Otherwise, it returns false.
    /// 
    /// # Output
    /// It returns true if it is either zero or one. Otherwise, it returns false.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::zero();
    /// println!("a = {}", a);
    /// let b_zero_or_one = a.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a is One or Zero.");
    ///     assert_eq!(b_zero_or_one, true);
    /// }
    /// else
    /// {
    ///     println!("a is Neither One nor Zero.");
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::one();
    /// println!("a = {}", a);
    /// let b_zero_or_one = a.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a is One or Zero.");
    /// }
    /// else
    /// {
    ///     println!("a is Neither One nor Zero.");
    ///     assert_eq!(b_zero_or_one, true);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = U256::one();
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("a = {}", a);
    /// let b_zero_or_one = a.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a is One or Zero.");
    /// }
    /// else
    /// {
    ///     println!("a is Neither One nor Zero.");
    ///     assert_eq!(b_zero_or_one, false);
    /// }
    /// ```
    pub fn is_zero_or_one(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_max(&mut self)
    /// Sets `self` to be maximum value in which all bits are
    /// set to be `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_max();
    /// println!("a = {}", a.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    pub fn set_max(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_submax(&mut self, _size_in_bits: usize)
    /// Sets `set` to be `size_in_bits`-bit long maximum value
    /// in which all bits are set to be `1`.
    /// 
    /// # Features
    /// This method will make all the `size_in_bits` bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_submax(200_usize);
    /// println!("a = {}", a.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    pub fn set_submax(&mut self, _size_in_bits: usize)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_halfmax(&mut self)
    /// Sets `self` to be half long maximum value
    /// in which all bits are set to be `1`.
    /// 
    /// # Features
    /// This method will make all the half lower bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_halfmax();
    /// println!("a = {}", a.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    #[inline]
    pub fn set_halfmax(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_max(&self) -> bool
    /// Checks whether or not `BigUInt`-type number to be maximum value.
    /// 
    /// # Output
    /// It returns `true` if it has maxmum number.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::max();
    /// println!("Is {} a 256-bit maximum? - {}", a, a.is_max());
    /// assert_eq!(a.is_max(), true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let b = U256::max().wrapping_sub_uint(1_u8);
    /// println!("Is {} a 256-bit maximum? - {}", b, b.is_max());
    /// assert_eq!(b.is_max(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_max(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_msb(&mut self)
    /// Sets the MSB (Most Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_msb();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_msb(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_lsb(&mut self)
    /// Sets the LSB (Least Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_lsb();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_lsb(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_uint<U>(&mut self, _val: U)
    /// Sets `BigUInt`-type number with `U`-type small value such as `u8`,
    /// `u16`, `u32`, `u64`, and `u128` type value. This mathod set_uint()
    /// is useful especially when you initialize `BigUInt`-type big
    /// unsigned integer with a small value.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, this method may panic or
    /// its behavior may be undefined though it may not panic.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a = U1024::new();
    /// println!("a = {}", a);
    /// a.set_uint(340282366920938463453374607431768211455_u128);
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string(), "340282366920938463453374607431768211455");
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_uint<U>(&mut self, _val: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_uint<U>(&self, _val: U) -> bool
    /// Check whether the `BigUInt`-type number is equal to `U`-type number.
    /// It will return `true`, if it is equal to the `U`-type number. Otherwise,
    /// it will return `false`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It will return `true`, if it is equal to val.
    /// Otherwise, it will return `false`.
    /// 
    /// # Counterpart Method
    /// This method is_uint() is virtually the same the method [eq_uint()](struct@BigUInt#method.eq_uint).
    /// However, you may want to use this method is_uint() rather than [eq_uint()](struct@BigUInt#method.eq_uint),
    /// if you know that this method is_uint() is a bit faster than [eq_uint()](struct@BigUInt#method.eq_uint),
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = U1024::one() + 50_u16;
    /// println!("Question: Is a 51?\nAnswer: {}", a.is_uint(51_u32));
    /// assert_eq!(a.is_uint(51_u16), true);
    /// assert_eq!(a.is_uint(50_u16), false);
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_uint<U>(&self, _val: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_odd(&self) -> bool
    /// Checks whether the `BigUInt`-type number is an odd number.
    /// 
    /// # Output
    /// It will return `true`, if it is odd. Otherwise, it will return `false`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = U256::new();
    /// a.set_uint(340282366920938463453374697431768211455_u128);
    /// if a.is_odd()
    ///     { println!("{} is odd", a); }
    /// else
    ///     { println!("{} is even", a); }
    /// assert_eq!(a.is_odd(), true);
    /// 
    /// a <<= 1;
    /// if a.is_odd()
    ///     { println!("{} is odd", a); }
    /// else
    ///     { println!("{} is even", a); }
    /// assert_eq!(a.is_odd(), false);
    /// ```
    #[inline]
    pub fn is_odd(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_even(&self) -> bool
    /// Checks whether the `BigUInt`-type number is an even number.
    /// 
    /// # Output
    /// It will return `true`, if it is even. Otherwise, it will return `false`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::new();
    /// a.set_uint(169743176821145534028236692093846345739_u128);
    /// if a.is_even()
    ///     { println!("{} is even", a); }
    /// else
    ///     { println!("{} is odd", a); }
    /// assert_eq!(a.is_even(), false);
    /// 
    /// a <<= 1;
    /// if a.is_even()
    ///     { println!("{} is even", a); }
    /// else
    ///     { println!("{} is odd", a); }
    /// assert_eq!(a.is_even(), true);
    /// ```
    #[inline]
    pub fn is_even(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_msb_set(&self) -> bool
    /// Checks whether the MSB (Most Segnificant Bit) of `self` is set to be
    /// one.
    /// 
    /// # Output
    /// It will return `true`, if the MSB of `self` is set to be one.
    /// Otherwise, it will return `false`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::new();
    /// a.set_uint(169743176821145534028236692093846345739_u128);
    /// if a.is_msb_set()
    ///     { println!("{} is greater than halfmax ({}).", a, U256::halfmax()); }
    /// else
    ///     { println!("{} is less than or equal to halfmax ({}).", a, U256::halfmax()); }
    /// assert_eq!(a.is_msb_set(), false);
    /// 
    /// a.set_msb();
    /// if a.is_msb_set()
    ///     { println!("{} is greater than halfmax ({}).", a, U256::halfmax()); }
    /// else
    ///     { println!("{} is less than or equal to halfmax ({}).", a, U256::halfmax()); }
    /// assert_eq!(a.is_msb_set(), true);
    /// ```
    #[inline]
    pub fn is_msb_set(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** METHODS TO CHECK BITS *****/

    // pub fn count_ones(&self) -> u32
    /// Returns the number of ones in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the bits that are set to be one.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// println!("{} is {} in binary and has {} ones in binary.", a, a.to_string_with_radix_and_stride(2, 10).unwrap(), a.count_ones());
    /// assert_eq!(a.count_ones(), 107);
    /// ```
    pub fn count_ones(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn count_zeros(&self) -> u32
    /// Returns the number of zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the bits that are set to be zero.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    /// println!("{} is {} in binary and has {} zeros in binary.", a, a.to_string_with_radix_and_stride(2, 10).unwrap(), a.count_zeros());
    /// assert_eq!(a.count_zeros(), 149);
    /// ```
    pub fn count_zeros(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn leading_ones(&self) -> u32
    /// Returns the number of leading ones in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading bits that are set to be one.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// println!("{} is {} in binary and has {} leading ones in binary.", a, a.to_string_with_radix_and_stride(2, 10).unwrap(), a.leading_ones());
    /// assert_eq!(a.leading_ones(), 2);
    /// ```
    pub fn leading_ones(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn leading_zeros(&self) -> u32
    /// Returns the number of leading zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading bits that are set to be zero.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    /// println!("{} is {} in binary and has {} leading zeros in binary.", a, a.to_string_with_radix_and_stride(2, 10).unwrap(), a.leading_zeros());
    /// assert_eq!(a.leading_zeros(), 0);
    /// ```
    pub fn leading_zeros(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn trailing_ones(&self) -> u32
    /// Returns the number of trailing ones in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be one.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use std::str::FromStr;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str("111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();
    /// println!("{} is {} in binary and has {} leading ones in binary.", a, a.to_string_with_radix_and_stride(2, 10).unwrap(), a.leading_ones());
    /// assert_eq!(a.trailing_ones(), 3);
    /// ```
    pub fn trailing_ones(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn trailing_zeros(&self) -> u32
    /// Returns the number of trailing zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be zero.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = "111111111111111111111111111111111111111111111111111111111111111111111111111111".parse::<U256>().unwrap();
    /// println!("{} is {} in binary and has {} leading zeros in binary.", a, a.to_string_with_radix_and_stride(2, 10).unwrap(), a.leading_zeros());
    /// assert_eq!(a.trailing_zeros(), 0);
    /// ```
    pub fn trailing_zeros(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn leading_max_elements(&self) -> u32
    /// Returns the number of leading maximum elements in the binary
    /// representation of the array `number[T;N]` of `self`.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Output
    /// It returns the total number of the leading maximum elements
    /// that has all bits set to be one.
    /// Here, 'maximum element' means the element that has all bits to be one. 
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999_88888888", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} leading max elements in array.", a, a.to_string_with_radix_and_stride(16, 2).unwrap(), a.leading_max_elements());
    /// assert_eq!(a.leading_max_elements(), 4);
    /// ```
    pub fn leading_max_elements(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn leading_zero_elements(&self) -> u32
    /// Returns the number of leading zero elements in the binary
    /// representation of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading zero elemments
    /// that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_str_radix("00000000_FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} leading zero elements in array.", a, a.to_string_with_radix_and_stride(16, 8).unwrap(), a.leading_zero_elements());
    /// assert_eq!(a.leading_zero_elements(), 1);
    /// ```
    pub fn leading_zero_elements(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn trailing_max_elements(&self) -> u32
    /// Returns the number of trailing maximum elements in the binary
    /// representation of the array `number[T;N]` of `self`.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Output
    /// It returns the total number of the trailing maximum elemeents
    /// that have all bits set to be one.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = U256::from_str_radix("88888888_99999999_AAAAAAAA_BBBBBBBB_CCCCCCCC_DDDDDDDD_EEEEEEEE_FFFFFFFF", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} trailing max elements in array.", a, a.to_string_with_radix_and_stride(16, 4).unwrap(),a.trailing_max_elements());
    /// assert_eq!(a.trailing_max_elements(), 2);
    /// ```
    pub fn trailing_max_elements(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn trailing_zero_elements(&self) -> u32
    /// Returns the number of trailing zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_9999999_900000000", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} trailing zero elements in array.", a, a.to_string_with_radix_and_stride(16, 2).unwrap(),a.trailing_zero_elements());
    /// assert_eq!(a.trailing_zero_elements(), 4);
    /// ```
    pub fn trailing_zero_elements(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** METHODS FOR COMPARISON WITH UINT *****/

    // pub fn partial_cmp_uint<U>(&self, _other: U) -> Option<Ordering>
    /// Compares `self` and a value of type `U` and returns the
    /// result of the comparison in the type `Option<Ordering>`.
    /// However, if the datatype `U` is the same datatype `T`, it will be
    /// more convenient for you to use the operators `<`, `>`, `<=`, `>=`,
    /// `==`, and `!=`. Then, you don't have to use `partial_cmp_uint()`
    /// directly. But, if the datatype `U` is not the same datatype `T`, you
    /// can use the methods `lt_uint()`, `gt_uint()`, `le_uint()`,
    /// `ge_uint()`, and `eq_uint()` for your convenience. Then, you don't
    /// have to use `partial_cmp_uint()` directly too.
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
    /// It returns `Ordering::Greater` wrapped by `Some` of enum `Option`
    /// if `self` is greater than `other`.
    /// It returns `Ordering::Less` wrapped by `Some` of enum `Option`
    /// if `self` is less than `other`.
    /// It returns `Ordering::Equal` wrapped by `Some` of enum `Option`
    /// if `self` is equal to `other`.
    /// 
    /// # Example 1
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(90_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 90"); }
    ///     Ordering::Less => { println!("100 < 90"); }
    ///     Ordering::Equal => { println!("100 = 90"); }
    /// }
    /// assert_eq!(res, Ordering::Greater);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(110_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 110"); }
    ///     Ordering::Less => { println!("100 < 110"); }
    ///     Ordering::Equal => { println!("100 = 110"); }
    /// }
    /// assert_eq!(res, Ordering::Less);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(100_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 100"); }
    ///     Ordering::Less => { println!("100 < 100"); }
    ///     Ordering::Equal => { println!("100 = 100"); }
    /// }
    /// assert_eq!(res, Ordering::Equal);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn partial_cmp_uint<U>(&self, _other: U) -> Option<Ordering>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn lt_uint<U>(&self, _other: U) -> bool
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
    /// let res = a_biguint.lt_uint(b_uint);
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
    /// let res = a_biguint.lt_uint(b_uint);
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
    /// let res = a_biguint.lt_uint(b_uint);
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn lt_uint<U>(&self, _other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn gt_uint<U>(&self, _other: U) -> bool
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
    /// let res = a_biguint.gt_uint(b_uint);
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
    /// let res = a_biguint.gt_uint(b_uint);
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
    /// let res = a_biguint.gt_uint(b_uint);
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn gt_uint<U>(&self, _other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn le_uint<U>(&self, _other: U) -> bool
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
    /// let res = a_biguint.le_uint(b_uint);
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
    /// let res = a_biguint.le_uint(b_uint);
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
    /// let res = a_biguint.le_uint(b_uint);
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn le_uint<U>(&self, _other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ge_uint<U>(&self, _other: U) -> bool 
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
    /// let res = a_biguint.ge_uint(b_uint);
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
    /// let res = a_biguint.ge_uint(b_uint);
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
    /// let res = a_biguint.ge_uint(b_uint);
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ge_uint<U>(&self, _other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn eq_uint<U>(&self, _other: U) -> bool
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
    /// let res = a_biguint.eq_uint(b_uint);
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
    /// let res = a_biguint.eq_uint(b_uint);
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn eq_uint<U>(&self, _other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** METHODS FOR COMPARISON WITH BIGUINT *****/

    // pub fn partial_cmp(&self, _other: &Self) -> Option<Ordering>
    /// Compares `self` and a value of `other` and returns the result of the
    /// comparison in the type `Option<Ordering>`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
    /// 
    /// # Output
    /// It returns `Ordering::Greater` wrapped by `Some` of enum `Option`
    /// if `self` is greater than `other`.
    /// It returns `Ordering::Less` wrapped by `Some` of enum `Option`
    /// if `self` is less than `other`.
    /// It returns `Ordering::Equal` wrapped by `Some` of enum `Option`
    /// if `self` is equal to `other`.
    /// 
    /// # Example 1
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num_str2 = "60000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// let num2 = num_str2.parse::<UU32>().unwrap();
    /// 
    /// let res = num1.partial_cmp(&num2).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{} > {}", num1, num2); }
    ///     Ordering::Less => { println!("{} < {}", num1, num2); }
    ///     Ordering::Equal => { println!("{} = {}", num1, num2); }
    /// }
    /// assert_eq!(res, Ordering::Greater);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num_str3 = "80000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// let num3 = num_str3.parse::<UU32>().unwrap();
    /// let res = num1.partial_cmp(&num3).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{} > {}", num1, num3); }
    ///     Ordering::Less => { println!("{} < {}", num1, num3); }
    ///     Ordering::Equal => { println!("{} = {}", num1, num3); }
    /// }
    /// assert_eq!(res, Ordering::Less);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// let res = num1.partial_cmp(&num1).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{0} > {0}", num1); }
    ///     Ordering::Less => { println!("{0} < {0}", num1); }
    ///     Ordering::Equal => { println!("{0} = {0}", num1); }
    /// }
    /// assert_eq!(res, Ordering::Equal);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "little")]
    pub fn partial_cmp(&self, _other: &Self) -> Option<Ordering>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn lt(&self, _other: &Self) -> bool
    /// Compares `self` and `other` to find whether `self` is less than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
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
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.lt(&b_biguint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.lt(&b_biguint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.lt(&b_biguint);
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn lt(&self, _other: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn gt(&self, _other: &Self) -> bool
    /// Compares `self` and `other` to find whether `self` is greater
    /// than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
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
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.gt(&b_biguint);
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
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.gt(&b_biguint);
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
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.gt(&b_biguint);
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn gt(&self, _other: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn le(&self, _other: &Self) -> bool
    /// Compares `self` and `other` to find whether `self` is less than or
    /// equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
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
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.le(&b_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.le(&b_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.le(&b_biguint);
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn le(&self, _other: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ge(&self, _other: &Self) -> bool
    /// Compares `self` and `other` to find whether `self` is greater than
    /// or equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
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
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.ge(&b_biguint);
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
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.ge(&b_biguint);
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
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.ge(&b_biguint);
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ge(&self, _other: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn eq(&self, _other: &Self) -> bool
    /// Compare `self` with `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.eq(&b_biguint);
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
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.eq(&b_biguint);
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn eq(&self, _other: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    /*** METHODS FOR PRIME NUMBER TEST ***/

    // pub fn is_prime_using_miller_rabin(&self, _repetition: usize) -> bool
    /// Tests a `BigUInt`-type object to find whether or not it is a
    /// primne number.
    /// 
    /// # Arguments
    /// The argument `repetition` defines how many times it tests whether or
    /// not `self` is a prime number. Usually, `5` is given to repetition`
    /// in order to achieve 99.9% accuracy.
    /// 
    /// # Output
    /// It returns `true` if it is a primne number.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - It uses the method `test_miller_rabin()` which uses
    ///   [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number,
    ///   the probability that the tested number is not a prime number is
    ///   (1/4) ^ `repeatition`.
    ///   So, if `repeatition` is two and it results in prime number the
    ///   probability that the tested number is not a prime number is
    ///   1/16 (= 1/4 * 1/4). Therefore, if you test any number with
    ///   `repeatition` (= 5) and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// - However, for performance, if the number is less than 10000,
    ///   it does not use Miller-Rabin alogrithm but deterministic algorithm
    ///   so that the argument `repetition` is meaningless.
    /// - If the number is less than u32::MAX (= 4294967295_u32),
    ///   3 is enough for `repetition` for 100% certainty for determination of
    ///   prime number. This method tests the number with 2, 7, and 61.
    /// - If the number is less than u64::MAX (= 18446744073709551615_u64),
    ///   7 is enough for `repetition` for 100% certainty for determination of
    ///   prime number. This method tests the number with 2, 325, 9375, 28178,
    ///   450775, 9780504, and 1795265022.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1 for prime numer case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    /// println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    /// assert_eq!(b, true);
    /// ```
    /// 
    /// # Example 2 for composite number case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    /// println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    /// assert_eq!(b, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_prime_using_miller_rabin(&self, _repetition: usize) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** METHODS FOR BIT OPERATION *****/

    // pub fn shift_left<U>(&self, _n: U) -> Self
    /// Shift left the field `number: [T;N]` to the left by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`, which is shifted to the
    /// left by `n` bits.
    /// 
    /// # Left Carry
    /// 'A left-carry occurs' means that a bit `1` is pushed out
    /// during shift-left operation.
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
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.shift_left(n);
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.shift_left(n);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.shift_left(n);
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
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.shift_left(n);
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
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.shift_left(n);
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
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn shift_left<U>(&self, _n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn shift_left_assign<U>(&mut self, _n: U)
    /// shifts the field `number: [T;N]` to the left by `n`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Left Carry
    /// 'A left-carry occurs' means that a bit `1` is pushed out
    /// during shift-left operation.
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
    /// a_biguint.shift_left_assign(n);
    /// println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 2
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
    /// let n = 4_u8;
    /// a_biguint.shift_left_assign(n);
    /// println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 3
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
    /// let n = 128_u8;
    /// a_biguint.shift_left_assign(n);
    /// println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 4
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
    /// let n = 256_u16;
    /// a_biguint.shift_left_assign(n);
    /// println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 5
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
    /// let n = 512_u16;
    /// a_biguint.shift_left_assign(n);
    /// println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    pub fn shift_left_assign<U>(&mut self, _n: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_shift_left<U>(&self, _n: U) -> Option<Self>
    /// Shift left the field `number: [T;N]` to the left by `n`,
    /// and returns the result, wrapped by `some` of enum `Option`.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// - If n is less than the size of the type `T` * N * 8,
    ///   it returns the left-shifted version of `self`, which is shifted to the
    ///   left by `n` bits, wrapped by `some` of enum `Option`.
    /// - If `n` is greater than or equal to the size of the type `T` * N * 8,
    ///   all bits will be gone. So, it returns `None`.
    /// 
    /// # Left Carry
    /// 'A left-carry occurs' means that a bit `1` is pushed out
    /// during shift-left operation.
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
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), true);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), true);
    ///             assert_eq!(r.is_right_carry(), false);
    ///        },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_shift_left<U>(&self, _n: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_shift_left<U>(&self, _n: U) -> Self
    /// Shift left the field `number: [T;N]` to the left by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`, which is shifted to the
    /// left by `n` bits.
    /// 
    /// # Left Carry
    /// 'A left-carry occurs' means that a bit `1` is pushed out
    /// during shift-left operation.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `n` is greater than or equal to the size of the type `T` * N * 8,
    ///   all bits will be gone. So, it will panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.unchecked_shift_left(n);
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.unchecked_shift_left(n);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.unchecked_shift_left(n);
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
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 256_u16;
    /// // It will panic!
    /// let _res = _a_biguint.unchecked_shift_left(_n);
    /// 
    /// let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 512_u16;
    /// // It will panic!
    /// let _res = _a_biguint.unchecked_shift_left(_n);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_shift_left<U>(&self, _n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn shift_right<U>(&self, _n: U) -> Self
    /// Shift right the field `number: [T;N]` to the right by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits.
    /// 
    /// # Right Carry
    /// 'A right-carry occurs' means that a bit `1` is pushed out
    /// during shift-right operation.
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
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.shift_right(n);
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.shift_right(n);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.shift_right(n);
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
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.shift_right(n);
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
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.shift_right(n);
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
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn shift_right<U>(&self, _n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn shift_right_assign<U>(&mut self, _n: U)
    /// shifts the field `number: [T;N]` to the right by `n`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Right Carry
    /// 'A right-carry occurs' means that a bit `1` is pushed out
    /// during shift-right operation.
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
    /// a_biguint.shift_right_assign(n);
    /// println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let n = 4_u8;
    /// a_biguint.shift_right_assign(n);
    /// println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let n = 128_u8;
    /// a_biguint.shift_right_assign(n);
    /// println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let n = 256_u16;
    /// a_biguint.shift_right_assign(n);
    /// println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let n = 512_u16;
    /// a_biguint.shift_right_assign(n);
    /// println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    pub fn shift_right_assign<U>(&mut self, _n: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_shift_right<U>(&self, _n: U) -> Option<Self>
    /// Shift right the field `number: [T;N]` to the right by `n`,
    /// and returns the result, wrapped by `some` of enum `Option`.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits, wrapped by `some` of enum `Option`.
    /// If n is greater than or equal to the size of the type `T` * N * 8,
    /// all bits will be gone. So, it returns `None`.
    /// 
    /// # Right Carry
    /// 'A right-carry occurs' means that a bit `1` is pushed out
    /// during shift-right operation.
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
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), true);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), true);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_shift_right<U>(&self, _n: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_shift_right<U>(&self, _n: U) -> Self
    /// shifts the field `number: [T;N]` to the right by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits.
    /// 
    /// # Right Carry
    /// 'A right-carry occurs' means that a bit `1` is pushed out
    /// during shift-right operation.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If n is greater than or equal to the size of the type `T` * N * 8,
    ///   all bits will be gone. So, it will panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.unchecked_shift_right(n);
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.unchecked_shift_right(n);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.unchecked_shift_right(n);
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
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 256_u16;
    /// // It will panic!
    /// let res = _a_biguint.unchecked_shift_right(_n);
    /// 
    /// let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 512_u16;
    /// // It will panic!
    /// let res = _a_biguint.unchecked_shift_right(_n);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_shift_right<U>(&self, _n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn rotate_left<U>(&self, _n: U) -> Self
    /// Rotates the field `number: [T;N]` to the left by `n`,
    /// and returns the result.
    /// 
    /// # Features
    /// - 'Rotate left' means 'shift left' with filling the left-pushed-out bits
    ///   to the empty rightmost bits. So, if `10011010` is rotated left by 2,
    ///   it will be `01101010`, for example.
    /// - This method does not set `LEFT_CARRY`.
    /// 
    /// # Output
    /// It returns the left-rotated version of `self`. which is rotated to the
    /// left by `n` bits.
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
    /// let n = 3_u8;
    /// let res = a_biguint.rotate_left(n);
    /// println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010110_01100111_10000111_11111000_00000111_11111111_11111000_00000000_00000111_11111111_11111111_11111000_00000000_00000000_00000111_11111111_11111111_11111111_11111000_00000000_00000000_00000000_00000101_10011100_01111000_01111100_00011111_10000001_11111100_00000111_11111000_00000101");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.rotate_left(n);
    /// println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000_00001010");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.rotate_left(n);
    /// println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.rotate_left(n);
    /// println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.rotate_left(n);
    /// println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn rotate_left<U>(&self, _n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn rotate_left_assign<U>(&mut self, _n: U)
    /// Rotates the field `number: [T;N]` to the left by `n`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// - 'Rotate left' means 'shift left' with filling the left-pushed-out bits
    ///   to the empty rightmost bits. So, if `10011010` is rotated left by 2,
    ///   it will be `01101010`, for example.
    /// - This method does not set `LEFT_CARRY`.
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
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 3_u8;
    /// a_biguint.rotate_left_assign(n);
    /// println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010110_01100111_10000111_11111000_00000111_11111111_11111000_00000000_00000111_11111111_11111111_11111000_00000000_00000000_00000111_11111111_11111111_11111111_11111000_00000000_00000000_00000000_00000101_10011100_01111000_01111100_00011111_10000001_11111100_00000111_11111000_00000101");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
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
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 4_u8;
    /// a_biguint.rotate_left_assign(n);
    /// println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000_00001010");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
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
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 128_u8;
    /// a_biguint.rotate_left_assign(n);
    /// println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
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
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 256_u16;
    /// a_biguint.rotate_left_assign(n);
    /// println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
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
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 512_u16;
    /// a_biguint.rotate_left_assign(n);
    /// println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn rotate_left_assign<U>(&mut self, _n: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn rotate_right<U>(&self, _n: U) -> Self
    /// Rotates the field `number: [T;N]` to the right by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// - 'Rotate right' means 'shift right' with filling the right-pushed-out
    ///   bits to the empty leftmost bits. So, if `10011010` is rotated right
    ///   by 2, it will be `10100110`, for example.
    /// - This method does not set `RIGHT_CARRY`.
    /// 
    /// # Output
    /// It returns the right-rotated version of `self`. which is rotated to the
    /// right by `n` bits.
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
    /// let n = 3_u8;
    /// let res = a_biguint.rotate_right(n);
    /// println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101_01011001_10011110_00011111_11100000_00011111_11111111_11100000_00000000_00011111_11111111_11111111_11100000_00000000_00000000_00011111_11111111_11111111_11111111_11100000_00000000_00000000_00000000_00010110_01110001_11100001_11110000_01111110_00000111_11110000_00011111_11100000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.rotate_right(n);
    /// println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010_10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.rotate_right(n);
    /// println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.rotate_right(n);
    /// println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.rotate_right(n);
    /// println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn rotate_right<U>(&self, _n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn rotate_right_assign<U>(&mut self, _n: U)
    /// Rotates the field `number: [T;N]` to the right by `n`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// - 'Rotate right' means 'shift right' with filling the right-pushed-out
    ///   bits to the empty leftmost bits. So, if `10011010` is rotated right
    ///   by 2, it will be `10100110`, for example.
    /// - This method does not set `RIGHT_CARRY`.
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
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
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
    /// a_biguint.rotate_right_assign(n);
    /// println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101_01011001_10011110_00011111_11100000_00011111_11111111_11100000_00000000_00011111_11111111_11111111_11100000_00000000_00000000_00011111_11111111_11111111_11111111_11100000_00000000_00000000_00000000_00010110_01110001_11100001_11110000_01111110_00000111_11110000_00011111_11100000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 4_u8;
    /// a_biguint.rotate_right_assign(n);
    /// println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010_10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 128_u8;
    /// a_biguint.rotate_right_assign(n);
    /// println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 256_u16;
    /// a_biguint.rotate_right_assign(n);
    /// println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 512_u16;
    /// a_biguint.rotate_right_assign(n);
    /// println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn rotate_right_assign<U>(&mut self, _n: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn and(&self, _rhs: &Self) -> Self
    /// Performs the bitwise AND (&) operation, and then returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the reference of another object that AND (&) operation is
    ///   performed with.
    /// - `rhs` is of `&Self` type.
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.and(&b_biguint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.and(&b_biguint);
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.and(&b_biguint);
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn and(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn and_assign(&mut self, _rhs: &Self)
    /// Performs the bitwise AND (&) operation,
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.and_assign(&b_biguint);
    /// println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.and_assign(&b_biguint);
    /// println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.and_assign(&b_biguint);
    /// println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn and_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn or(self, _rhs: &Self) -> Self
    /// Performs the bitwise OR (|) operation, and then returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the reference of another object that OR (|) operation is
    ///   performed with.
    /// - `rhs` is of `&Self` type.
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.or(&b_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.or(&b_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.or(&b_biguint);
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn or(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn or_assign(&mut self, _rhs: &Self)
    /// Performs the bitwise OR (|) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the OR (|) operation
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
    /// a_biguint.or_assign(&b_biguint);
    /// println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// a_biguint.or_assign(&b_biguint);
    /// println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// a_biguint.or_assign(&b_biguint);
    /// println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn or_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn xor(self, _rhs: &Self) -> Self
    /// Performs the bitwise XOR (^) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `&Self` type.
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.xor(&b_biguint);
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.xor(&b_biguint);
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.xor(&b_biguint);
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn xor(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn xor_assign(&mut self, _rhs: &Self)
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
    /// a_biguint.xor_assign(&b_biguint);
    /// println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// a_biguint.xor_assign(&b_biguint);
    /// println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// a_biguint.xor_assign(&b_biguint);
    /// println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn xor_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn flip(&self) -> Self
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.flip();
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.flip();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// let a_biguint = U256::max();
    /// let res = a_biguint.flip();
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.flip();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    pub fn flip(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn flip_assign(&mut self)
    /// Performs the bitwise NOT (!) operation,
    /// and then assigns the result to `self` back.
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
    /// a_biguint.flip_assign();
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
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
    /// let mut a_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.flip_assign();
    /// assert_eq!(a_biguint.to_string(), "0");
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
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.flip_assign();
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn flip_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reverse_bits(&self) -> Self
    /// Reverses the order of bits of the field `number` [T; N] of `self`,
    /// and then returns the result.
    /// 
    /// # Output
    /// It returns the reversed order of bits in the field `number` [T; N]
    /// of `self`.
    /// 
    /// # Features
    /// The least significant bit becomes the most significant bit,
    /// second least-significant bit becomes second most-significant bit, etc.
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
    /// let res = a_biguint.reverse_bits();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000001_11111100_00001111_11000001_11110000_11110001_11001101_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_00001111_00110011_01010101");
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
    /// let res = a_biguint.reverse_bits();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
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
    /// let res = a_biguint.reverse_bits();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn reverse_bits(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reverse_bits_assign(&mut self)
    /// Reverses the order of bits of the field `number` [T; N] of `self`,
    /// and assigns the result to `self` back.
    /// 
    /// # Features
    /// The least significant bit becomes the most significant bit,
    /// second least-significant bit becomes second most-significant bit, etc.
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
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.reverse_bits_assign();
    /// println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000001_11111100_00001111_11000001_11110000_11110001_11001101_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_00001111_00110011_01010101");
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.reverse_bits_assign();
    /// println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.reverse_bits_assign();
    /// println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn reverse_bits_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn swap_bytes(&self) -> Self
    /// Reverses the byte order of the field `number` [T; N] of `self`,
    /// and then returns the result.
    /// 
    /// # Output
    /// It returns the reversed byte order of the field `number` [T; N]
    /// of `self`.
    /// 
    /// # Features
    /// The least significant byte becomes the most significant byte,
    /// second least-significant byte becomes second most-significant byte, etc.
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
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.swap_bytes();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.swap_bytes();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.swap_bytes();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn swap_bytes(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn swap_bytes_assign(&mut self)
    /// Reverses the byte order of the field `number` [T; N] of `self`,
    /// and assigns the result to `self` back.
    /// 
    /// # Features
    /// The least significant byte becomes the most significant byte,
    /// second least-significant byte becomes second most-significant byte, etc.
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
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.swap_bytes_assign();
    /// println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.swap_bytes_assign();
    /// println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.swap_bytes_assign();
    /// println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn swap_bytes_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** METHODS FOR CONVERTING INTO OTHER TYPES WITH/WITHOUT LOSS *****/

    // pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    /// Converts `self` into another kind of `BigUInt<U, M>`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// It copies the contents of its field `number[T;N]` to the field
    /// `number: [U;M]` of `BigUInt<U, M>`. If the size of `number: [T;N]`
    /// of `self` is bigger than the size of `number: [U;M]` of `BigUInt<U, M>`,
    /// that is, `size_of::<T>() * N` > `size_of::<U>() * M`, it is lossy
    /// conversion. Otherwise, no contents of the field `number: [T;N]` of
    /// `self` is lost. Always, the field `flag` is not copied.
    /// 
    /// # Output
    /// It returns another kind of `BigUInt<U, M>` with keeping the contents
    /// of the field `number: [T;N]` as much as possible.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::number::U256_with_u128;
    /// use std::fmt::Write;
    /// 
    /// let mut a_biguint = U256_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// a_biguint.set_overflow();
    /// a_biguint.set_underflow();
    /// a_biguint.set_undefined();
    /// a_biguint.set_infinity();
    /// a_biguint.set_divided_by_zero();
    /// 
    /// let b__biguint: BigUInt<u16, 32> = a_biguint.into_biguint();
    /// println!("a_biguint = {0} = {0:?}", a_biguint);
    /// println!("b_biguint = {0} = {0:?}", b_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(b_biguint.is_overflow(), false);
    /// assert_eq!(b_biguint.is_underflow(), false);
    /// assert_eq!(b_biguint.is_infinity(), false);
    /// assert_eq!(b_biguint.is_undefined(), false);
    /// assert_eq!(b_biguint.is_divided_by_zero(), false);
    /// 
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", a_biguint)
    /// {
    ///     Ok(_) =>    { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 31 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// let mut b_txt = String::new();
    /// match write!(&mut b_txt, "{:?}", b_biguint)
    /// {
    ///     Ok(_) => { assert_eq!(b_txt, "BigUInt { number: [65280, 16256, 33776, 36623, 179, 0, 65280, 65535, 255, 0, 65535, 255, 65280, 255, 61695, 43724, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], flag: 0 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::U256_with_u128;
    /// use cryptocol::number::U512_with_u8;
    /// use std::fmt::Write;
    /// 
    /// let mut a_biguint = U256_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// a_biguint.set_overflow();
    /// a_biguint.set_underflow();
    /// a_biguint.set_undefined();
    /// a_biguint.set_infinity();
    /// a_biguint.set_divided_by_zero();
    /// 
    /// let b__biguint: U512_with_u8 = a_biguint.into_biguint();
    /// println!("a_biguint = {0} = {0:?}", a_biguint);
    /// println!("b_biguint = {0} = {0:?}", b_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(b_biguint.is_overflow(), false);
    /// assert_eq!(b_biguint.is_underflow(), false);
    /// assert_eq!(b_biguint.is_infinity(), false);
    /// assert_eq!(b_biguint.is_undefined(), false);
    /// assert_eq!(b_biguint.is_divided_by_zero(), false);
    /// 
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", a_biguint)
    /// {
    ///     Ok(_) =>    { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 31 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// let mut b_txt = String::new();
    /// match write!(&mut b_txt, "{:?}", b_biguint)
    /// {
    ///     Ok(_) =>    { assert_eq!(b_txt, "BigUInt { number: [0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], flag: 0 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::number::U256_with_u8;
    /// use cryptocol::number::U512_with_u128;
    /// use std::fmt::Write;
    /// 
    /// let mut a_biguint = U512_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// a_biguint.set_overflow();
    /// a_biguint.set_underflow();
    /// a_biguint.set_undefined();
    /// a_biguint.set_infinity();
    /// a_biguint.set_divided_by_zero();
    /// 
    /// let b__biguint: U256_with_u8 = a_biguint.into_biguint();
    /// println!("a_biguint = {0} = {0:?}", a_biguint);
    /// println!("b_biguint = {0} = {0:?}", b_biguint);
    /// assert_eq!(a_biguint.to_string(), "8945550780017187584626056870222733452660064686360582980627279346698888314793843532145493214749705164311564838731068213948692682076110455767663905463140096");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(b_biguint.is_overflow(), false);
    /// assert_eq!(b_biguint.is_underflow(), false);
    /// assert_eq!(b_biguint.is_infinity(), false);
    /// assert_eq!(b_biguint.is_undefined(), false);
    /// assert_eq!(b_biguint.is_divided_by_zero(), false);
    /// 
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", a_biguint)
    /// {
    ///     Ok(_) => { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279, 340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 31 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// let mut b_txt = String::new();
    /// match write!(&mut b_txt, "{:?}", b_biguint)
    /// {
    ///     Ok(_) => { assert_eq!(b_txt, "BigUInt { number: [0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170], flag: 0 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_uint<U>(&self) -> U
    /// Converts `self` into `U`-type small value
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128` type value,
    /// and returns the `U`-type small unsigned integer.
    /// This mathod into_uint() is useful especially when `self` has `U`-type
    /// small unsigned integer sized value and you want to cast `self` into
    /// `U`-type small unsigned integer with a small value.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the `U`-type-casted small unsigned integer.
    /// 
    /// # Features
    /// If the size of the value that `self` has is bigger than
    /// the size of `U`, the higher-bit portion will be truncated.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(16384545419507531775_u64);
    /// let b_u128: u128 = a_biguint.into_uint();
    /// let b_u64: u64 = a_biguint.into_uint();
    /// let b_u32: u32 = a_biguint.into_uint();
    /// let b_u16: u16 = a_biguint.into_uint();
    /// let b_u8: u8 = a_biguint.into_uint();
    /// println!("u128 of {} = {}", a_biguint, b_u128);
    /// println!("u64 of {} = {}", a_biguint, b_u64);
    /// println!("u32 of {} = {}", a_biguint, b_u32);
    /// println!("u16 of {} = {}", a_biguint, b_u16);
    /// println!("u8 of {} = {}", a_biguint, b_u8);
    /// assert_eq!(b_u128, 16384545419507531775_u128);
    /// assert_eq!(b_u64, 16384545419507531775_u64);
    /// assert_eq!(b_u32, 4294967295_u32);
    /// assert_eq!(b_u16, 65535_u16);
    /// assert_eq!(b_u8, 255_u8);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_u128: u128 = a_biguint.into_uint();
    /// let b_u64: u64 = a_biguint.into_uint();
    /// let b_u32: u32 = a_biguint.into_uint();
    /// let b_u16: u16 = a_biguint.into_uint();
    /// let b_u8: u8 = a_biguint.into_uint();
    /// println!("u128 of {} = {}", a_biguint, b_u128);
    /// println!("u64 of {} = {}", a_biguint, b_u64);
    /// println!("u32 of {} = {}", a_biguint, b_u32);
    /// println!("u16 of {} = {}", a_biguint, b_u16);
    /// println!("u8 of {} = {}", a_biguint, b_u8);
    /// assert_eq!(b_u128, 340282346638528863123979975818481827584_u128);
    /// assert_eq!(b_u64, 10308603139955162880_u64);
    /// assert_eq!(b_u32, 1065418496_u32);
    /// assert_eq!(b_u16, 65280_u16);
    /// assert_eq!(b_u8, 0_u8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn into_uint<U>(&self) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_u128(&self) -> u128
    /// Converts `self` into `u128`,
    /// and returns the `u128`-type small unsigned integer.
    /// This mathod into_u128() is useful especially when `self` has `u128`
    /// type unsigned integer sized value and you want to cast `self` into
    /// `u128` type unsigned integer.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest sixteen bytes of `self` as `u128`.
    /// 
    /// # Features
    /// It takes the lowest `u128`-sized bytes, that is, the lowest sixteen
    /// bytes from `self`, and return then as `u128` data type.
    /// If the size of the value that `self` has is bigger than
    /// the size of `u128`, the higher-bit portion will be truncated.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(16384545419507531775_u64);
    /// let b_u128 = a_biguint.into_u128();
    /// println!("u128 of {} = {}", a_biguint, b_u128);
    /// assert_eq!(b_u128, 16384545419507531775_u128);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_u128: u128 = a_biguint.into_u128();
    /// println!("u128 of {} = {}", a_biguint, b_u128);
    /// assert_eq!(b_u128, 340282346638528863123979975818481827584_u128);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn into_u128(&self) -> u128
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_u64(&self) -> u64
    /// Converts `self` into `u64`,
    /// and returns the `u64`-type small unsigned integer.
    /// This mathod into_u64() is useful especially when `self` has `u64`
    /// type unsigned integer sized value and you want to cast `self` into
    /// `u64` type unsigned integer.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest eight bytes of `self` as `u64`.
    /// 
    /// # Features
    /// It takes the lowest `u64`-sized bytes, that is, the lowest eight
    /// bytes from `self`, and return then as `u64` data type.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(16384545419507531775_u64);
    /// let b_u64: u64 = a_biguint.into_u64();
    /// println!("u64 of {} = {}", a_biguint, b_u64);
    /// assert_eq!(b_u64, 16384545419507531775_u64);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_u64: u64 = a_biguint.into_u64();
    /// println!("u64 of {} = {}", a_biguint, b_u64);
    /// assert_eq!(b_u64, 10308603139955162880_u64);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn into_u64(&self) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_u32(&self) -> u32
    /// Converts `self` into `u32`,
    /// and returns the `u32`-type small unsigned integer.
    /// This mathod into_u32() is useful especially when `self` has `u32`
    /// type unsigned integer sized value and you want to cast `self` into
    /// `u32` type unsigned integer.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest four bytes of `self` as `u32`.
    /// 
    /// # Features
    /// It takes the lowest `u32`-sized bytes, that is, the lowest four
    /// bytes from `self`, and return then as `u32` data type.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_uint(163_u8);
    /// let b_u32 = a_biguint.into_u32();
    /// println!("u32 of {} = {}", a_biguint, b_u32);
    /// assert_eq!(b_u32, 163_u32);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_u32 = a_biguint.into_u32();
    /// println!("u32 of {} = {}", a_biguint, b_u32);
    /// assert_eq!(b_u32, 1065418496_u32);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn into_u32(&self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_u16(&self) -> u16
    /// Converts `self` into `u16`,
    /// and returns the `u16`-type small unsigned integer.
    /// This mathod into_u16() is useful especially when `self` has `u16`
    /// type unsigned integer sized value and you want to cast `self` into
    /// `u16` type unsigned integer.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest two bytes of `self` as `u16`.
    /// 
    /// # Features
    /// It takes the lowest `u16`-sized bytes, that is, the lowest two
    /// bytes from `self`, and return then as `u16` data type.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(163_u8);
    /// let b_u16 = a_biguint.into_u16();
    /// println!("u16 of {} = {}", a_biguint, b_u16);
    /// assert_eq!(b_u16, 163_u16);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_u16 = a_biguint.into_u16();
    /// println!("u16 of {} = {}", a_biguint, b_u16);
    /// assert_eq!(b_u16, 65280_u16);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn into_u16(&self) -> u16
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_u8(&self) -> u8
    /// Converts `self` into `u8`,
    /// and returns the `u8`-type small unsigned integer.
    /// This mathod into_u8() is useful especially when `self` has `u8`
    /// type unsigned integer sized value and you want to cast `self` into
    /// `u8` type unsigned integer.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest one byte of `self` as `u32`.
    /// 
    /// # Features
    /// It takes the lowest `u8`-sized byte, that is, the lowest one
    /// byte from `self`, and return it as `u8` data type.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(163_u8);
    /// let b_u8: u8 = a_biguint.into_u8();
    /// println!("u8 of {} = {}", a_biguint, b_u8);
    /// assert_eq!(b_u8, 163_u8);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_u8: u8 = a_biguint.into_u8();
    /// println!("u8 of {} = {}", a_biguint, b_u8);
    /// assert_eq!(b_u8, 0_u8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn into_u8(&self) -> u8
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn into_usize(&self) -> usize
    /// Converts `self` into `usize`,
    /// and returns the `usize`-type small unsigned integer.
    /// This mathod into_usize() is useful especially when `self` has `usize`
    /// type unsigned integer sized value and you want to cast `self` into
    /// `usize` type unsigned integer.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest `usize` long part of `self` as `usize`.
    /// 
    /// # Features
    /// It takes the lowest `usize`-sized bytes from `self`,
    /// and then returns them as `usize` data type.
    /// It is usually lossy conversion.
    /// Always, the field `flag` is not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(65280_u16);
    /// let b_usize = a_biguint.into_usize();
    /// println!("usize of {} = {}", a_biguint, b_usize);
    /// assert_eq!(b_usize, 65280_usize);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// let b_usize = a_biguint.into_usize();
    /// println!("usize of {} = {}", a_biguint, b_usize);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(b_usize, 10308603139955162880_usize);
    /// #[cfg(target_pointer_width = "32")] assert_eq!(b_usize, 1065418496_usize);
    /// #[cfg(target_pointer_width = "16")] assert_eq!(b_usize, 65280_usize);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn into_usize(&self) -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_be(&self) -> Self
    /// Converts `self` to big endian expression from the target's endianness,
    /// and returns the result.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the converted `self` to big endian expression
    /// from the target's endianness.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_be();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// #[cfg(target_endian = "big")]       assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.to_be();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.to_be();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn to_be(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_be_assign(&mut self)
    /// Converts `self` to big endian expression from the target's endianness,
    /// and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
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
    /// a_biguint.to_be_assign();
    /// println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// #[cfg(target_endian = "big")]       assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
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
    /// let mut a_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.to_be_assign();
    /// println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.to_be_assign();
    /// println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn to_be_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_be_bytes(&self) -> [T; N]
    /// Return the memory representation of this integer as a byte array
    /// in big-endian (network) byte order.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the converted number of `self` to big endian expression
    /// from the target's endianness.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
    /// 
    /// # Example 1
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_be_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => {
    ///             #[cfg(target_endian = "little")]    assert_eq!(a_txt, "[170, 204, 240, 255, 0, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 179, 143, 15, 131, 240, 63, 128, 255, 0]");
    ///             #[cfg(target_endian = "big")]       assert_eq!(a_txt, "[0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170]");
    ///         },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.to_be_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => { assert_eq!(a_txt, "[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]"); },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.to_be_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => { assert_eq!(a_txt, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"); },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn to_be_bytes(&self) -> [T; N]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_le(&self) -> Self
    /// Converts `self` to little endian from the target’s endianness,
    /// and returns the result.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the converted `self` to little endian expression
    /// from the target's endianness.
    /// 
    /// # Features
    /// - On little endian machine, this is a no-op.
    /// - On big endian machine, the bytes are swapped.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_le();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// #[cfg(target_endian = "big")]       assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.to_le();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.to_le();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn to_le(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
    // pub fn to_le_assign(&mut self)
    /// Converts `self` to little endian from the target’s endianness,
    /// and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - On little endian machine, this is a no-op.
    /// - On big endian machine, the bytes are swapped.
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
    /// a_biguint.to_le_assign();
    /// println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// #[cfg(target_endian = "big")]       assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
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
    /// let mut a_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.to_le_assign();
    /// println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.to_le_assign();
    /// println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn to_le_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_le_bytes(&self) -> [T; N]
    /// Return the memory representation of this integer as a byte array
    /// in little-endian byte order, and returns the result.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - On little endian machine, this is a no-op.
    /// - On big endian machine, the bytes are swapped.
    /// 
    /// # Example 1
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_le_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => {
    ///             #[cfg(target_endian = "little")]    assert_eq!(a_txt, "[0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170]");
    ///             #[cfg(target_endian = "big")]       assert_eq!(a_txt, "[170, 204, 240, 255, 0, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 179, 143, 15, 131, 240, 63, 128, 255, 0]");
    ///         },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.to_le_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => { assert_eq!(a_txt, "[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]"); },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.to_le_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => { assert_eq!(a_txt, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"); },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn to_le_bytes(&self) -> [T; N]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_string_with_radix_and_stride_and_delimiter(&self, _radix: usize, _stride: usize, _delimiter: &str) -> Result<String, NumberErr>
    /// Reads the value of `BigUInt<T, N>`, and writes it into String
    /// in `radix`-ary system with a delimiter indicated by `delimiter`
    /// every `stride` digits.
    /// 
    /// # Arguments
    /// - `radix` is the numerical system, and is of `usize` type.
    /// - `delimiter` is the delimiter to separate digits.
    /// - `stride` is the number of digits.
    ///   The delimiter `delimiter` is marked every `stride` digits.
    /// 
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a `String`-typed sring that shows the value of
    /// `BigUInt<T, N>` in `radix`-ary system with a delimiter indicated
    /// by `delimiter` every `stride` digits.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Stride
    /// In the number expression in a string, you can separate the digits every
    /// certain number of digits which is called stride. For example, if
    /// `stride` is 4, the delimeter will be added every four digits. So,
    /// `100000000` will be written as "1_0000_0000".
    /// 
    /// # Replaceable Delimiter
    /// In the number expression in a string, you can replace the default
    /// delimiter with any `str` such as "," or "--" in order to make it more
    /// readable. So, `100000000` will be written as "1,0000,0000" or
    /// "1--0000--0000", for example.
    /// 
    /// # Error
    /// | argument | value                              | Caused Error                      |
    /// |----------|------------------------------------|-----------------------------------|
    /// | `radix`  | less than `2` or greater than `62` | `NumberErr::OutOfValidRadixRange` |
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(10, 3, ",").unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(10, 3, ",").unwrap(), "77,255,284,354,385,016,970,177,264,758,879,158,019,392,010,587,479,561,699,232,008,238,232,688,983,808");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(16, 4, "-").unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(16, 4, "-").unwrap(), "AACC-F0FF-00FF-FF00-00FF-FFFF-0000-00FF-FFFF-FF00-0000-00B3-8F0F-83F0-3F80-FF00");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(2, 8, "_").unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(2, 8, "_").unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(36, 4, ":").unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(36, 4, ":").unwrap(), "49:93ID:4SD9:M4DT:2QO9:EF7V:ZKGD:LH3S:Y0SO:W4CH:RKE5:CQA4:0MPS");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(62, 4, "~").unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(62, 4, ":").unwrap(), "eV7:xhnH:Dmgs:yLnq:m5P9:ZaJf:dOP0:7xlq:S2Da:BiV2:F7dg");
    /// ```
    /// 
    /// # Error Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// match a_biguint.to_string_with_radix_and_stride_and_delimiter(1, 4, "$")
    /// {
    ///     Ok(txt) => { println!("a_biguint = {}", txt); },
    ///     Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    /// }
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// match a_biguint.to_string_with_radix_and_stride_and_delimiter(63, 4, "~")
    /// {
    ///     Ok(txt) => { println!("a_biguint = {}", txt); },
    ///     Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn to_string_with_radix_and_stride_and_delimiter(&self, _radix: usize, _stride: usize, _delimiter: &str) -> Result<String, NumberErr>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_string_with_radix_and_stride(&self, _radix: usize, _stride: usize) -> Result<String, NumberErr>
    /// Reads the value of `BigUInt<T, N>` and write it into String
    /// in `radix`-ary system with delimiter '_' every `stride` digits.
    /// 
    /// # Arguments
    /// - `radix` is the numerical system, and is of `usize` type.
    /// - `stride` is the number of digits.
    ///   The delimiter '_' is marked every `stride` digits.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a sring that shows the value of `BigUInt<T, N>`.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Stride
    /// In the number expression in a string, you can separate the digits every
    /// certain number of digits which is called stride. For example, if
    /// `stride` is 4, the delimeter will be added every four digits. So,
    /// `100000000` will be written as "1_0000_0000".
    /// 
    /// # Default Delimiter _
    /// In the number expression in a string, you can separate the digits with
    /// the default delimiter '_' in order to make it more readable. So, "10000"
    /// is the same as "1_0000".
    /// 
    /// # Error
    /// | argument | value                              | Caused Error                      |
    /// |----------|------------------------------------|-----------------------------------|
    /// | `radix`  | less than `2` or greater than `62` | `NumberErr::OutOfValidRadixRange` |
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(10, 3).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(10, 3).unwrap(), "77_255_284_354_385_016_970_177_264_758_879_158_019_392_010_587_479_561_699_232_008_238_232_688_983_808");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 4).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 4).unwrap(), "AACC_F0FF_00FF_FF00_00FF_FFFF_0000_00FF_FFFF_FF00_0000_00B3_8F0F_83F0_3F80_FF00");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(36, 6).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(36, 6).unwrap(), "49_93ID4S_D9M4DT_2QO9EF_7VZKGD_LH3SY0_SOW4CH_RKE5CQ_A40MPS");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(62, 5).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(62, 5).unwrap(), "eV7_xhnHD_mgsyL_nqm5P_9ZaJf_dOP07_xlqS2_DaBiV_2F7dg");
    /// ```
    /// 
    /// # Error Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// match a_biguint.to_string_with_radix_and_stride(1, 4)
    /// {
    ///     Ok(txt) => { println!("a_biguint = {}", txt); },
    ///     Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    /// }
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// match a_biguint.to_string_with_radix_and_stride(63, 5)
    /// {
    ///     Ok(txt) => { println!("a_biguint = {}", txt); },
    ///     Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn to_string_with_radix_and_stride(&self, _radix: usize, _stride: usize) -> Result<String, NumberErr>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn to_string_with_radix(&self, _radix: usize) -> Result<String, NumberErr>
    /// Reads the value of `BigUInt<T, N>` and write it into String
    /// in `radix`-ary system.
    /// 
    /// # Arguments
    /// `radix` is the numerical system, and is of `usize` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a `String`-typed sring
    /// that shows the value of `BigUInt<T, N>` in `radix`-ary system.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Error
    /// | argument | value                              | Caused Error                      |
    /// |----------|------------------------------------|-----------------------------------|
    /// | `radix`  | less than `2` or greater than `62` | `NumberErr::OutOfValidRadixRange` |
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix(10).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix(10).unwrap(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix(16).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix(16).unwrap(), "AACCF0FF00FFFF0000FFFFFF000000FFFFFFFF00000000B38F0F83F03F80FF00");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix(2).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix(2).unwrap(), "1010101011001100111100001111111100000000111111111111111100000000000000001111111111111111111111110000000000000000000000001111111111111111111111111111111100000000000000000000000000000000101100111000111100001111100000111111000000111111100000001111111100000000");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix(36).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix(36).unwrap(), "4993ID4SD9M4DT2QO9EF7VZKGDLH3SY0SOW4CHRKE5CQA40MPS");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix(62).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix(62).unwrap(), "eV7xhnHDmgsyLnqm5P9ZaJfdOP07xlqS2DaBiV2F7dg");
    /// ```
    /// 
    /// # Error Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// match a_biguint.to_string_with_radix(1)
    /// {
    ///     Ok(txt) => { println!("a_biguint = {}", txt); },
    ///     Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    /// }
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// match a_biguint.to_string_with_radix(63)
    /// {
    ///     Ok(txt) => { println!("a_biguint = {}", txt); },
    ///     Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn to_string_with_radix(&self, _radix: usize) -> Result<String, NumberErr>
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***** FLAG MANIPULATION *****/

    // pub fn set_overflow(&mut self)
    /// Sets `OVERFLOW` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_overflow(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_overflow(&mut self)
    /// Resets `OVERFLOW` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_overflow(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_overflow(&self) -> bool
    /// Checks whether or not `OVERFLOW` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `OVERFLOW` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_overflow(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_underflow(&mut self)
    /// Sets `UNDERFLOW` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_underflow(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_underflow(&mut self)
    /// Reets `UNDERFLOW` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_underflow(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_underflow(&self) -> bool
    /// Checks whether or not `UNDERFLOW` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `UNDERFLOW` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_underflow(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_infinity(&mut self)
    /// Sets `INFINITY` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_infinity(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_infinity(&mut self)
    /// Resets infinity flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_infinity(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_infinity(&self) -> bool
    /// Checks whether or not `INFINITY` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `INFINITY` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_infinity(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_divided_by_zero(&mut self)
    /// Sets `DIVIDED_BY_ZERO` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_divided_by_zero(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_divided_by_zero(&mut self)
    /// Resets `DIVIDED_BY_ZERO` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_divided_by_zero(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_divided_by_zero(&self) -> bool
    /// Checks whether or not `DIVIDED_BY_ZERO` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `DIVIDED_BY_ZERO` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_divided_by_zero(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_undefined(&mut self)
    /// Sets `UNDEFINED` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_undefined(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_undefined(&mut self)
    /// Resets `UNDEFINED` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_undefined(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_undefined(&self) -> bool
    /// Checks whether or not `UNDEFINED` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `UNDEFINED` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.set_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_undefined(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_left_carry(&mut self)
    /// Sets `LEFT_CARRY` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_left_carry(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_left_carry(&mut self)
    /// Resets `LEFT_CARRY` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_left_carry(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_left_carry(&self) -> bool
    /// Checks whether or not `LEFT_CARRY` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `LEFT_CARRY` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_left_carry(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_right_carry(&mut self)
    /// Sets `RIGHT_CARRY` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// ```
    #[inline]
    pub fn set_right_carry(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_right_carry(&mut self)
    /// Resets `RIGHT_CARRY` flag.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// 
    /// a_biguint.reset_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_right_carry(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_right_carry(&self) -> bool
    /// Checks whether or not `RIGHT_CARRY` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `RIGHT_CARRY` flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// ```
    #[inline]
    pub fn is_right_carry(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
}