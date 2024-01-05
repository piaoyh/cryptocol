// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a random number generator struct

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]


use std::fmt::{ Debug, Display };
use std::ops::*;
use std::cmp::{ PartialEq, PartialOrd};
use std::ptr::copy_nonoverlapping;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::collections::hash_map::RandomState;
use std::hash::{ BuildHasher, Hasher };
use std::fs::File;
use std::io::Read;

use crate::number::{small_uint::*, BigUInt};
use crate::number::small_int_unions::*;
use crate::hash::{ MD4, MD5, SHA0, SHA1, SHA2_256, SHA2_512 };
use super::PRNG;


pub type Random_MD4 = Random_Generic<MD4>;
pub type Random_MD5 = Random_Generic<MD5>;
pub type Random_SHA0 = Random_Generic<SHA0>;
pub type Random_SHA1 = Random_Generic<SHA1>;
pub type Random_SHA2_256 = Random_Generic<SHA2_256>;
pub type Random_SHA2_512 = Random_Generic<SHA2_512>;
pub type Random = Random_SHA2_512;
pub type Any = Random_SHA2_256;


pub struct Random_Generic<GenFunc: PRNG + 'static>
{
    seed_generator: GenFunc,
    aux_generator: GenFunc,
    i_seed: i32,
    i_aux: i32,
}

impl<GenFunc: PRNG + 'static> Random_Generic<GenFunc>
{
    const COUNT: i32 = 100;

    pub fn new() -> Self
    {
        Self
        {
            seed_generator: GenFunc::new_with(&Self::collect_seed()),
            aux_generator: GenFunc::new_with(&Self::collect_seed()),
            i_seed: Self::COUNT,
            i_aux: Self::COUNT,
        }
    }

    pub fn new_with_seeds<T>(seed: T, aux: T) -> Self
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        const SIZE: usize = 128;
        let mut s = [T::zero(); SIZE];
        s[1] = T::one();
        s[2] = seed;
        s[3] = aux;
        for i in 4..SIZE
            { s[i] = s[i-1].wrapping_add(s[i-2]); }
        let seed_generator = GenFunc::new_with(&s);

        s[0] = T::one();
        s[1] = T::zero();
        s[2] = aux;
        s[3] = seed;
        for i in 3..SIZE
            { s[i] = !s[i]; }
        let aux_generator = GenFunc::new_with(&s);

        Self
        {
            seed_generator,
            aux_generator,
            i_seed: Self::COUNT,
            i_aux: Self::COUNT,
        }
    }

