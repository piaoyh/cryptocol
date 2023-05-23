//
// Struct BigUInt: the base struct of all cryptography struct for Little Endian
// Version:				0.1.0.0
// Author:				PARK Youngho
//

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
use std::mem::{ size_of, transmute_copy, zeroed };
use std::ops::*;
use std::cmp::{PartialEq, PartialOrd, Ordering};

use super::primitive::*;

type u_max = u128;
type u_half = u64;

type u256_with_u128 = BigUInt<u128, 2>;
type u512_with_u128 = BigUInt<u128, 4>;
type u1024_with_u128 = BigUInt<u128, 8>;
type u2048_with_u128 = BigUInt<u128, 16>;
type u3072_with_u128 = BigUInt<u128, 24>;
type u4096_with_u128 = BigUInt<u128, 32>;
type u5120_with_u128 = BigUInt<u128, 40>;
type u6144_with_u128 = BigUInt<u128, 48>;
type u7168_with_u128 = BigUInt<u128, 56>;
type u8192_with_u128 = BigUInt<u128, 64>;
type u16384_with_u128 = BigUInt<u128, 128>;

type u256_with_u64 = BigUInt<u64, 4>;
type u512_with_u64 = BigUInt<u64, 8>;
type u1024_with_u64 = BigUInt<u64, 16>;
type u2048_with_u64 = BigUInt<u64, 32>;
type u3072_with_u64 = BigUInt<u64, 48>;
type u4096_with_u64 = BigUInt<u64, 64>;
type u5120_with_u64 = BigUInt<u64, 80>;
type u6144_with_u64 = BigUInt<u64, 96>;
type u7168_with_u64 = BigUInt<u64, 112>;
type u8192_with_u64 = BigUInt<u64, 128>;
type u16384_with_u64 = BigUInt<u64, 256>;

type u256_with_u32 = BigUInt<u32, 8>;
type u512_with_u32 = BigUInt<u32, 16>;
type u1024_with_u32 = BigUInt<u32, 32>;
type u2048_with_u32 = BigUInt<u32, 64>;
type u3072_with_u32 = BigUInt<u32, 96>;
type u4096_with_u32 = BigUInt<u32, 128>;
type u5120_with_u32 = BigUInt<u32, 160>;
type u6144_with_u32 = BigUInt<u32, 192>;
type u7168_with_u32 = BigUInt<u32, 224>;
type u8192_with_u32 = BigUInt<u32, 256>;
type u16384_with_u32 = BigUInt<u32, 512>;

type u256_with_u16 = BigUInt<u16, 16>;
type u512_with_u16 = BigUInt<u16, 32>;
type u1024_with_u16 = BigUInt<u16, 64>;
type u2048_with_u16 = BigUInt<u16, 128>;
type u3072_with_u16 = BigUInt<u16, 192>;
type u4096_with_u16 = BigUInt<u16, 256>;
type u5120_with_u16 = BigUInt<u16, 320>;
type u6144_with_u16 = BigUInt<u16, 384>;
type u7168_with_u16 = BigUInt<u16, 448>;
type u8192_with_u16 = BigUInt<u16, 512>;
type u16384_with_u16 = BigUInt<u16, 1024>;

type u256_with_u8 = BigUInt<u8, 32>;
type u512_with_u8 = BigUInt<u8, 64>;
type u1024_with_u8 = BigUInt<u8, 128>;
type u2048_with_u8 = BigUInt<u8, 256>;
type u3072_with_u8 = BigUInt<u8, 384>;
type u4096_with_u8 = BigUInt<u8, 512>;
type u5120_with_u8 = BigUInt<u8, 640>;
type u6144_with_u8 = BigUInt<u8, 768>;
type u7168_with_u8 = BigUInt<u8, 896>;
type u8192_with_u8 = BigUInt<u8, 1024>;
type u16384_with_u8 = BigUInt<u8, 2048>;

