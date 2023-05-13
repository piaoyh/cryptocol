//#![feature(generic_const_exprs)]
// Struct BigUInt: the base struct of all cryptography class for Little Endian
// Version:				0.8
// Author:				PARK Youngho (2022-05-24)
// Next Modifier(s):    PARK Youngho (2022-05-24)
//                      Somebody Else (YYYY-MM-DD)

use std::mem::size_of;
use std::f64::consts;
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
where T: Uint
{
    number: [T; N],
    flag: u8,
}

impl<T, const N: usize> Large_Integer<T, N> for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    fn new() -> Self    { Self { number: [T::zero(); N], flag: 0, } }

    fn from_array(val: &[T; N]) -> Self
    {
        let mut s = Self::new();
        for n in 0..N
            { s.number[n] = val[n]; }
        s
    }

    fn from_uint<V: Copy>(val: V) -> Self
    {
        union U<VV: Copy, TT: Copy>
        {
            ulonger: ULonger,
            v: VV,
            t: TT,
        }

        let mut s = Self::new();
        let mut tu: U<V, T> = U { ulonger: ULonger::new() };
        tu.v = val;
        if size_of::<T>() >= size_of::<V>()
        {
            unsafe { s.set_num(0, tu.t); }
        }
        else
        {
            for i in 0..(size_of::<V>()/size_of::<T>())
            {
                unsafe {
                    s.set_num(i, tu.t);
                    tu.ulonger.ulonger >>= size_of::<T>() * 8;
                }
            }
        }
        return s;
    }

    fn from_string_with_radix(txt: &str, radix: usize) -> Option<Self>
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

    fn to_string_with_radix(&self, radix: usize) -> String
    {
        let mut txt = String::new();
        let zero = Self::new();
        let divisor = Self::from_uint(radix);
        let mut dividend = *self;
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
        let mut numStr = String::new();
        while let Some(ch) = txt.pop()
            { numStr.push(ch); }
        numStr
    }

    fn add_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.accumulate(rhs);
        bi
    }

    fn sub_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.dissipate(rhs);
        bi
    }

    fn mul_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.times(rhs);
        bi
    }

    fn div_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.quotient(rhs);
        bi
    }

    fn rem_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.remainder(rhs);
        bi
    }

    fn get_num(&self, i: usize) -> T    { self.number[i] }
    fn set_num(&mut self, i: usize, val: T) { self.number[i] = val; }
    fn set_zero(&mut self)  { for it in &mut self.number { *it = T::zero(); } }
    fn set_max(&mut self)   { for it in &mut self.number { *it = T::Max(); } }
    fn set_flag_bit(&mut self, flag_bits: u8)   { self.flag |= flag_bits }
    fn reset_flag_bit(&mut self, flag_bits: u8) { self.flag &= !flag_bits }
    fn is_flag_bit_on(&self, flag_bits: u8) -> bool { (self.flag & flag_bits) != 0 }
}