/*
    pub fn new_with_generator(seed_generator: GenFunc, aux_generator: GenFunc) -> Self
    {
        let mut res = Self
            {
                seed_generator,
                aux_generator,
                i_seed: Self::COUNT,
                i_aux: Self::COUNT,
            };
        res.seed_generator.absorb_array(&Self::get_seed());
        res.aux_generator.absorb_array(&Self::get_seed());
        res
    }
*/

    fn collect_seed() -> [u64; 8]
    {
        let mut seed_buffer = [0_u64; 8];
        let mut read_long = 0_usize;
        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(mut file) = File::open("/dev/random")
            {
                let mut buffer = [0u8; 64];
                if let Ok(n) = file.read(&mut buffer)
                {
                    unsafe { copy_nonoverlapping(buffer.as_ptr(), seed_buffer.as_mut_ptr() as *mut u8, n); }
                    read_long = n >> 3;
                    if (n & 0b111) != 0
                        { read_long += 1; }
                }
            }
        }
        if read_long < 8
        {
            if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
            {
                let common = LongerUnion::new_with(nanos.as_nanos());
                seed_buffer[read_long] = common.get_ulong_(0);
                read_long += 1;
                if read_long < 8
                {
                    seed_buffer[read_long] = common.get_ulong_(1);
                    read_long += 1;
                }
            }
        }
        for i in read_long..8
            { seed_buffer[i] = RandomState::new().build_hasher().finish(); }
        seed_buffer
    }

    fn produce_seed(&mut self) -> [u64; 8]
    {
        if self.i_seed == 0
        {
            self.seed_generator.sow_array(&Self::collect_seed());
            self.i_seed = Self::COUNT;
        }
        self.i_seed -= 1;
        self.seed_generator.harvest()
    }

    fn produce_aux(&mut self) -> [u64; 8] 
    {
        if self.i_aux == 0
        {
            self.aux_generator.sow_array(&Self::collect_seed());
            self.i_aux = Self::COUNT;
        }
        self.i_aux -= 1;
        self.aux_generator.harvest()
    }

    pub fn random_u8(&mut self) -> u8
    {
        let aux = self.produce_aux();
        let seed = self.produce_seed();
        let mut i = seed[0] as usize & 0b111;
        let mut j = seed[1] as usize & 0b111;
        i = aux[i] as usize & 0b111;
        j = aux[j] as usize & 0b111;
        LongUnion::new_with(seed[i]).get_ubyte_(j)
    }

    pub fn random_u16(&mut self) -> u16
    {
        let aux = self.produce_aux();
        let seed = self.produce_seed();
        let mut i = seed[2] as usize & 0b111;
        let mut j = seed[3] as usize & 0b111;
        i = aux[i] as usize & 0b111;
        j = aux[j] as usize & 0b11;
        LongUnion::new_with(seed[i]).get_ushort_(j)
    }

    pub fn random_u32(&mut self) -> u32
    {
        let aux = self.produce_aux();
        let seed = self.produce_seed();
        let mut i = seed[4] as usize & 0b111;
        let mut j = seed[5] as usize & 0b111;
        i = aux[i] as usize & 0b111;
        j = aux[j] as usize & 1;
        LongUnion::new_with(seed[i]).get_uint_(j)
    }

    pub fn random_u64(&mut self) -> u64
    {
        let aux = self.produce_aux();
        let seed = self.produce_seed();
        let mut i = seed[0] as usize & 0b111;
        i = aux[i] as usize & 0b111;
        seed[i]
    }

    pub fn random_u128(&mut self) -> u128
    {
        let aux = self.produce_aux();
        let seed = self.produce_seed();
        let mut i = seed[6] as usize & 0b111;
        let mut j = seed[7] as usize & 0b111;
        i = aux[i] as usize & 0b111;
        j = aux[j] as usize & 0b111;
        let mut res = LongerUnion::new();
        res.set_ulong(0, seed[i]);
        res.set_ulong(1, seed[j]);
        res.get()
    }

    pub fn random_usize(&mut self) -> usize
    {
        #[cfg(target_pointer_width = "8")]      return self.random_u8().into_usize();
        #[cfg(target_pointer_width = "16")]     return self.random_u16().into_usize();
        #[cfg(target_pointer_width = "32")]     return self.random_u32().into_usize();
        #[cfg(target_pointer_width = "64")]     return self.random_u64().into_usize();
        #[cfg(target_pointer_width = "128")]    return self.random_u128().into_usize();
    }

    pub fn random_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        match T::size_in_bytes()
        {
            1 => T::u8_as_SmallUInt(self.random_u8()),
            2 => T::u16_as_SmallUInt(self.random_u16()),
            4 => T::u32_as_SmallUInt(self.random_u32()),
            8 => T::u64_as_SmallUInt(self.random_u64()),
            _ => T::u128_as_SmallUInt(self.random_u128()),
        }
    }

    #[inline]
    pub fn random_minmax<T>(&mut self, from: T, under: T) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        self.random_under(under - from) + from
    }

    #[inline]
    pub fn random_under<T>(&mut self, under: T) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        self.random_uint::<T>() % under
    }

    pub fn random_array<T, const N: usize>(&mut self) -> [T; N]
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let res = [T::zero(); N];
        let mut s = self.produce_seed();
        let lres = N * T::size_in_bytes();
        if lres <= 128
        {
            unsafe { copy_nonoverlapping(s.as_ptr() as *const u8, res.as_ptr() as *mut u8, lres); }
            return res;
        }
        let size = lres / 128;
        for i in 0..size
        {
            unsafe { copy_nonoverlapping(s.as_ptr() as *const u8, res.as_ptr().add(i*128) as *mut u8, 128); }
            s = self.produce_seed();
        }
        let len = lres - 128 * size;
        unsafe { copy_nonoverlapping(s.as_ptr() as *const u8, res.as_ptr().add(size*128) as *mut u8, len); }
        res
    }

    // pub fn random_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value.
    /// 
    /// # Output
    /// The random number that this method returns is a pure random number
    /// whose range is from `0` up to `BigUInt::max()` inclusively for
    /// both ends.
    /// 
    /// # Features
    /// The random numbers that are may or may not be cryptographically secure
    /// depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::random::{ Any, Random };
    /// define_utypes_with!(u128);
    /// println!("Random Number: {}", random_BigUInt::<u128, 8>::random_BigUInt());
    /// ```
    pub fn random_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = BigUInt::<T, N>::new();
        let arr = self.random_array::<T, N>();
        res.set_number(&arr);
        res
    }

    // pub fn random_under_BigUInt<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number that this method random_less_than() returns is
    /// a pure random number whose range is between 0 inclusively
    /// and the certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    /// the certain value to get its remainder.
    /// - The random numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// let r = u1024::random_less_than(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline]
    pub fn random_under_BigUInt<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        self.random_BigUInt::<T, N>().wrapping_rem(ceiling)
    }

    // pub fn random_odd_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value.
    /// 
    /// # Output
    /// The random number that this method `any_odd()` returns is a pure
    /// random odd number whose range is from `1` up to `BigUInt::max()`
    /// inclusively for both ends.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// let r = u1024::any_odd();
    /// println!("Random Odd Number: {}", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn random_odd_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_BigUInt::<T, N>();
        res.set_LSB();
        res
    }

    // pub fn random_odd_under_BigUInt<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number that this method returns is a pure random odd number
    /// whose range is between 0 inclusively and the certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    /// by the certain value to get its remainder and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// let r = u1024::random_odd_less_than(&ceiling);
    /// println!("Random Odd Number less than {} is\n{}", ceiling, u1024::random_odd_less_than(&ceiling));
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn random_odd_under_BigUInt<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_under_BigUInt::<T, N>(ceiling);
        res.set_LSB();
        while res >= *ceiling   // when res is equal to ceiling by one.
        {
            res = self.random_under_BigUInt::<T, N>(ceiling);
            res.set_LSB();
        }
        res
    }

    // pub fn random_with_MSB_set_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value
    /// with `(N * sizeof::<T>() * 8)`-bit length.
    /// 
    /// # Output
    /// The random number that this method returns is a random number whose
    /// range is from !(BigUInt::max() >> 1) up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the MSB
    /// (Most Significant Bit).
    /// - The random numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num = u1024::random_with_MSB_set();
    /// println!("Random Number = {}", u1024::random());
    /// println!("1024-bit Random Number = {}", num);
    /// assert!(num > u1024::submax(1023));
    /// ```
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn random_with_MSB_set_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_BigUInt::<T, N>();
        res.set_MSB();
        res
    }

    // pub fn random_odd_with_MSB_set_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value with `(N * sizeof::<T>() * 8)`-bit length
    /// 
    /// # Output
    /// The random number that this method random_odd_with_MSB_set() returns is
    /// a random odd number whose range is from !(BigUInt::max() >> 1) + 1 up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the MSB
    /// (Most Significant Bit) and LSB (Least Significant Bit).
    /// - The random numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num = u1024::random_odd_with_MSB_set();
    /// println!("Random Number = {}", u1024::random());
    /// println!("1024-bit Random Odd Number = {}", num);
    /// assert!(num > u1024::submax(1023));
    /// assert!(num.is_odd());
    /// ```
    pub fn random_odd_with_MSB_set_BigUInt<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut r = self.random_with_MSB_set_BigUInt();
        r.set_LSB();
        r
    }

    // pub fn random_prime_using_Miller_Rabin_BigUInt<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// 2 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in prime number, the
    /// probability that the tested number is not a prime number is 1/4. So,
    /// if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    /// if you test any number 5 times and they all result in a prime number,
    /// it is 99.9% that the number is a prime number.
    /// - The random prime numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy. 
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_MSB_set_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_with_MSB_set_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    ///
    /// let num = u256::random_prime_using_Miller_Rabin(5);
    /// println!("Random Prime Number = {}", num);
    /// assert!(num.is_prime_using_Miller_Rabin(5));
    /// ```
    pub fn random_prime_using_Miller_Rabin_BigUInt<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_BigUInt::<T, N>();
        while !res.is_prime_using_Miller_Rabin(repetition)
            { res = self.random_odd_BigUInt::<T, N>(); }
        res
    }


    // pub fn random_prime_with_MSB_set_using_Miller_Rabin_BigUInt<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// 2 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply sets its MSB
    /// (Most Significant Bit) to be one, and then checks whether or not the
    /// generated random number is prime number, and then it repeats until it
    /// will generate a prime number.
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in prime number, the
    /// probability that the tested number is not a prime number is 1/4. So,
    /// if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    /// if you test any number 5 times and they all result in a prime number,
    /// it is 99.9% that the number is a prime number.
    /// - The random prime numbers that are may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy. 
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_BigUInt()](struct@Random_Generic#method.random_BigUInt)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_BigUInt()](struct@Random_Generic#method.random_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_BigUInt()](struct@Random_Generic#method.random_odd_BigUInt)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_BigUInt()](struct@Random_Generic#method.ranodm_odd_under_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_MSB_set_BigUInt()](struct@Random_Generic#method.random_odd_with_MSB_set_BigUInt)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_Miller_Rabin_BigUInt()](struct@Random_Generic#method.random_prime_using_Miller_Rabin_BigUInt)
    /// rather than this method.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    ///
    /// let num = u256::random_prime_using_Miller_Rabin(5);
    /// println!("Random Prime Number = {}", num);
    /// assert!(num.is_prime_using_Miller_Rabin(5));
    /// ```
    pub fn random_prime_with_MSB_set_using_Miller_Rabin_BigUInt<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_with_MSB_set_BigUInt::<T, N>();
        while !res.is_prime_using_Miller_Rabin(repetition)
            { res = self.random_odd_with_MSB_set_BigUInt::<T, N>(); }
        res
    }
}