type u256 = u256_with_u64;
type u512 = u512_with_u64;
type u1024 = u1024_with_u64;
type u2048 = u2048_with_u64;
type u3072 = u3072_with_u64;
type u4096 = u4096_with_u64;
type u5120 = u5120_with_u64;
type u6144 = u6144_with_u64;
type u7168 = u7168_with_u64;
type u8192 = u8192_with_u64;
type u16384 = u16384_with_u64;

type U32 = u256;
type U64 = u512;
type U128 = u1024;
type U256 = u2048;
type U384 = u3072;
type U512 = u4096;
type U640 = u5120;
type U768 = u6144;
type U896 = u7168;
type U1024 = u8192;
type U2048 = u16384;


#[derive(Debug, Copy, Clone)]
pub struct BigUInt<T, const N: usize>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    number: [T; N],
    flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone
        + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
{

    const OVERFLOW: u8  = 0b0000_0001;
    const UNDERFLOW: u8 = 0b0000_0010;
    const INFINITY: u8  = 0b0000_0100;
    const DIVIDED_BY_ZERO: u8 = Self::INFINITY;


    /// Constructs a new BigUInt<T, N>.
    /// All the attributes of te constructed object will be initialized with 0.
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let big_int = BigUInt<u64,16>::new();
    /// ```
    pub fn new() -> Self
    {
        Self { number: [T::zero(); N], flag: 0, }   // unsafe { zeroed::<Self>() }
    }

    /// Constructs a new BigUInt<T, N> which has the value of zero.
    /// This function calls BigUInt<T, N>::new(), so it is virtually exactly the same as the function BigUInt<T, N>::new(). Your source code will be better readable if you use BigUInt<T, N>::zero() instead of BigUInt<T, N>::new() especially when you create the big number zero.
    ///
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let zero = BigUInt<u64,16>::zero();
    /// ```
    pub fn zero() -> Self
    {
        Self::new()   // unsafe { zeroed::<Self>() }
    }

    /// Constructs a new BigUInt<T, N> from an array of type T with N elements.
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let big_num = BigUInt<u8,32>::from_array(&[1;32]);
    /// ```
    pub fn from_array(val: &[T; N]) -> Self
    {
        let mut s = Self::new();
        s.set_number(val);
        s
    }

    /// Constructs a new BigUInt<T, N> from an unsigned integer such as u8, u16, u32, u64, u128 and usize.
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let bi = BigUInt<u8,32>::from_uint(1004);
    /// ```
    pub fn from_uint<V: Copy>(val: V) -> Self
    {
        union U<VV: Copy, TT: Copy>
        {
            ulonger: ULonger,
            v: VV,
            t: TT,
        }

        let TSIZE = size_of::<T>();
        let mut s = Self::new();
        let mut tu: U<V, T> = U { ulonger: ULonger::new() };
        tu.v = val;
        if TSIZE >= size_of::<V>()
        {
            unsafe { s.set_num(0, tu.t); }
        }
        else
        {
            for i in 0..(size_of::<V>()/TSIZE)
            {
                unsafe {
                    s.set_num(i, tu.t);
                    tu.ulonger.ulonger >>= TSIZE * 8;
                }
            }
        }
        return s;
    }

    /// Constructs a new BigUInt<T, N> from a string with radix.
    /// The constructed object will be wrapped in Some(BigUInt<T, N>) if it is successfully created. Otherwise, this method returns None.
    /// The radix can be from 2 up to 62 (= 10 + 26 + 26). The radix 1 or more than 62 is not available, so that this method will return None. If the radix is more than 10 and less than 37, the digit bigger than 9 will be expressed with alphabets. The avaiable alphabets are case-insensitive. For example, the digit whose value is 10, 11, 15, 16, 35 and 36 are A or a, B or b, F or f, G or g, Y or y, and Z or z, respectively. However, if the radix is more than 36 and less than 62, the digit bigger than 9 will be expressed with alphabets. The avaiable alphabets are case-sensitive, so A is different from a. For instance, the digit whose value is 10, 11, 35, 36, 37, 38, 60 and 61 are A, B, Y, Z, a, b, y and z, respectively.
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let bi = BigUInt<u8,32>::from_string_with_radix("A16F", 16);
    /// ```
    pub fn from_string_with_radix(txt: &str, radix: usize) -> Option<Self>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return None; }

        let mut bignum = Self::new();
        for c in txt.chars()
        {
            if c == '_'
                { continue; }
            if !c.is_alphanumeric()
                { return None; }
            if radix <= 10
            {
                if c as usize >= '0' as usize + radix
                    { return None; }
            }
            else if radix <= 10 + 26
            {
                if (c as usize >= 'A' as usize + radix - 10)
                        ||  (c as usize >= 'a' as usize + radix - 10)
                    { return None; }
            }
            else if c as usize >= 'a' as usize + radix - (10 + 26)  // radix <= 10 + 26 + 26
                { return None; }

            let num: usize = if radix <= 10    { c as usize - '0' as usize }
                        else if radix <= 10 + 26
                        {
                            if c as usize <= '9' as usize
                                { c as usize - '0' as usize }
                            else if c as usize <= 'Z' as usize
                                { c as usize - 'A' as usize + 10 }
                            else
                                { c as usize - 'a' as usize + 10 }
                        }
                        else    // if radix <= 10 + 26 + 26
                        {
                            if c as usize <= '9' as usize
                                { c as usize - '0' as usize }
                            else if c as usize <= 'Z' as usize
                                { c as usize - 'A' as usize + 10 }
                            else
                                { c as usize - 'a' as usize + 10 + 26 }
                        };
            bignum.times(T::num(radix as u128));
            bignum.accumulate(T::num(num as u128));
        }
        Some(bignum)
    }

    pub fn from_string(txt: &str) -> Option<Self>
    {
        Self::from_string_with_radix(txt, 10)
    }

    fn make_check_bits(bit_pos: usize) -> Self
    {
        let mut check_bits = Self::new();
        check_bits.turn_check_bits(bit_pos);
        check_bits
    }
    
    fn turn_check_bits(&mut self, bit_pos: usize)
    {
        let TSIZE: usize = size_of::<T>();
        let chunk_num = bit_pos / (8 * TSIZE);
        let piece_num = bit_pos % (8 * TSIZE);
        let mut val = T::one();
        val <<= T::num(piece_num as u128);
        self.set_zero();
        self.set_num(chunk_num, val);
    }

    pub fn to_string_with_radix(&self, radix: usize) -> String
    {
        let mut txt = String::new();
        let zero = Self::zero();
        let mut dividend = self.clone();
        let mut remainder;
        while dividend != zero
        {
            (dividend, remainder) = dividend.divide_fully(Self::from_uint(radix as u128));
            let r = remainder.get_num(0).into_u32();
            let c: char = if r < 10     { ('0' as u32 + r) as u8 as char }
                    else if r < 10 + 26 { ('A' as u32 - 10 + r) as u8 as char }
                    else                { ('a' as u32 - 10 - 26 + r) as u8 as char};  // r < 10 + 26 + 26
            txt.push(c);
        }
        if txt.len() == 0
            { txt.push('0'); }
        let mut num_str = String::new();
        while let Some(ch) = txt.pop()
            { num_str.push(ch); }
        num_str
    }

    pub fn divide_fully(&mut self, rhs: Self) -> (Self, Self)
    {
        let mut quotient = Self::new();
        if rhs.is_zero()
        {
            quotient.set_divided_by_zero();
            quotient.set_overflow();
            quotient.set_max();
            let mut remainder = Self::new();
            remainder.set_divided_by_zero();
            return (quotient, remainder);
        }
        if *self < rhs
        {
            return (quotient, self.clone());
        }
        else if *self == rhs
        {
            quotient.set_uint(T::one());
            return (quotient, Self::zero());
        }

        let TSIZE: usize = size_of::<T>();
        let mut adder = Self::zero();
        let mut res;
        let mut sum;
        let mut highest = N * TSIZE * 8;
        let mut high = highest;
        let mut low = 0;
        let mut mid = (high + low) >> 1;
        loop
        {
            high = highest;
            low = 0;
            if high == 0
            {
                return (quotient, *self - quotient * rhs);
            }
            else    // if high > 0
            {
                loop
                {
                    mid = (high + low) >> 1;
                    adder.turn_check_bits(mid);
                    sum = quotient + adder;
                    res = sum * rhs;
                    if *self > res
                    {
                        if mid == low
                        { 
                            quotient = sum;
                            highest = mid;
                            break;
                        }
                        low = mid;
                    }
                    else if res > *self
                    {
                        if mid == low
                        { 
                            highest = mid;
                            break;
                        }
                        high = mid;
                    }
                    else    // if res == *self
                        { return (quotient + adder, Self::zero()); }
                }
            }
        }
/*
        let mut search = || -> Option<(Self, Self)> {
            while high > low
            {
                mid = (high + low) >> 1;
                adder.turn_check_bits(mid);
                sum = quotient + adder;
                res = sum * rhs;
                if res < *self
                    { low = mid + 1; }
                else if res > *self
                    { high = mid - 1; }
                else    // if res == *self
                    { return Some((sum, Self::new())); }
            }
            if mid > high   // It means "when res > *self".
            {
                adder.turn_check_bits(low);
                quotient += adder;
            }
            else    // if mid < low // It means that when *self > res
            {
                adder.turn_check_bits(high);
                sum = quotient + adder;
                res = sum * rhs;
                if res < *self
                    { quotient += adder; }
                else    // if res > *self
                {
                    adder.turn_check_bits(mid);
                    sum = quotient + adder;
                    res = sum * rhs;
                    if res < *self
                        { quotient += adder; }
                    else if high == 0
                        { return Some((sum, sum - rhs)); }
                }
            }
            highest = if mid > low {low} else {mid};
            None
        };
*/
    }

    pub fn divide_by_uint_fully(&mut self, rhs: T) -> (Self, T)
    {
        let zero = T::zero();
        let one = T::one();
        let mut quotient = Self::new();
        if rhs == zero
        {
            quotient.set_divided_by_zero();
            quotient.set_overflow();
            quotient.set_max();
            let remainder = zero;
            return (quotient, remainder);
        }
        if self.lt_uint(rhs)
        {
            return (quotient, self.get_num(0));
        }
        else if self.eq_uint(rhs)
        {
            quotient.set_uint(T::one());
            return (quotient, zero);
        }

        let mut adder = one;
        let mut res;
        loop
        {
            adder = one;
            res = quotient.add_uint(adder).mul_uint(rhs);
            while (*self >= res) && !res.is_overflow()
            {
                adder <<= one;
                res = quotient.add_uint(adder).mul_uint(rhs);
            }
            adder >>= one;
            if adder == zero
                { break; }
            else
                { quotient.accumulate(adder); }
        }
        (quotient, (*self - quotient.mul_uint(rhs)).get_num(0))
    }

    pub fn times(&mut self, rhs: T)
    {
        let zero = T::zero();
        let one = T::one();
        if rhs == zero
        {
            self.set_zero();
            return;
        }

        let adder = self.clone();
        let TSIZE = size_of::<T>();
        let mut bit_check = one;
        bit_check <<= T::num((TSIZE - 1).into_u128());
        while (bit_check != zero) && ((bit_check & rhs) == zero)
            { bit_check >>= one; }

        bit_check >>= one;
        while bit_check != zero
        {
            *self <<= 1;
            if bit_check & rhs != zero
                { *self += adder; }
            bit_check >>= one;
        }
    }

    pub fn quotient(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self /= bi;
    }

    pub fn remainder(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self %= bi;
    }

    pub fn add_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.accumulate(rhs);
        bi
    }

    pub fn sub_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.dissipate(rhs);
        bi
    }

    pub fn mul_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.times(rhs);
        bi
    }

    pub fn div_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.quotient(rhs);
        bi
    }

    pub fn rem_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.remainder(rhs);
        bi
    }

    fn partial_cmp_uint(&self, other: T) -> Option<Ordering>
    {
        if self.number[0] > other
        {
            return Some(Ordering::Greater);
        }
        else if self.number[0] < other
        {
            for idx in 1..N
            {
                if self.number[idx] != T::zero()
                    { return Some(Ordering::Greater); }
            }
            return Some(Ordering::Less);
        }
        else    // if self.number[0] == other
        {
            for idx in 1..N
            {
                if self.number[idx] != T::zero()
                    { return Some(Ordering::Greater); }
            }
        }
        Some(Ordering::Equal)
    }

    pub fn lt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_lt() }
    pub fn gt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_gt() }
    pub fn le_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_le() }
    pub fn ge_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_ge() }
    pub fn eq_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_eq() }

    pub fn get_num(&self, i: usize) -> T    { self.number[i] }
    pub fn set_num(&mut self, i: usize, val: T) { self.number[i] = val; }
    pub fn set_number(&mut self, val: &[T; N])
    {
        self.number.copy_from_slice(val);
    }
    
    pub fn set_zero(&mut self)
    {
        for it in &mut self.number
            { *it = T::zero(); }
    }

    pub fn is_zero(&self) -> bool
    {
        for it in &self.number
        {
            if *it != T::zero()
                { return false; }
        }
        true
    }

    pub fn set_max(&mut self)
    {
        for it in &mut self.number
            { *it = T::Max(); }
    }

    pub fn is_max(&self) -> bool
    {
        for it in &self.number
        {
            if *it != T::Max()
                { return false; }
        }
        true
    }

    pub fn set_uint(&mut self, val: T)
    {
        self.set_zero();
        self.number[0] = val;
    }

    pub fn is_uint(&self, val: T) -> bool
    {
        if self.number[0] != val
            { return false; }
        for i in 1..N
        {
            if self.number[i] != T::zero()
                { return false; }
        }
        true
    }

    fn set_flag_bit(&mut self, flag_bits: u8)   { self.flag |= flag_bits }
    fn reset_flag_bit(&mut self, flag_bits: u8) { self.flag &= !flag_bits }
    fn is_flag_bit_on(&self, flag_bits: u8) -> bool { (self.flag & flag_bits) != 0 }






    pub fn accumulate(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self += bi;
    }

    pub fn dissipate(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self -= bi;
    }

    pub fn to_string(&self) -> String   { self.to_string_with_radix(10) }
    pub fn set_overflow(&mut self)      { self.set_flag_bit(Self::OVERFLOW); }
    pub fn reset_overflow(&mut self)    { self.reset_flag_bit(Self::OVERFLOW); }
    pub fn is_overflow(&self) -> bool   { self.is_flag_bit_on(Self::OVERFLOW) }
    pub fn set_underflow(&mut self)     { self.set_flag_bit(Self::UNDERFLOW) }
    pub fn reset_underflow(&mut self)   { self.reset_flag_bit(Self::UNDERFLOW) }
    pub fn is_underflow(&self) -> bool  { self.is_flag_bit_on(Self::UNDERFLOW) }
    pub fn set_inifinity(&mut self)     { self.set_flag_bit(Self::INFINITY); }
    pub fn reset_inifinity(&mut self)   { self.reset_flag_bit(Self::INFINITY); }
    pub fn is_inifinity(&self) -> bool  { self.is_flag_bit_on(Self::INFINITY) }
    pub fn set_untrustable(&mut self)   { self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    pub fn reset_untrustable(&mut self) { self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    pub fn is_untrustable(&self) -> bool { self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW) }
    pub fn set_divided_by_zero(&mut self)   { self.set_inifinity(); }
    pub fn reset_divided_by_zero(&mut self) { self.reset_inifinity(); }
    pub fn is_divided_by_zero(&self) -> bool { self.is_inifinity() }
}


