// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various pseudo-random number generators
//! 
//! # Introduction
//! The module that contains a few sub-modules to define various pseudo-random
//! number generators
//! 
//! # Background: Random number generators
//! Generating true random numbers is very difficult. However, if artificial
//! random numbers which are widely called 'pseudo-random numbers' has the
//! same statistical characterisics as the true random numbers, it is
//! considered to be virtually random. For more in detail about randomness,
//! [Read more](https://en.wikipedia.org/wiki/Statistical_randomness).
//! 
//! # Predefined pseudo-random number generators
//! There is name consistancy. For the names of pseudo-random number generators
//! in this module, `Any` indicates cryptographically insecure while `Random`
//! indicates cryptographically secure.
//! 
//! There are provided predefined pseudo-random number generators:
//! - Any: is a synonym of Any_SHA2_256 at the moment and can be __silently
//!     changed__ to have better algorithm in the future. If you want to keep
//!     using SHA2_256 for a pseudo-random number generator, you may want to
//!     use Any_SHA2_256. If you are happy that you will automatically use the
//!     better algotrithm in the future, you may want to use `Any`.
//!     Read [here](random/random/type.Any.html#type.Any).
//! - Random: is a synonym of Random_SHA2_512 at the moment and can be
//!     __silently changed__ to have better algorithm in the future. If you want
//!     to keep using SHA2_512 for a pseudo-random number generator, you may
//!     want to use Random_SHA2_512. If you are happy that you will automatically
//!     use the better algotrithm in the future, you may want to use `Random`.
//!     Read [here](random/random/type.Random.html#type.Random).
//! - Any_Num: is a synonym of Any_Num_C at the moment and can be __silently
//!     changed__ to have better algorithm in the future. If you want to keep
//!     using the algorithm of C standard libraray for a pseudo-random number
//!     generator, you may want to use Any_Num_C. If you are happy that you
//!     will automatically use the better algotrithm in the future, you may
//!     want to use `Any_Num`.
//!     Read [here](random/random/type.Any_Num.html#type.Any_Num).
//! - Any_Num_C: uses a pseudo-random number generator algorithm of the
//!     function rand() of C standard library at the moment.
//!     Read [here](random/type.Any_Num_C.html#type.Any_Num_C).
//! - Any_MD4: uses a hash algorithm MD4. Read [here](random/random/type.Any_MD4.html#type.Any_MD4).
//! - Any_MD5: uses a hash algorithm MD5. Read [here](random/random/type.Any_MD5.html#type.Any_MD5),
//! - Any_SHA0: uses a hash algorithm SHA0. Read [here](random/random/type.Any_SHA0.html#type.Any_SHA0),
//! - Any_SHA1: uses a hash algorithm SHA1. Read [here](random/random/type.Any_SHA1.html#type.Any_SHA1),
//! - Any_SHA2_256: uses a hash algorithm SHA2_256. Read [here](random/random/type.Any_SHA2_256.html#type.Any_SHA2_256)
//! - Any_SHA2_512: uses a hash algorithm SHA2_512. Read [here](random/random/type.Any_SHA2_512.html#type.Any_SHA2_512), and
//! - Random_SHA2_512: uses a hash algorithm SHA2_512. Read [here](random/random/type.Random_SHA2_512.html#type.Random_SHA2_512).
// ! - Any_DES: uses a symmetric-key encryption algorithm DES algorithm DES.
// ! - Any_NDES: uses a symmetric-key encryption algorithm NDES algorithm NDES.
// ! - Any_AES: uses a symmetric-key encryption algorithm AES algorithm AES.
// ! - Any_NAES: uses a symmetric-key encryption algorithm NAES algorithm NAES.
// ! - Random_AES: uses a symmetric-key encryption algorithm AES algorithm AES.
//! 
//! # Quality Issues and Debate
//! The pseudo-random number generators in this module use hash algorithms,
//! encrytion/decryption algorithms, etc. which are not originally designed
//! for pseudo-random number generator. At the Internet, you can find a lot of
//! research results in terms of the possibility to use hash algorithms and/or
//! encryption algorithms for a pseudo-random number generator. This module can
//! also be considered to be a part of the research.
//! 
//! Some people doubt the cryptographical security of the pseudo-random number
//! generator using hash algorithm and/or encryption algorithm though the
//! offical hash algorithms published by NIST such as SHA-3 and SHA-2 are known
//! to have passed all the statistical and cryptographical security tests,
//! which have been done from 2006 to 2015 for SHA-3 and from 2001 to 2012 for
//! SHA-2. The tests included collision attack, preimage attack, and
//! second-preimage attack. It means that a pseudo-random number generator that
//! uses a hash algorithm has the long enough period of its recursively[^note]
//! produced random numbers for most of the cases. According to
//! [security collision chart](https://en.wikipedia.org/wiki/SHA-3#Comparison_of_SHA_functions),
//! the security stregths against collision of SHA-3-512, SHA-3-256, SHA-2-512
//! and SHA-2-256 are 256 bits, 128 bits, 256 bits, and 128 bits, respectively,
//! because of
//! [birthday paradox or birthday problem](https://en.wikipedia.org/wiki/Birthday_problem).
//! So, the period of pseudo-random numbers which is hash values generated by
//! hash algorithms can be theoretically
//! '115792089237316195423570985008687907853269984665640564039457584007913129639935',
//! which is 2^256, for 512-bit hash values of SHA-3-512 and SHA-2-512, and
//! '340282366920938463463374607431768211455', which is 2^128, for 256-bit hash
//! values of SHA-3-256 and SHA-2-256. So, __for non-cryptographical
//! purposes,__ all the pseudo-random number generators in this module are
//! completely fine to use.
//! 
//! However, __for serious cryptographical purpose,__ it is still
//! debatable.
//! So, if you really want one of the best quality pseudo-random number
//! generator rather than this module for serious cryptographical purpose,
//! you are encouraged to use the crate
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! which is well known to be a good pseudo-random number generator for
//! _cryptographical_ security purpose. The module of implementation of
//! `Random_Generic<GenFunc: PRNG + 'static>` to use
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! as a pseudo-random number generator is not implemented in this crate in
//! order to keep small number of dependencies, but how to embed
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! in a module of `Random_Generic<GenFunc: PRNG + 'static>`to use
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! is shown below in the section 'HOW TO EMBED OsRng IN THIS MODULE' in order
//! to help you implement a module to use
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! as a pseudo-random number generator in your project by yourself. 
//! 
//! # QUICK START
//! You can use either struct `Any` or `Random` depending on your purpose.
//! `Any` is for normal non-cryptographical purpose while `Random` is for
//! cryptographical purpose if you are fine to use hash algorithm for
//! pseudo-random number generator for cryptographical purpose.
//! 
//! - For `Random_Generic`, read [here](struct@Random_Generic#quick-start).
//! - For `Any`, read [here](random/random/type.Any.html#type.Any).
//! - For `Random`, read [here](random/type.Random.html#type.Random).
//! - For `Any_Num`, read [here](random/type.Any_Num.html#type.Any_Num).
//! - For `Any_Num_C`, read [here](random/type.Any_Num_C.html#type.Any_Num_C).
// ! - For `AnyNumber_C_Generic`, read [here](any_number_c_generic/struct.AnyNumber_C_Generic.html#struct.AnyNumber_C_Generic).
//! - For `Any_MD4`, read [here](random/type.Any_MD4.html#type.Any_MD4).
//! - For `Any_MD5`, read [here](random/type.Any_MD5.html#type.Any_MD5),
//! - For `Any_SHA0`, read [here](random/type.Any_SHA0.html#type.Any_SHA0),
//! - For `Any_SHA1`, read [here](random/type.Any_SHA1.html#type.Any_SHA1),
//! - For `Any_SHA2_256`, read [here](random/type.Any_SHA2_256.html#type.Any_SHA2_256)
//! - For `Any_SHA2_512`, read [here](random/type.Any_SHA2_512.html#type.Any_SHA2_512), and
//! - For `Random_SHA2_512`, read [here](random/type.Random_SHA2_512.html#type.Random_SHA2_512).
// ! - For `Any_DES`, read [head]
// ! - For `Any_NDES`, read [head]
// ! - For `Any_AES`, read [head]
// ! - For `Any_NAES`, read [head]
// ! - For `Random_AES`, read [head]
//! 
//! # How to embed OsRng in this module
//! This is a simple illustration to embed OsRng in this module. It is assumed
//! that you will have `main.rs` and `os_rng.rs`.
//! 
//! First, you have to include additional dependencies in your Cargo.toml
//! as following example.
//! 
//! ## Example 3
//! ```
//! [dependencies]
//! cryptocol = "0.8.0"
//! rand = { version = "0.8", features = ["getrandom"] }
//! ```
//! It is good if you keep the version number of each crate as latest version.
//! 
//! Second, you need to make a new empty rust source file in proper folder.
//! Let's say the empty rust source file to be `os_rng.rs` and to be located
//! in the same folder where main.rs is located. In the file `os_rng.rs`,
//! you have to import some `struct`s as following example.
//! 
//! ## Example 4
//! ```
//! use std::ops::*;
//! use std::fmt::{ Display, Debug };
//! use rand::{ rngs, RngCore };
//! use cryptocol::number::SmallUInt;
//! use cryptocol::random::{ Random_Engine, Random_Generic };
//! ```
//! 
//! Third, you have to make an empty struct `OsRng` in `os_rng.rs`
//! as following example.
//! 
//! ## Example 5
//! ```
//! pub struct OsRng;
//! ```
//! 
//! Fourth, you are supposed to make implementation of trait Random_Engine
//! for the empty struct `OsRng` in `os_rng.rs` as following example.
//! 
//! ## Example 6
//! ```
//! impl Random_Engine for OsRng
//! {
//!     #[inline]
//!     fn new() -> Self    { Self }
//! 
//!     #[inline]
//!     fn new_with<T, const N: usize>(_: &[T; N]) -> Self
//!     where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//!         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
//!         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
//!         + Rem<Output=T> + RemAssign
//!         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
//!         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
//!         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
//!         + PartialEq + PartialOrd
//!     { Self::new() }
//! 
//!     #[inline]
//!     fn sow_array<T, const N: usize>(&mut self, _: &[T; N])
//!     where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//!         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
//!         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
//!         + Rem<Output=T> + RemAssign
//!         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
//!         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
//!         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
//!         + PartialEq + PartialOrd
//!     {}
//! 
//!     #[inline]
//!     fn harvest(&mut self, _: u64) -> [u64; 8]
//!     {
//!         [rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
//!         rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
//!         rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
//!         rngs::OsRng.next_u64(), rngs::OsRng.next_u64()]
//!     }
//! }
//! ```
//! 
//! Fifth, you can define user-defined data type for your convenience
//! in `os_rng.rs` as following example.
//! 
//! ## Example 7
//! ```
//! pub type Random_OsRng = Random_Generic<OsRng>;
//! ```
//! 
//! If you correctly follow the above-instructions, your `os_rng.rs` will
//! look like as Example 8.
//! 
//! ## Example 8 (os_rng.rs)
//! ```
//! use std::ops::*;
//! use std::fmt::{ Display, Debug };
//! use rand::{ rngs, RngCore };
//! use cryptocol::number::SmallUInt;
//! use cryptocol::random::{ Random_Engine, Random_Generic };
//! 
//! pub struct OsRng;
//! 
//! impl Random_Engine for OsRng
//! {
//!     #[inline]
//!     fn new() -> Self    { Self }
//! 
//!     #[inline]
//!     fn new_with<T, const N: usize>(_: &[T; N]) -> Self
//!     where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//!         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
//!         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
//!         + Rem<Output=T> + RemAssign
//!         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
//!         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
//!         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
//!         + PartialEq + PartialOrd
//!     { Self::new() }
//! 
//!     #[inline]
//!     fn sow_array<T, const N: usize>(&mut self, _: &[T; N])
//!     where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//!         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
//!         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
//!         + Rem<Output=T> + RemAssign
//!         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
//!         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
//!         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
//!         + PartialEq + PartialOrd
//!     {}
//! 
//!     #[inline]
//!     fn harvest(&mut self, _: u64) -> [u64; 8]
//!     {
//!         [   rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
//!             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
//!             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
//!             rngs::OsRng.next_u64(), rngs::OsRng.next_u64()  ]
//!     }
//! }
//! 
//! pub type Random_OsRng = Random_Generic<OsRng>;
//! ```
//! 
//! Now, you are very ready to use `Random_OsRng` in your own project. And,
//! all the methods of Random_OsRng has been automagically implemented and
//! you can use them for free. In other source files of your project,
//! you are supposed to import `Random_OsRng`.
//! 
//! In the following example, it is assumed that `os_rng.rs` is placed in
//! the same folder where `main.rs` is located. The following example shows
//! how to use `Random_OsRng` in your `main.rs`.
//! 
//! ## Example 9 (main.rs)
//! ```
//! use super::trait_impl_for_OsRng::Random_OsRng;
//! 
//! let mut r = Random_OsRng::new();
//! println!("Random_OsRng u8 = {}", r.random_u8());
//! println!("Random_OsRng u16 = {}", r.random_u16());
//! println!("Random_OsRng u32 = {}", r.random_u32());
//! println!("Random_OsRng u64 = {}", r.random_u64());
//! println!("Random_OsRng u128 = {}", r.random_u128());
//! println!("Random_OsRng under 123456789 = {}", r.random_under_uint_(123456789_u64));
//! println!("Random_OsRng prime number = {}", r.random_prime_using_Miller_Rabin_uint::<u128>(5));
//! println!("Random_OsRng BigUInt = {}", r.random_BigUInt::<u64, 8>());
//! println!("Random_OsRng odd BigUInt = {}", r.random_odd_BigUInt::<u64, 8>());
//! println!("Random_OsRng BigUInt prime number = {}", r.random_prime_using_Miller_Rabin_BigUInt::<u64, 8>(5));
//! ```
//! 
//! Now, you are ready to embed OsRng in this module and use it in any kind of
//! your projects.
//! 
//! For `Random_Generic`, read [here](struct@Random_Generic).
//! 
//! [^note]: Here, 'recursively' means that the output hash value of a hash
//! function is fed back to the hash function as its message, and a new hash
//! value is gotten from it, and then the new hash value is fed back to the
//! hash function as its message again, and this process is repeated.
//! 



pub mod random;

/// The module that contains struct AnyMumber_C_Generic
pub mod any_number_engine_c_generic;

/// The module that contains trait Random_Engine
pub mod trait_random_engine;

/// The module that contains implementation of trait Random_Engine for MD4
pub mod trait_impl_for_md4;

/// The module that contains implementation of trait Random_Engine for MD5
pub mod trait_impl_for_md5;

/// The module that contains implementation of trait Random_Engine for SHA1
pub mod trait_impl_for_sha1;

/// The module that contains implementation of trait Random_Engine for SHA2_256
pub mod trait_impl_for_sha2_256;

/// The module that contains implementation of trait Random_Engine for SHA2_512
pub mod trait_impl_for_sha2_512;

/// The module that contains implementation of trait Random_Engine for AnyNumber
pub mod trait_impl_for_any_number;

pub use random::*;
pub use trait_random_engine::*;
pub use any_number_engine_c_generic::{ AnyNumber_Engine_C_Generic, AnyNumber_Engine_C };
