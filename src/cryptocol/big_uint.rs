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
    /// The constructed object will be initialized with 0.
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let bi = BigUInt<u64,16>::new();
    /// ```
    pub fn new() -> Self
    {
        Self { number: [T::zero(); N], flag: 0, }   // unsafe { zeroed::<Self>() }
    }

    /// Constructs a new BigUInt<T, N> from an array of type T with N elements.
    /// # Examples
    /// ```
    /// use Cryptocol::cryptocol::BigUInt;
    /// let bi = BigUInt<u8,32>::from_array(&[1;32]);
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
                        else    // radix <= 10 + 26 + 26
                        {
                            if c as usize <= '9' as usize
                                { c as usize - '0' as usize }
                            else if c as usize <= 'Z' as usize
                                { c as usize - 'A' as usize + 10 }
                            else
                                { c as usize - 'a' as usize + 10 + 26 }
                        };
            bignum *= Self::from_uint(radix);
            bignum += Self::from_uint(num);
        }
        Some(bignum)
    }

    pub fn from_string(txt: &str) -> Option<Self>
    {
        Self::from_string_with_radix(txt, 10)
    }
    
    pub fn to_string_with_radix(&self, radix: usize) -> String
    {
        let mut txt = String::new();
        let zero = Self::new();
        let divisor = Self::from_uint(radix);
        let mut dividend = self.clone();
        while dividend != zero
        {
            let remainder = dividend % divisor;
            let r = remainder.number[0].into_u32() as u8;
            let c: char = if r < 10     { ('0' as u8 + r) as char }
                    else if r < 10 + 26 { ('A' as u8 + r - 10 ) as char }
                    else                { ('a' as u8 + r - 10 - 26) as char};  // r < 10 + 26 + 26
            txt.push(c);
            dividend /= divisor;
        }
        if txt.len() == 0
            { txt.push('0'); }
        let mut num_str = String::new();
        while let Some(ch) = txt.pop()
            { num_str.push(ch); }
        num_str
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
        let TSIZE = size_of::<T>();
        let mut res = Self::new();
        let mut b_carry = false;
        if TSIZE == size_of::<u_max>()
        {
            union U<V: Uint>
            {
                full: V,
                half: [u_half; 2],
            }
            let mut mul: U<T> = U { full: T::zero() };
            let mut sum: U<T> = U { full: T::zero() };
            for i in 0..N*2
            {
                mul.full = T::zero();
                sum.full = T::zero();
                for j in 0..N*2
                {
                    if N * 2 <= i + j
                        { break; }

                    let me_half: U<T> = U { full: self.number[j/2] };
                    let mut me_h: U<T> = U { full: T::zero() };
                    unsafe { me_h.half[0] = me_half.half[j%2]; }

                    //let number_half: U<T> = U { full: rhs.number[i/2] };
                    let mut number_h: U<T> = U { full: T::zero() };
                    unsafe { number_h.half[0] = number_h.half[i%2]; }

                    let mut mul_h: U<T> = U { full: T::zero() };
                    unsafe { mul_h.half[0] = mul.half[1]; }

                    unsafe { mul.full = me_h.full * number_h.full + mul_h.full; }

                    let mut out_half: U<T> = U { full: self.number[(i+j)/2] };
                    let mut out_h: U<T> = U { full: T::zero() };
                    unsafe { out_h.half[0] = out_half.half[(i+j)%2]; }

                    mul_h.full = T::zero();
                    unsafe { mul_h.half[0] = mul.half[0]; }

                    let mut sum_h: U<T> = U { full: T::zero() };
                    unsafe { sum_h.half[0] = sum.half[1]; }

                    unsafe { sum.full = out_h.full + mul_h.full + sum_h.full; }

                    out_half.full = self.number[(i+j)/2];
                    unsafe { out_half.half[(i+j)%2] = sum.half[0]; }

                    if N * 2 - 2 <= i + j 
                        { unsafe { b_carry = b_carry || out_half.full.into_bool(); } }
                    else 
                        { unsafe { res.number[(i+j)/2] = out_half.full; } }
                }
            }
            if b_carry
                { res.set_untrustable(); }
        }
        else if TSIZE == size_of::<u64>()
        {
            union U<F: Uint>
            {
                wider: u128,
                full: [F; 2],
            }

            let mut mul: U<T> = U { wider: 0 };
            let mut sum: U<T> = U { wider: 0 };
            for i in 0..N
            {
                mul.wider = 0;
                mul.wider = 0;
                for j in 0..N
                {
                    if i + j >= N
                        { break; }
                    unsafe { 
                    mul.wider = self.number[j].into_u128().wrapping_mul(rhs.number[i].into_u128()).wrapping_add(mul.full[1].into_u128());
                    sum.wider = res.number[i+j].into_u128().wrapping_add(mul.full[0].into_u128()).wrapping_add(sum.full[1].into_u128());
                    res.number[i+j] = sum.full[0];
                    if i + j == N - 1
                        { b_carry = b_carry || (sum.full[1] != T::zero()) || (mul.full[1] != T::zero()); }
                    }
                }
            }
            if b_carry
                { res.set_untrustable(); }
        }
        else //if TSIZE <= size_of::<u32>()
        {
            union U<F: Uint>
            {
                wider: u64,
                full: [F; 2],
            }
            let mut mul: U<T> = U { wider: 0 };
            let mut sum: U<T> = U { wider: 0 };
            for i in 0..N
            {
                mul.wider = 0;
                mul.wider = 0;
                for j in 0..N
                {
                    if i + j >= N
                        { break; }
                    unsafe {
                    mul.wider = self.number[j].into_u64().wrapping_mul(rhs.number[i].into_u64()).wrapping_add(mul.full[1].into_u64());
                    sum.wider = res.number[i+j].into_u64().wrapping_add(mul.full[0].into_u64()).wrapping_add(sum.full[1].into_u64());
                    res.number[i+j] = sum.full[0];
                    if i + j == N - 1
                        { b_carry = b_carry || (sum.full[1] != T::zero()) || (mul.full[1] != T::zero()); }
                    }
                }
            }
            if b_carry
                { res.set_untrustable(); }
        }
        res
    }
}

impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
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