/*
impl<T, const N: usize> Large_Integer<T, N> for BigUInt<T, N>
where T: Uint + Add + Sub + Mul + Div
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd + BitAndAssign + BitOr + BitOrAssign
        + BitXorAssign + Not,
    Self: Sized + Clone
        + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
{

}
*/



impl<T, const N: usize> Add for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s += rhs;
        s
    }
}

impl<T, const N: usize> AddAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn add_assign(&mut self, rhs: Self)
    {
        let mut	carry: T = T::zero();
        let mut midres: T;
        let mut c: bool;

        for i in 0..N
        {
            midres = self.number[i].wrapping_add(rhs.number[i]);
            c = midres < self.number[i];
            midres = midres.wrapping_add(carry);
            carry = if c || (midres < carry) { T::one() } else { T::zero() };
            self.number[i] = midres;
        }
        if carry != T::zero()
        {
            if !self.is_untrustable()
            {
                if self.is_underflow()
                    { self.reset_underflow(); }
                else if self.is_overflow()
                    { self.set_untrustable(); }
                else
                    { self.set_overflow(); }
            }
        }
    }
}

impl<T, const N: usize> Sub for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s -= rhs;
        s
    }
}

impl<T, const N: usize> SubAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn sub_assign(&mut self, rhs: Self)
    {
        let mut	carry: T = T::zero();
        let mut midres: T;
        let mut c: bool;
        let mut cc: T;

        for i in 0..N
        {
            midres = self.number[i].wrapping_sub(rhs.number[i]);
            c = midres > self.number[i];
            cc = midres;
            midres = midres.wrapping_sub(carry);
            carry = if c || (midres > cc) { T::one() } else { T::zero() };
            self.number[i] = midres;
        }
        if carry != T::zero()
        {
            if !self.is_untrustable()
            {
                if self.is_overflow()
                    { self.reset_overflow(); }
                else if self.is_underflow()
                    { self.set_untrustable(); }
                else
                    { self.set_underflow(); }
            }
        }
    }
}

