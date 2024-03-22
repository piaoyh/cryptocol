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
use std::time::{ SystemTime, UNIX_EPOCH };
use std::collections::hash_map::RandomState;
use std::hash::{ BuildHasher, Hasher };
#[cfg(not(target_os = "windows"))] use std::fs::File;
#[cfg(not(target_os = "windows"))] use std::ptr::copy_nonoverlapping;
#[cfg(not(target_os = "windows"))] use std::io::Read;

use crate::number::{ SmallUInt, LongUnion, LongerUnion, BigUInt };
use crate::hash::{ MD4, MD5, SHA0, SHA1, SHA2_256, SHA2_512 };
use super::Random_Engine;
use super::AnyNumber;


/// The struct `Any_MD4` which is a pseudo-random number generator using a hash
/// algorithm MD4. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_MD4 = Random_Generic<MD4, 16383>;  // COUNT = 2^18 / 4 because of hashing 4 times for each random number generation

/// The struct `Any_MD5` which is a pseudo-random number generator using a hash
/// algorithm MD5. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_MD5 = Random_Generic<MD5, 16383>;  // COUNT = 2^18 / 4 because of hashing 4 times for each random number generation

/// The struct `Any_SHA0` which is a pseudo-random number generator using a hash
/// algorithm SHA0. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_SHA0 = Random_Generic<SHA0, 2147483647>;   // COUNT = 2^33 / 4 because of hashing 4 times for each random number generation

/// The struct `Any_SHA1` which is a pseudo-random number generator using a hash
/// algorithm SHA1. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_SHA1 = Random_Generic<SHA1, 4611686018427387903>;  // COUNT = 2^63 / 4 because of hashing 4 times for each random number generation

/// The struct `Any_SHA2_256` which is a pseudo-random number generator using
/// a hash algorithm SHA-2-256. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_SHA2_256 = Random_Generic<SHA2_256, 170141183460469231731687303715884105727>;   // COUNT = 2^128 / 2 because of hashing 2 times for each random number generation

/// The struct `Random_SHA2_512` which is a pseudo-random number generator using
/// a hash algorithm SHA-2-512. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_SHA2_512 = Random_Generic<SHA2_512, 340282366920938463463374607431768211455>;   // COUNT = min(2^256, u128::MAX) because of hashing one time for each random number generation

/// The struct `Random_SHA2_512` which is a pseudo-random number generator using
/// a hash algorithm SHA-2-512. It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Random_SHA2_512 = Random_Generic<SHA2_512, 100>;   // COUNT = 2^256 because of hashing one time for each random number generation

/// The struct `Any_Num` which is a random number generator using an
/// pseudo-random number generator algorithm of rand() of C standard library.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub type Any_Num = Random_Generic<AnyNumber,  2147483647>;   // COUNT = u32::MAX

/// The struct `Random` which is a random number generator and is a synonym of
/// [`Random_SHA2_512`](type@Random_SHA2_512). It means `Random` uses a hash
/// algorithm SHA-2-512. It is cryptographically securer than its counterpart
/// struct `Any` and a bit slower than [`Any`](type@Any).
/// *In the near future, `Random` will be changed to use SHA-3-512 alogrithm
/// and to be a synonym of `Random_SHA3_512` when `Random_SHA3_512` is
/// implemented.
pub type Random = Random_SHA2_512;

/// The struct `Any` which is a random number generator and is a synonym of
/// [`Any_SHA2_256`](type@Any_SHA2_256). It means `Any` uses a hash
/// algorithm SHA-2-256. It is cryptographically less secure than its
/// counterpart struct `Random` and a bit faster than [`Random`](type@Random).
pub type Any = Any_SHA2_256;