impl<T, const N: usize> Add for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self
    {
        let mut res = Self::new();
        let mut b_carry = false;
        if size_of::<T>() == size_of::<u_max>()
        {
            union U<V: Uint>
            {
                full: V,
                half: [u_half; 2],
            }

            for i in 0..N*2
            {
                let mut mul: U<T> = U { full: T::zero() };
                let mut sum: U<T> = U { full: T::zero() };
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
        else if size_of::<T>() == size_of::<u64>()
        {
            union U<F: Uint>
            {
                wider: u128,
                full: [F; 2],
            }

            for i in 0..N
            {
                let mut mul: U<T> = U { wider: 0 };
                let mut sum: U<T> = U { wider: 0 };
                for j in 0..N
                {
                    if i + j >= N
                        { break; }
                    unsafe { 
                    mul.wider = self.number[j].into_u128() * rhs.number[i].into_u128() + mul.full[1].into_u128();
                    sum.wider = res.number[i+j].into_u128() + mul.full[0].into_u128() + sum.full[1].into_u128();
                    res.number[i+j] = sum.full[0];
                    if i + j == N - 1
                        { b_carry = b_carry || (sum.full[1] != T::zero()) || (mul.full[1] != T::zero()); }
                    }
                }
            }
            if b_carry
                { res.set_untrustable(); }
        }
        else //if size_of::<T>() <= size_of::<u32>()
        {
            union U<F: Uint>
            {
                wider: u64,
                full: [F; 2],
            }

            for i in 0..N
            {
                let mut mul: U<T> = U { wider: 0 };
                let mut sum: U<T> = U { wider: 0 };
                for j in 0..N
                {
                    if i + j >= N
                        { break; }
                    unsafe {
                    mul.wider = self.number[j].into_u64() * rhs.number[i].into_u64() + mul.full[1].into_u64();
                    sum.wider = res.number[i+j].into_u64() + mul.full[0].into_u64() + sum.full[1].into_u64();
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl<T, const N: usize> Div for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self
    {
        let mut quotient = Self::new();
        if rhs == Self::new()
        {
            quotient.set_divided_by_zero();
            quotient.set_max();
            return quotient;
        }

        let mut dividend = self.clone();
        while dividend >= rhs
        {
            let mut subquotient = Self::from_uint(1);
            let mut subdividend = dividend >> 1;
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl<T, const N: usize> Rem for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
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
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    fn rem_assign(&mut self, rhs: Self)
    {
        if rhs == Self::new()
        {
            self.set_divided_by_zero();
            self.set_zero();
            return;
        }
        while *self >= rhs
        {
            let mut	accumulation = rhs.clone();
            let mut newAcc = *self >> 1;
            while accumulation <= newAcc
                { accumulation <<= 1; }
            *self -= accumulation;
        }
    }
}

impl<T, const N: usize> Shl<i64> for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    type Output = Self;
    fn shl(self, rhs: i64) -> Self
    {
        let mut s = self.clone();
        s <<= rhs;
        s
    }
}

impl<T, const N: usize> ShlAssign<i64> for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    fn shl_assign(&mut self, rhs: i64)
    {
        if rhs < 0
        {
		    *self >>= -rhs;
            return;
        }
        let chunk_num = rhs as usize / (8 * size_of::<T>()) as usize;
        let piece_num = rhs as usize % (8 * size_of::<T>()) as usize;

        if chunk_num > 0
        {
            let mut i = N - 1;
            let mut j = N - 1 - chunk_num;
            while i >= chunk_num
            {
                self.number[i] = self.number[j];
                i-=1;
                j-=1;
            }
            for idx in 0..chunk_num
                { self.number[idx] = T::zero(); }
        }
        let mut num: T;
        let mut carry = T::zero();
        for idx in 0..N
        {
            num = (self.number[idx] << piece_num) | carry;
            carry = self.number[idx] >> (8 * size_of::<T>() - piece_num);
            self.number[idx] = num;
        }
    }
}

impl<T, const N: usize> Shr<i64> for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    type Output = Self;
    fn shr(self, rhs: i64) -> Self
    {
        let mut s = self.clone();
        s >>= rhs;
        s
    }
}

impl<T, const N: usize> ShrAssign<i64> for BigUInt<T, N>
where T: Uint + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + Shl<usize, Output = T> + Shr<usize, Output = T>
        + BitAndAssign + BitOr<Output = T> + BitOrAssign + BitXorAssign + Not<Output = T>
{
    fn shr_assign(&mut self, rhs: i64)
    {
        if rhs < 0
        {
		    *self <<= -rhs;
            return;
        }
        let chunk_num = rhs as usize / (8 * size_of::<T>()) as usize;
        let piece_num = rhs as usize % (8 * size_of::<T>()) as usize;

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

            let mut j = chunk_num;
            for i in 0..(N-chunk_num)
            {
                self.number[i] = self.number[j];
                j += 1;
            }
            for i in (N-chunk_num)..N
                { self.number[i] = T::zero(); }
        }
        let mut num: T;
        let mut carry = T::zero();
        let mut idx = N - 1;
        loop
        {
            num = (self.number[idx] >> piece_num) | carry;
            carry = self.number[idx] << (8 * size_of::<T>() - piece_num);
            self.number[idx] = num;
            if (idx == 0)
                { break; }
            idx -= 1;
        }
    }
}

impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: Uint + BitAndAssign
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
where T: Uint + BitAndAssign
{
    fn bitand_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] &= rhs.number[idx]; }
    }
}

impl<T, const N: usize> BitOr for BigUInt<T, N>
where T: Uint + BitOrAssign
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
where T: Uint + BitOrAssign
{
    fn bitor_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] |= rhs.number[idx]; }
    }
}

impl<T, const N: usize> BitXor for BigUInt<T, N>
where T: Uint + BitXorAssign
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
where T: Uint + BitXorAssign
{
    fn bitxor_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] ^= rhs.number[idx]; }
    }
}

impl<T, const N: usize> Not for BigUInt<T, N>
where T: Uint + Not<Output = T>
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
where T: Uint
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
where T: Uint
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