impl<T, const N: usize> Mul for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s *= rhs;
        s
    }
}

impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn mul_assign(&mut self, rhs: Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE = size_of::<T>();
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE - 1).into_u128());
            while bit_check & num == zero
                { bit_check >>= one; }
            bit_check >>= one;
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        let mut n = N - 1;
        while rhs.number[n] == zero
            { n = n.wrapping_sub(1); }
        multiply_first(rhs.number[n]);
        n = n.wrapping_sub(1);

        let mut multiply = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE - 1).into_u128());
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
            n = n.wrapping_sub(1);
        }
    }
}

impl<T, const N: usize> Div for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self
    {
        let mut quotient = Self::new();
        if rhs.is_zero()
        {
            quotient.set_divided_by_zero();
            quotient.set_max();
            return quotient;
        }

        let mut dividend = self.clone();
        let mut subquotient = Self::new();
        let mut subdividend: BigUInt<T, N>;
        while dividend >= rhs
        {
            subquotient.set_uint(T::one());
            subdividend = dividend >> 1;
            while subdividend > rhs
            {
                subquotient <<= 1;
                subdividend >>= 1;
            }
            dividend -= subquotient * rhs;
            quotient += subquotient;
        }
        quotient
    }
}

impl<T, const N: usize> DivAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl<T, const N: usize> Rem for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s %= rhs;
        s
    }
}

