// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::Debug;

/// This struct is for just creating pseudo-random numbers. The basic algorithm
/// is taken from the rand() function of C standard library and tweaked by the
/// author. So, it is light weight. It is designed to be used
/// to plug as `Random_Engine` in the struct `Random_Generic`.
/// It is __for non-cryptographic purpose__. So, normally it is OK to use this
/// struct as `Random_Engine` embedded in the struct `Random_Generic` for
/// pseudo-random number generator. However, __DO NOT USE THIS STRUCT FOR
/// SERIOUS CRYPTOGRAPHIC PURPOSE__ because it does not guarrantee the
/// cryptographic security.
#[allow(non_camel_case_types)]
pub type AnyNumber_Engine_C = AnyNumber_Engine_C_Generic;

/// This struct `AnyNumber_Engine_C_Generic` is for just creating simple
/// pseudo-random numbers. __It is designed to be used to plug as
/// `Random_Engine` in the struct `Random_Generic`.__ It is low chance
/// that you will use this struct `AnyNumber_Engine_C_Generic` directly.
/// You may want to use
/// [`Any_Num_C`](random/type.Any_Num_C.html#type.Any_Num_C)
/// instead if you want to use a pseudo-random number generator rather
/// than to make a new pseudo-random number generator that uses the
/// algoriithm of the rand() function of C standard library.
/// 
/// The basic algorithm is taken from the rand() function of C standard
/// library and tweaked by the author. So, it is light weight. The algoriithm
/// of the rand() function of C standard library is
/// `random number = seed * 1103515245_u64 + 12345_u64` where `random number`
/// is the next random number in random number sequence, and `seed` is the
/// initial random number and the generated `random number` will be fed as
/// `seed` to this formula to get next `random number`. Owing to its addition
/// operation and overflow, this formula produce pseudo-random numbers
/// non-linearly. 
/// 
/// However, this struct `AnyNumber_Engine_C_Generic` is better than the
/// algoriithm of the rand() function of C standard library though it is
/// still cryptographically not secure enough. It is because it
/// has eight sets of 64-bit pseudo-random number sequences and allows
/// `Random_Generic` to randomly choose one of eight sets of the 64-bit
/// pseudo-random number sequences at the very time when it produce a 64-bit
/// pseudo-random number. For extremely simple example, suppose that the first
/// set have the pseudo-random number sequence: 1-2-3-4-5, the second set:
/// 2-4-6-8-10, the third set: 3-6-9-12-15-..., the fourth set:
/// 4-8-12-16-20-..., the fifth set: 5-10-15-20-25-..., the sixth set:
/// 6-12-18-24-30-..., the seventh set: 7-14-21-28-35-..., the eighth set:
/// 8-16-24-32-40-... Then, the actual produced pseudo-random number can be
/// 3-16-18-8-30-... if the first random number is chosen from the first set,
/// and the second random number is chosen from the eighth set, and the third
/// random number is chosen from sixth set, and the fourth random number is
/// chosen from the fourth set, and fifth random number is chosen from the
/// sixth set.
/// 
/// You can change the constant values 1103515245_u64 and 12345_u64 into other
/// numbers by changing the generic constant parameters MULTIPLIER and ADDER
/// if you want to create simple pseudo-random numbers your own way.
/// 
/// # Generic constant parameters
/// - MULTIPLIER: the number to multiply to the previous random number or seed.
/// The default value of `MULTIPLIER` is 1103515245_u64.
/// - ADDER: the number to add to the previous random number or seed multiplied
/// with MULTIPLIER. The default value of `ADDER` is 12345_u64.
/// 
/// It is not guarranteed that your simply changing generic constant parameters
/// generates pseudo-random numbers better than that of C standard library in
/// quality unless you seriously tested your algorithm from the view point of
/// statistics.
/// 
/// It is __for non-cryptographic purpose__. So, normally it is OK to use this
/// struct as `Random_Engine` embedded in the struct `Random_Generic` for
/// pseudo-random number generator. However, __DO NOT USE THIS STRUCT FOR
/// SERIOUS CRYPTOGRAPHIC PURPOSE__ because it does not guarrantee the
/// cryptographic security.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct AnyNumber_Engine_C_Generic<const MULTIPLIER: u64 = 1103515245_u64, const ADDER: u64 = 12345_u64>
{
    any_numbers: [u64; 8],
}