/// This generic struct Random_Generic<GenFunc: PRNG + 'static> is the struct
/// for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. It uses hash algorithms to generate pseudo random
/// numbers.
/// 
/// # Feature
/// - The generic parameter `COUNT` should be `1` ~ `u128::MAX` inclusively.
/// - The default value of `COUNT` is `340282366920938463463374607431768211455`
/// which is `u128::MAX` and far less than theoretical period of pseudo-random
/// numbers recursively generated by 512-bit hash algorithms. `COUNT is the
/// limited number of pseudo-random numbers generated from a seed.
/// - After it generates pseudo-random numbers `COUNT` times, it finds a new
/// seed from a system automatically and generates next pseudo-random numbers
/// with a new seed. It means that it uses new seeds every `COUNT` times
/// generation of pseudo-random numbers.
/// - The generic parameter GenFunc is usually one of `SHA3_512`, `SHA2_512`,
/// `SHA1`, `SHA0`, `MD5`, `MD4`, and `AnyNumber`. You can also use
/// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
/// 
/// # Panics
/// If `COUNT` is `0`, the constructor method
/// such as `new()` and `new_with_seeds()` will panic!
/// 
/// # Predetermined Provided Specific `struct`s
/// - Any_Num: uses a pseudo-random number generator algorithm of the function
/// rand() of C standard library.
/// - Any_MD4: uses a hash algorithm MD4.
/// - Any_MD5: uses a hash algorithm MD5.
/// - Any_SHA0: uses a hash algorithm SHA0.
/// - Any_SHA1: uses a hash algorithm SHA1.
/// - Any_SHA2_256: uses a hash algorithm SHA2_256.
/// - Random_SHA2_512: uses a hash algorithm SHA2_512.
/// - Any: uses a hash algorithm SHA2_256.
/// - Random: uses a hash algorithm SHA2_512.
/// 
/// # QUICK START
/// You can use either struct `Any` or `Random` depending on your purpose.
/// `Any` is for normal non-cryptographical purpose while `Random` is for
/// cryptographical purpose if you are fine to use hash algorithm for
/// pseudo-random number generator for cryptographical purpose. Look into
/// the following examples.
/// 
/// ## Example
/// ```
/// use cryptocol::random::Random;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random::new();
/// println!("Random number = {}", rand.random_u128());
/// println!("Random number = {}", rand.random_u64());
/// println!("Random number = {}", rand.random_u32());
/// println!("Random number = {}", rand.random_u16());
/// println!("Random number = {}", rand.random_u8());
/// 
/// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
///     { println!("Random number u64 = {}", num); }
/// 
/// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
///     { println!("Random number u16 = {}", num); }
/// 
/// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
/// if let Some(num) = rand.random_odd_under_uint(1234_u16)
///     { println!("Random odd number u16 = {}", num); }
/// 
/// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
/// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
/// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = rand.random_array();
/// for i in 0..20
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// rand.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = rand.random_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = rand.random_under_biguint(&ceiling)
/// {
///     println!("Random Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = rand.random_under_biguint_(&ceiling);
/// println!("Random Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
/// {
///     println!("Random odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = rand.random_with_msb_set_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// biguint = rand.random_odd_with_msb_set_biguint();
/// println!("512-bit Random Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_using_miller_rabin_biguint(5);
/// println!("Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)]
pub struct Random_Generic<GenFunc: Random_Engine + 'static, const COUNT: u128 = 170141183460469231731687303715884105727>
{
    seed_generator: GenFunc,
    aux_generator: GenFunc,
    count: u128,
    sugar: u64,
}

impl<GenFunc: Random_Engine + 'static, const COUNT: u128> Random_Generic<GenFunc, COUNT>
{
    // pub fn new() -> Self
    /// Constructs a new struct Random_Generic.
    /// 
    /// # Output
    /// It returns a new object of Random_Generic.
    /// 
    /// # Panics
    /// If `COUNT` is `0` or greator than `i32::MAX`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random number = {}", rand.random_u128());
    /// println!("Random number = {}", rand.random_u64());
    /// println!("Random number = {}", rand.random_u32());
    /// println!("Random number = {}", rand.random_u16());
    /// println!("Random number = {}", rand.random_u8());
    /// ```
    pub fn new() -> Self
    {
        if COUNT == 0
            { panic!("COUNT should be greater than 0."); }

        Self
        {
            seed_generator: GenFunc::new_with(&Self::collect_seed()),
            aux_generator: GenFunc::new_with(&Self::collect_seed()),
            count: COUNT,
            sugar: 0,
        }
    }

    // pub fn new_with_seeds<T>(seed: T, aux: T) -> Self
    /// Constructs a new struct Random_Generic with two seeds of type `T` given.
    /// 
    /// # Output
    /// It returns a new object of Random_Generic.
    /// 
    /// # Panics
    /// If `COUNT` is `0` or greator than `i32::MAX`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new_with_seeds(1005458_u64, 1587625_u64);
    /// println!("Random number = {}", rand.random_u128());
    /// println!("Random number = {}", rand.random_u64());
    /// println!("Random number = {}", rand.random_u32());
    /// println!("Random number = {}", rand.random_u16());
    /// println!("Random number = {}", rand.random_u8());
    /// ```
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
        if COUNT == 0
            { panic!("COUNT should be greater than 0."); }

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
            count: COUNT,
            sugar: 0,
        }
    }