impl<T, const N: usize> RemAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn rem_assign(&mut self, rhs: Self)
    {
        if rhs.is_zero()
        {
            self.set_divided_by_zero();
            self.set_zero();
            return;
        }
        let mut	rhs_acc = Self::new();
        let mut self_acc = Self::new();
        while *self >= rhs
        {
            rhs_acc.set_number(&rhs.number);
            self_acc.set_number(&self.number);
            self_acc >>= 1;
            while rhs_acc <= self_acc
                { rhs_acc <<= 1; }
            *self -= rhs_acc;
        }
    }
}

impl<T, const N: usize> Shl<i32> for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn shl(self, rhs: i32) -> Self
    {
        let mut s = self.clone();
        s <<= rhs;
        s
    }
}

impl<T, const N: usize> ShlAssign<i32> for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn shl_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
		    *self >>= -rhs;
            return;
        }
        let TSIZE: usize = size_of::<T>();
        let chunk_num = rhs as usize / (8 * TSIZE) as usize;
        let piece_num = rhs as usize % (8 * TSIZE) as usize;

        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if self.number[i] > T::zero()
                {
                    self.set_overflow();
                    break;
                }
            }
            self.number.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.number[idx] = T::zero(); }
        }
        if piece_num == 0
            { return; }
        if !self.is_overflow() && ((self.number[N-1] >> T::num((TSIZE - piece_num).into_u128())) != T::zero())
            { self.set_overflow(); }

        let mut num: T;
        let mut carry = T::zero();
        for idx in chunk_num..N
        {
            num = (self.number[idx] << T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] >> T::num((8 * TSIZE - piece_num).into_u128());
            self.number[idx] = num;
        }
    }
}

