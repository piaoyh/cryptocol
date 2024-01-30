// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::Debug;

/// This struct is for just creating pseudo-random numbers. The basic algorithm
/// is taken from the rand() function of C standard library and treaked by the
/// author. So, it is light weight. It is designed to be used as embedded as
/// `Random_Engine` in the struct `Random_Generic`.
/// It is __for non-cryptographic purpose__. So, normally it is OK to use this
/// struct as `Random_Engine` embedded in the struct `Random_Generic` for
/// pseudo-random number generator. However, __DO NOT USE THIS STRUCT FOR
/// SERIOUS CRYPTOGRAPHIC PURPOSE__ because it does not guarrantee the
/// cryptographic security.
#[derive(Debug, Clone)]
pub struct AnyNumber
{
    any_numbers: [u64; 8],
}

impl AnyNumber
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
            self.any_numbers[i] = self.any_numbers[i].wrapping_mul(1103515245);
            self.any_numbers[i] = self.any_numbers[i].wrapping_add(12345);
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