/*
    pub fn new_with_generator(seed_generator: GenFunc, aux_generator: GenFunc) -> Self
    {
        let mut res = Self
            {
                seed_generator,
                aux_generator,
                i_seed: Self::COUNT as i32,
                i_aux: Self::COUNT as i32,
            };
        res.seed_generator.absorb_array(&Self::get_seed());
        res.aux_generator.absorb_array(&Self::get_seed());
        res
    }
*/

    // fn collect_seed() -> [u64; 8]
    /// Collects seed from a system.
    /// 
    /// # Output
    /// It returns a random number array `[u64; 8]` made by seeds.
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

/*
    // fn collect_seed_u8() -> u8
    /// Collects seed from a system.
    /// 
    /// # Output
    /// It returns a random number array `[u64; 8]` made by seeds.
    fn collect_seed_u8() -> u8
    {
        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(mut file) = File::open("/dev/random")
            {
                let mut buffer = [0u8; 1];
                if let Ok(_) = file.read(&mut buffer)
                {
                    return buffer[0];
                }
            }
        }
        if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
        {
            let common = LongerUnion::new_with(nanos.as_nanos());
            common.get_ubyte_(0)
        }
        else
        {
            let common = LongUnion::new_with(RandomState::new().build_hasher().finish());
            common.get_ubyte_(0)
        }
    }
*/
    // fn produce_seed(&mut self) -> [u64; 8]
    /// Runs the registered pseudo-random number generator to prepare for
    /// generating a random number for seed.
    /// 
    /// # Output
    /// It returns a random number array `[u64; 8]`.
    fn produce_seed(&mut self) -> [u64; 8]
    {
        self.change_count_and_sugar();
        self.seed_generator.harvest(self.sugar)
    }

    // fn produce_aux(&mut self) -> [u64; 8]
    /// Runs the registered pseudo-random number generator to prepare for
    /// generating a random number for aux.
    /// 
    /// # Output
    /// It returns a random number array `[u64; 8]`.
    fn produce_aux(&mut self) -> [u64; 8] 
    {
        self.change_count_and_sugar();
        self.aux_generator.harvest(self.sugar)
    }

    // fn change_count_and_sugar(&mut self)
    /// Changes`self.count` and `self.sugar` when `self.count` becomes `0`,
    /// changes the seeds for both generators when `self.sugar` becomes `0`.
    /// Otherwise, subracts `1` from `self.count`.
    fn change_count_and_sugar(&mut self)
    {
        if self.sugar == 0
        {
            self.seed_generator.sow_array(&Self::collect_seed());
            self.aux_generator.sow_array(&Self::collect_seed());
        }
        if self.count == 0
        {
            self.sugar = self.sugar.wrapping_add(1);
            self.count = COUNT;
        }
        self.count = self.count.wrapping_sub(1);
    }

    // pub fn random_u8(&mut self) -> u8
    /// Generates random numbers of type `u8`.
    /// 
    /// # Output
    /// It returns a random number of type `u8`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random number = {}", rand.random_u8());
    /// println!("Random number = {}", rand.random_u8());
    /// println!("Random number = {}", rand.random_u8());
    /// println!("Random number = {}", rand.random_u8());
    /// println!("Random number = {}", rand.random_u8());
    /// ```
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

    // pub fn random_u16(&mut self) -> u16
    /// Generates random numbers of type `u16`.
    /// 
    /// # Output
    /// It returns a random number of type `u16`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// println!("Random number = {}", rand.random_u16());
    /// println!("Random number = {}", rand.random_u16());
    /// println!("Random number = {}", rand.random_u16());
    /// println!("Random number = {}", rand.random_u16());
    /// println!("Random number = {}", rand.random_u16());
    /// ```
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

    // pub fn random_u32(&mut self) -> u32
    /// Generates random numbers of type `u32`.
    /// 
    /// # Output
    /// It returns a random number of type `u32`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut rand = Any_MD4::new();
    /// println!("Random number = {}", rand.random_u32());
    /// println!("Random number = {}", rand.random_u32());
    /// println!("Random number = {}", rand.random_u32());
    /// println!("Random number = {}", rand.random_u32());
    /// println!("Random number = {}", rand.random_u32());
    /// ```
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

    // pub fn random_u64(&mut self) -> u64
    /// Generates random numbers of type `u64`.
    /// 
    /// # Output
    /// It returns a random number of type `u64`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut rand = Any_MD5::new();
    /// println!("Random number = {}", rand.random_u64());
    /// println!("Random number = {}", rand.random_u64());
    /// println!("Random number = {}", rand.random_u64());
    /// println!("Random number = {}", rand.random_u64());
    /// println!("Random number = {}", rand.random_u64());
    /// ```
    pub fn random_u64(&mut self) -> u64
    {
        let aux = self.produce_aux();
        let seed = self.produce_seed();
        let mut i = seed[0] as usize & 0b111;
        i = aux[i] as usize & 0b111;
        seed[i]
    }

    // pub fn random_u128(&mut self) -> u128
    /// Generates random numbers of type `u128`.
    /// 
    /// # Output
    /// It returns a random number of type `u128`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut rand = Any_SHA0::new();
    /// println!("Random number = {}", rand.random_u128());
    /// println!("Random number = {}", rand.random_u128());
    /// println!("Random number = {}", rand.random_u128());
    /// println!("Random number = {}", rand.random_u128());
    /// println!("Random number = {}", rand.random_u128());
    /// ```
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

    // pub fn random_usize(&mut self) -> usize
    /// Generates random numbers of type `usize`.
    /// 
    /// # Output
    /// It returns a random number of type `usize`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut rand = Any_SHA1::new();
    /// println!("Random number = {}", rand.random_usize());
    /// println!("Random number = {}", rand.random_usize());
    /// println!("Random number = {}", rand.random_usize());
    /// println!("Random number = {}", rand.random_usize());
    /// println!("Random number = {}", rand.random_usize());
    /// ```
    #[inline]
    pub fn random_usize(&mut self) -> usize
    {
        #[cfg(target_pointer_width = "8")]      return self.random_u8().into_usize();
        #[cfg(target_pointer_width = "16")]     return self.random_u16().into_usize();
        #[cfg(target_pointer_width = "32")]     return self.random_u32().into_usize();
        #[cfg(target_pointer_width = "64")]     return self.random_u64().into_usize();
        #[cfg(target_pointer_width = "128")]    return self.random_u128().into_usize();
    }

    // pub fn random_uint<T>(&mut self) -> T
    /// Generates random numbers of type `T`.
    /// 
    /// # Output
    /// It returns a random number of type `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut rand = Any_SHA2_256::new();
    /// println!("Random number u8 = {}", rand.random_uint::<u8>());
    /// println!("Random number u16 = {}", rand.random_uint::<u16>());
    /// println!("Random number u32 = {}", rand.random_uint::<u32>());
    /// println!("Random number u64 = {}", rand.random_uint::<u64>());
    /// println!("Random number u128 = {}", rand.random_uint::<u128>());
    /// println!("Random number usize = {}", rand.random_uint::<usize>());
    /// ```
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
            1 => T::u8_as_smalluint(self.random_u8()),
            2 => T::u16_as_smalluint(self.random_u16()),
            4 => T::u32_as_smalluint(self.random_u32()),
            8 => T::u64_as_smalluint(self.random_u64()),
            _ => T::u128_as_smalluint(self.random_u128()),
        }
    }

    // pub fn random_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    /// Generates random numbers of type `T` less than `ceiling`.
    /// 
    /// # Output
    /// It returns a random number of type `T` less than `ceiling`
    /// wrapped by enum `Some` of `Option`.
    /// 
    /// # Features
    /// If `ceiling` is `0`, it returns `None`. Otherwise, it returns a
    /// random number of type `T` wrapped by enum `Some` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// if let Some(num) = rand.random_under_uint(12_u8)
    ///     { println!("Random number u8 = {}", num); }
    /// if let Some(num) = rand.random_under_uint(1234_u16)
    ///     { println!("Random number u16 = {}", num); }
    /// if let Some(num) = rand.random_under_uint(12345678_u32)
    ///     { println!("Random number u32 = {}", num); }
    /// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
    ///     { println!("Random number u64 = {}", num); }
    /// if let Some(num) = rand.random_under_uint(12345678901234567890_u128)
    ///     { println!("Random number u128 = {}", num); }
    /// if let Some(num) = rand.random_under_uint(123456789_usize)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number under 0!"); }
    /// ```
    #[inline]
    pub fn random_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        if ceiling != T::zero() { Some(self.random_under_uint_::<T>(ceiling)) } else {None}
    }

    // pub fn random_under_uint_<T>(&mut self, ceiling: T) -> T
    /// Generates random numbers of type `T` less than `ceiling`.
    /// 
    /// # Output
    /// It returns a random number of type `T` less than `ceiling`.
    /// 
    /// # Panics
    /// If `ceiling` is `0`, it will panic.
    /// 
    /// # Caution
    /// Use only when `ceiling` is not `0`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut rand = Any_Num::new();
    /// 
    /// let num = rand.random_under_uint_(12_u8);
    /// println!("Random number u8 = {}", num);
    /// 
    /// let num = rand.random_under_uint_(1234_u16);
    /// println!("Random number u16 = {}", num);
    /// 
    /// let num = rand.random_under_uint_(12345678_u32);
    /// println!("Random number u32 = {}", num);
    /// 
    /// let num = rand.random_under_uint_(1234567890123456_u64);
    /// println!("Random number u64 = {}", num);
    /// 
    /// let num = rand.random_under_uint_(12345678901234567890_u128);
    /// println!("Random number u128 = {}", num);
    /// 
    /// let num = rand.random_under_uint_(123456789_usize);
    /// println!("Random number usize = {}", num);
    /// 
    /// // It will panic.
    /// // let num = rand.random_under_uint_::<usize>(0_usize);
    /// // println!("Random number usize = {}", num);
    /// ```
    #[inline]
    pub fn random_under_uint_<T>(&mut self, ceiling: T) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        self.random_uint::<T>() % ceiling
    }

    // pub fn random_minmax_uint<T>(&mut self, from: T, ceiling: T) -> Option<T>
    /// Generates random numbers of type `T` less than `ceiling` exclusively
    /// and greater than or equal to `from` inclusively.
    /// 
    /// # Output
    /// If `ceiling` is `0` or `from` is greater than or equal to `ceiling`,
    /// it returns a random number of type `T` less than `ceiling` and greater
    /// than or equal to `from`, and the returned random number is wrapped by
    /// enum `Some` of `Option`. Otherwise, it returns enum `None` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut rand = Any_MD4::new();
    /// if let Some(num) = rand.random_minmax_uint(12_u8, 21)
    ///     { println!("Random number u8 = {}", num); }
    /// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
    ///     { println!("Random number u16 = {}", num); }
    /// if let Some(num) = rand.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Random number u32 = {}", num); }
    /// if let Some(num) = rand.random_minmax_uint(1234567890123456_u64, 6543210987654321)
    ///     { println!("Random number u64 = {}", num); }
    /// if let Some(num) = rand.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
    ///     { println!("Random number u128 = {}", num); }
    /// if let Some(num) = rand.random_minmax_uint(123456789_usize, 987654321)
    ///     { println!("Random number usize = {}", num); }
    /// if let Some(num) = rand.random_minmax_uint(10, 8_usize)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    #[inline]
    pub fn random_minmax_uint<T>(&mut self, from: T, ceiling: T) -> Option<T>
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        if ceiling > from { Some(self.random_minmax_uint_(from, ceiling)) } else { None }
    }

    // pub fn random_minmax_uint_<T>(&mut self, from: T, ceiling: T) -> T
    /// Generates random numbers of type `T` less than `ceiling` exclusively
    /// and greater than or equal to `from` inclusively.
    /// 
    /// # Output
    /// It returns a random number of type `T` less than `ceiling`
    /// and greater than or equal to `from`.
    /// 
    /// # Panics
    /// If `ceiling` is `0` or `from` is greater than or equal to `ceiling`,
    /// it will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not `0`
    /// and `from` is less than `ceiling`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut rand = Any_MD5::new();
    /// 
    /// let num = rand.random_minmax_uint_(12_u8, 21);
    /// println!("Random number u8 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(1234_u16, 6321);
    /// println!("Random number u16 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(12345678_u32, 87654321);
    /// println!("Random number u32 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(1234567890123456_u64, 6543210987654321);
    /// println!("Random number u64 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(12345678901234567890_u128, 19876543210987654321);
    /// println!("Random number u128 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(123456789_usize, 987654321);
    /// println!("Random number usize = {}", num);
    /// 
    /// // It will panic!
    /// // let num = rand.random_minmax_uint_(10, 8_usize);
    /// // println!("Random number usize = {}", num);
    /// ```
    #[inline]
    pub fn random_minmax_uint_<T>(&mut self, from: T, ceiling: T) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        self.random_under_uint_(ceiling - from) + from
    }

    // pub fn random_odd_uint<T>(&mut self) -> T
    /// Generates random odd numbers of type `T`.
    /// 
    /// # Output
    /// It returns a random odd number of type `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut rand = Any_SHA0::new();
    /// println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    /// ```
    pub fn random_odd_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_uint::<T>();
        res.set_lsb();
        res
    }

    // pub fn random_odd_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    /// Generates random odd numbers of type `T` less than `ceiling`.
    /// 
    /// # Output
    /// It returns a random odd numbers of type `T` less than `ceiling`
    /// wrapped by enum `Some` of `Option`.
    /// 
    /// # Features
    /// If `ceiling` is `0`, it returns `None`. Otherwise, it returns a
    /// random odd numbers of type `T` wrapped by enum `Some` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut rand = Any_SHA1::new();
    /// if let Some(num) = rand.random_odd_under_uint(12_u8)
    ///     { println!("Random odd number u8 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234_u16)
    ///     { println!("Random odd number u16 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678_u32)
    ///     { println!("Random odd number u32 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Random odd number u64 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Random odd number u128 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(123456789_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(0_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number under 0!"); }
    /// ```
    #[inline]
    pub fn random_odd_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        if ceiling <= T::one() { None } else { Some(self.random_odd_under_uint_(ceiling)) }
    }

    // pub fn random_odd_under_uint_<T>(&mut self, ceiling: T) -> T
    /// Generates random odd numbers of type `T` less than `ceiling`.
    /// 
    /// # Output
    /// It returns a random odd numbers of type `T` less than `ceiling`.
    /// 
    /// # Panics
    /// If `ceiling` is `0`, it will panic.
    /// 
    /// # Caution
    /// Use only when `ceiling` is not `0`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut rand = Any_SHA2_256::new();
    /// 
    /// let num = rand.random_odd_under_uint_(12_u8);
    /// println!("Random odd number u8 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234_u16);
    /// println!("Random odd number u16 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678_u32);
    /// println!("Random odd number u32 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Random odd number u64 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Random odd number u128 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_::<usize>(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// 
    /// // It will panic.
    /// // let num = rand.random_odd_under_uint_::<usize>(0_usize);
    /// // println!("Random odd number usize = {}", num);
    /// ```
    pub fn random_odd_under_uint_<T>(&mut self, ceiling: T) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_under_uint_(ceiling);
        res.set_lsb();
        while res >= ceiling
        {
            res = self.random_under_uint_(ceiling);
            res.set_lsb();
        }
        res
    }

    // pub fn random_with_msb_set_uint<T>(&mut self) -> T
    /// Generates random numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Output
    /// It returns random numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random 8-bit number u8 = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number u32 = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number u64 = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number usize = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    pub fn random_with_msb_set_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_uint::<T>();
        res.set_msb();
        res
    }

    // pub fn random_odd_with_msb_set_uint<T>(&mut self) -> T
    /// Generates random odd numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Output
    /// It returns random odd numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random 8-bit odd number u8 = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number u32 = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number u64 = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number usize = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    pub fn random_odd_with_msb_set_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_uint::<T>();
        res.set_msb();
        res
    }


    // pub fn random_prime_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    /// Returns a random prime number.
    /// 
    /// # Output
    /// A random prime number whose range is from 2 up to T::max() inclusively.
    /// 
    /// # Features
    /// - It uses
    /// [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in notprime number, the
    /// probability that the tested number is not a prime number is 1/4 (= 25%).
    /// So, if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4 = 6.25%).
    /// Therefore, if you test any number 5 times and they all result in a prime
    /// number, the probability that the tested number is not a prime number is
    /// 1/1024 = (= 1/4 * 1/4 * 1/4 * 1/4 * 1/4 = 0.09765625%). In other words,
    /// it is about 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether or not
    /// the generated random number is prime. Usually, `repetition` is given to
    /// be 5 to have 99.9% hit rate.
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
    /// - If you want to use a `(sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method
    /// [random_prime_with_msb_set_using_miller_rabin_uint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_uint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// println!("Random prime number u8 = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random prime number u16 = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random prime number u32 = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random prime number u128 = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random prime number usize = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    pub fn random_prime_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_uint::<T>();
        while !res.is_prime_using_miller_rabin(repetition)
            { res = self.random_odd_uint::<T>(); }
        res
    }

    // pub fn random_prime_with_msb_set_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    /// Returns a full-sized random prime number, which is its MSB (Most
    /// Segnificant Bit) is set `1`.
    /// 
    /// # Output
    /// A full-sized random prime number, which is its MSB (Most Segnificant
    /// Bit) is set `1` and whose range is from 2 up to T::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply sets its MSB
    /// (Most Significant Bit) to be one, and then checks whether or not the
    /// generated random number is prime number, and then it repeats until it
    /// will generate a prime number.
    /// - It uses
    /// [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in notprime number, the
    /// probability that the tested number is not a prime number is 1/4 (= 25%).
    /// So, if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4 = 6.25%).
    /// Therefore, if you test any number 5 times and they all result in a prime
    /// number, the probability that the tested number is not a prime number is
    /// 1/1024 = (= 1/4 * 1/4 * 1/4 * 1/4 * 1/4 = 0.09765625%). In other words,
    /// it is about 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether or not
    /// the generated random number is prime. Usually, `repetition` is given to
    /// be 5 to have 99.9% hit rate.
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
    /// - If you want to use a normal random prime number, you are highly
    /// recommended to use the method
    /// [random_prime_using_miller_rabin_uint()](struct@Random_Generic#method.random_prime_using_miller_rabin_uint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// println!("Random 8-bit prime number u8 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number u16 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number u32 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number u64 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number u128 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_with_msb_set_uint::<T>();
        while !res.is_prime_using_miller_rabin(repetition)
            { res = self.random_odd_with_msb_set_uint::<T>(); }
        res
    }

    // pub fn random_array<T, const N: usize>(&mut self) -> [T; N]
    /// Returns random number array [T; N].
    /// 
    /// # Output
    /// A random number array [T; N] each element of which has a range from
    /// `0` up to `T::max()` inclusively for both ends.
    /// 
    /// # Features
    /// The random numbers that may or may not be cryptographically secure
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
    /// - If you want random BigUInt, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// let num: [u128; 20] = rand.random_array();
    /// for i in 0..20
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
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
        let mut res = [T::zero(); N];
        self.put_random_in_array(&mut res);
        res
    }

    // pub fn put_random_in_array<T, const N: usize>(&mut self, out: &mut [T; N])
    /// Puts random number array [T; N] in `out`.
    /// 
    /// # Argument
    /// `out` is a random number array [T; N] each element of which has
    /// a range from `0` up to `T::max()` inclusively for both ends.
    /// 
    /// # Features
    /// The random numbers that may or may not be cryptographically secure
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
    /// - If you want random BigUInt, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut rand = Any_MD4::new();
    /// let mut num = [0_u64; 32];
    /// rand.put_random_in_array(&mut num);
    /// for i in 0..32
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    pub fn put_random_in_array<T, const N: usize>(&mut self, out: &mut [T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        for i in 0..N
            { out[i] = self.random_uint::<T>(); }
    }

    // pub fn random_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value.
    /// 
    /// # Output
    /// A random number whose range is from `0` up to `BigUInt::max()`
    /// inclusively for both ends.
    /// 
    /// # Features
    /// The random numbers that may or may not be cryptographically secure
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
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_MD5;
    /// 
    /// define_utypes_with!(u128);
    /// let mut rand = Any_MD5::new();
    /// let biguint: U512 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    #[inline]
    pub fn random_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        BigUInt::<T, N>::from_array(self.random_array::<T, N>().clone())
    }

    // pub fn random_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value, wrapped by enum `Some` of `Option`.
    /// 
    /// # Output
    /// A random number wrapped by enum `Some` of `Option`, whose range is
    /// between 0 inclusively and the certain value exclusively when `ceiling`
    /// is not zero. If `ceiling` is zero, `None` will be returned.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    /// the certain value to get its remainder.
    /// - The random numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA0;
    /// 
    /// define_utypes_with!(u64);
    /// let mut rand = Any_SHA0::new();
    /// let ceiling = U1024::max().wrapping_div_uint(3_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    #[inline]
    pub fn random_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        if ceiling.eq_uint(0_u8) {None} else {Some(self.random_under_biguint_::<T, N>(ceiling))}
    }

    // pub fn random_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number whose range is between 0 inclusively
    /// and the certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    /// the certain value to get its remainder.
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Panics
    /// If `ceiling` is zero, this method will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not zero.
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA1;
    /// 
    /// define_utypes_with!(u32);
    /// let mut rand = Any_SHA1::new();
    /// let ceiling = U1024::max().wrapping_div_uint(3_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline]
    pub fn random_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        self.random_biguint::<T, N>().wrapping_rem(ceiling)
    }

    // pub fn random_odd_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
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
    /// - The random numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA2_256;
    /// 
    /// define_utypes_with!(u16);
    /// let mut rand = Any_SHA2_256::new();
    /// let r: U256 = rand.random_odd_biguint();
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn random_odd_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_biguint::<T, N>();
        res.set_lsb();
        res
    }

    // pub fn random_odd_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value, wrapped by enum `Some` of `Option`.
    /// 
    /// # Output
    /// The random odd number whose range is between 0 inclusively and the
    /// certain value exclusively, wrapped by enum `Some` of `Option` if
    /// `ceiling` is not zero. If `ceiling` is zero, `None` will be returned.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    /// by the certain value to get its remainder and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_MD4;
    /// 
    /// define_utypes_with!(u128);
    /// let mut rand = Any_MD4::new();
    /// let ceiling = U1024::max().wrapping_div_uint(4_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn random_odd_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        if ceiling.le_uint(1_u8) {None} else {Some(self.random_odd_under_biguint_(ceiling))}
    }

    // pub fn random_odd_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random odd number whose range is between 0 inclusively and the
    /// certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    /// by the certain value to get its remainder and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Panics
    /// If `ceiling` is zero, this method will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not zero.
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA2_512;
    /// 
    /// define_utypes_with!(u8);
    /// let mut rand = Any_SHA2_512::new();
    /// let ceiling = U1024::max().wrapping_div_uint(3_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn random_odd_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_under_biguint_::<T, N>(ceiling);
        res.set_lsb();
        while res >= *ceiling   // when res is equal to ceiling by one.
        {
            res = self.random_under_biguint_::<T, N>(ceiling);
            res.set_lsb();
        }
        res
    }

    // pub fn random_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value
    /// with `(N * sizeof::<T>() * 8)`-bit length.
    /// 
    /// # Output
    /// The random number whose range is from !(BigUInt::max() >> 1) up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the MSB
    /// (Most Significant Bit).
    /// - The random numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_MD5;
    /// 
    /// define_utypes_with!(u64);
    /// let mut rand = Any_MD5::new();
    /// let biguint: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    #[inline]
    pub fn random_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_biguint::<T, N>();
        res.set_msb();
        res
    }

    // pub fn random_odd_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value with `(N * sizeof::<T>() * 8)`-bit length
    /// 
    /// # Output
    /// The random number that this method random_odd_with_msb_set() returns is
    /// a random odd number whose range is from !(BigUInt::max() >> 1) + 1 up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the MSB
    /// (Most Significant Bit) and LSB (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any;
    /// 
    /// define_utypes_with!(u32);
    /// let mut rand = Any::new();
    /// let num:U512 = rand.random_odd_with_msb_set_biguint();
    /// println!("512-bit Random Odd Number = {}", num);
    /// assert!(num > U512::halfmax());
    /// assert!(num.is_odd());
    /// ```
    pub fn random_odd_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut r = self.random_with_msb_set_biguint();
        r.set_lsb();
        r
    }

    // pub fn random_prime_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
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
    /// - The random prime numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Random;
    /// 
    /// define_utypes_with!(u16);
    /// let mut rand = Random::new();
    /// let num:U512 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random Prime Number = {}", num);
    /// assert!(num.is_odd());
    /// ```
    pub fn random_prime_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_biguint::<T, N>();
        while !res.is_prime_using_miller_rabin(repetition)
            { res = self.random_odd_biguint::<T, N>(); }
        res
    }


    // pub fn random_prime_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number of full-size of BigUInt<T, N>.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
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
    /// - The random prime numbers that may or may not be cryptographically
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
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA1;
    /// 
    /// define_utypes_with!(u16);
    /// let mut rand = Any_SHA1::new();
    /// let num:U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("512-bit Random Prime Number = {}", num);
    /// assert!(num.is_odd());
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut res = self.random_odd_with_msb_set_biguint::<T, N>();
        while !res.is_prime_using_miller_rabin(repetition)
            { res = self.random_odd_with_msb_set_biguint::<T, N>(); }
        res
    }
}