impl<T, const N: usize> Shr<i32> for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn shr(self, rhs: i32) -> Self
    {
        let mut s = self.clone();
        s >>= rhs;
        s
    }
}

impl<T, const N: usize> ShrAssign<i32> for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn shr_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
		    *self <<= -rhs;
            return;
        }
        let TSIZE: usize = size_of::<T>();
        let chunk_num = rhs as usize / (8 * TSIZE) as usize;
        let piece_num = rhs as usize % (8 * TSIZE) as usize;
        if chunk_num > 0
        {
            for i in 0..chunk_num
            {
                if self.number[i] > T::zero()
                {
                    self.set_underflow();
                    break;
                }
            }
            self.number.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.number[idx] = T::zero(); }
        }
        if piece_num == 0
            { return; }
        if !self.is_underflow() && ((self.number[0] << T::num((TSIZE - piece_num).into_u128())) != T::zero())
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.number[idx] >> T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] << T::num((8 * TSIZE - piece_num).into_u128());
            self.number[idx] = num;
            if idx == 0
                { break; }
            idx -= 1;
        }
    }
}

impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s &= rhs;
        s
    }
}

impl<T, const N: usize> BitAndAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn bitand_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] &= rhs.number[idx]; }
    }
}

impl<T, const N: usize> BitOr for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s |= rhs;
        s
    }
}

impl<T, const N: usize> BitOrAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn bitor_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] |= rhs.number[idx]; }
    }
}

impl<T, const N: usize> BitXor for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s ^= rhs;
        s
    }
}

impl<T, const N: usize> BitXorAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn bitxor_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] ^= rhs.number[idx]; }
    }
}

impl<T, const N: usize> Not for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn not(self) -> Self
    {
        let mut s = self.clone();
        for idx in 0..N
            { s.number[idx] = !s.number[idx]; }
        s
    }
}

impl<T, const N: usize> PartialEq for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn eq(&self, other: &Self) -> bool
    {
        for idx in 0..N
        {
            if self.number[idx] != other.number[idx]
                { return false; }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool  { !(self == other) }
}

impl<T, const N: usize> PartialOrd for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        for idx in 1..=N
        {
            if self.number[N-idx] > other.number[N-idx]
                { return Some(Ordering::Greater); }
            else if self.number[N-idx] < other.number[N-idx]
                { return Some(Ordering::Less); }
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_lt() }
    fn gt(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_gt() }
    fn le(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_le() }
    fn ge(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_ge() }
}