impl<const MULTIPLIER: u64, const ADDER: u64> AnyNumber_Engine_C_Generic<MULTIPLIER, ADDER>
{
    // pub fn new() -> Self
    /// Contructs a new AnyNumber object.
    pub fn new() -> Self
    {
        Self { any_numbers: [0_u64; 8] }
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Changes the direction of random number series so that the period
    /// of the pseudo-random number sequence will be restarted.
    /// 
    /// # Argument
    /// `tangling` is the u64-typed number to tangle the sequence.
    /// If it is `0`, it does not tangle at all.
    pub fn tangle(&mut self, tangling: u64)
    {
        for i in 0..8
        {
            self.any_numbers[i] += tangling;
            self.any_numbers[i] = self.any_numbers[i].wrapping_mul(MULTIPLIER);
            self.any_numbers[i] = self.any_numbers[i].wrapping_add(ADDER);
            self.any_numbers[i] ^= tangling;
        }
    }

    // pub fn get_any_numbers(&self) -> [u64; 8]
    /// Returns the 64-bit pseudo-random unisgned integer array
    /// of pseudo-random numbers.
    #[inline] pub fn get_any_numbers(&self) -> [u64; 8]  { self.any_numbers.clone() }

    // pub fn get_any_number(&self, idx: usize) -> Option<u64>
    /// Returns the pseudo-random 64-bit unsigned integer wrapped by `Some`
    /// of enum `Option`.
    /// 
    /// # Output
    /// When `idx` is greater than `7`, it returns the pseudo-random 64-bit
    /// unsigned integer wrapped by `Some` of enum `Option`.
    #[inline]
    pub fn get_any_number(&self, idx: usize) -> Option<u64>
    {
        if idx < 8
            { Some(self.any_numbers[idx]) }
        else
            { None }
    }

    // pub fn get_any_number_(&self, idx: usize) -> u64
    /// Returns the pseudo-random 64-bit unsigned integer
    /// 
    /// # Output
    /// It returns the pseudo-random 64-bit unsigned integer.
    /// 
    /// # Panics
    /// It will panic when `idx` is greater than `7`.
    #[inline] pub fn get_any_number_(&self, idx: usize) -> u64
    {
        self.any_numbers[idx]
    }

    // pub fn set_any_number(&mut self, idx: usize, val: u64) -> bool
    /// Set a specific random number indicated by `idx` to be `val`.
    /// 
    /// # Output
    /// When `idx` is greater than `7`, it does nothing and returns `false`.
    /// Otherwise, it set a specific random number indicated by `idx` to be
    /// `val` and returns `true`.
    #[inline] pub fn set_any_number(&mut self, idx: usize, val: u64) -> bool
    {
        if idx < 8
        {
            self.any_numbers[idx] = val;
            true
        }
        else
        {
            false
        }
    }

    // pub fn set_any_number_(&mut self, idx: usize, val: u64)
    /// Set a specific random number indicated by `idx` to be `val`.
    /// 
    /// # Panics
    /// It will panic when `idx` is greater than `7`.
    #[inline] pub fn set_any_number_(&mut self, idx: usize, val: u64)
    {
        self.any_numbers[idx] = val;
    }
    
    // pub fn as_mut_ptr(&mut self) -> *mut u8
    /// Returns the pointer to the random number array as *mut u8
    #[inline] pub fn as_mut_ptr(&mut self) -> *mut u8
    {
        self.any_numbers.as_mut_ptr() as *mut u8
    }
}
