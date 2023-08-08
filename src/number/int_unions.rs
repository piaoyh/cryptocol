// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic types of primitive unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __These unions are for segmentation.__

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ Debug, Display };
use std::mem::{ size_of, size_of_val };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::uint::*;


#[derive(Copy, Clone)]
pub union ShortUnion
{
    pub this: u16,
    pub that: i16,
    pub ushort: u16,
    pub sshort: i16,
    pub ubyte: [u8; 2],
    pub sbyte: [i8; 2],
    #[cfg(target_pointer_width = "16")] pub u_size: usize,
    #[cfg(target_pointer_width = "16")] pub s_size: isize,
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 2],
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 2],
}


#[derive(Copy, Clone)]
pub union IntUnion
{
    pub this: u32,
    pub that: i32,
    pub uint: u32,
    pub sint: i32,
    pub ushort: [u16; 2],
    pub sshort: [i16; 2],
    pub ubyte: [u8; 4],
    pub sbyte: [i8; 4],
    #[cfg(target_pointer_width = "32")] pub u_size: usize,
    #[cfg(target_pointer_width = "32")] pub s_size: isize,
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 2],
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 2],
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 4],
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 4],
}

#[derive(Copy, Clone)]
pub union LongUnion
{
    pub this: u64,
    pub that: i64,
    pub ulong: u64,
    pub slong: i64,
    pub uint: [u32; 2],
    pub sint: [i32; 2],
    pub ushort: [u16; 4],
    pub sshort: [i16; 4],
    pub ubyte: [u8; 8],
    pub sbyte: [i8; 8],
    #[cfg(target_pointer_width = "64")] pub u_size: usize,
    #[cfg(target_pointer_width = "64")] pub s_size: isize,
    #[cfg(target_pointer_width = "32")] pub u_size: [usize; 2],
    #[cfg(target_pointer_width = "32")] pub s_size: [uisize; 2],
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 4],
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 4],
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 8],
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 8],
}

#[derive(Copy, Clone)]
pub union LongerUnion
{
    pub this: u128,
    pub that: i128,
    pub ulonger: u128,
    pub slonger: i128,
    pub ulong: [u64; 2],
    pub slong: [i64; 2],
    pub uint: [u32; 4],
    pub sint: [i32; 4],
    pub ushort: [u16; 8],
    pub sshort: [i16; 8],
    pub ubyte: [u8; 16],
    pub sbyte: [i8; 16],
    #[cfg(target_pointer_width = "128")] pub u_size: usize,
    #[cfg(target_pointer_width = "128")] pub s_size: isize,
    #[cfg(target_pointer_width = "64")] pub u_size: [usize; 2],
    #[cfg(target_pointer_width = "64")] pub s_size: [isize; 2],
    #[cfg(target_pointer_width = "32")] pub u_size: [usize; 4],
    #[cfg(target_pointer_width = "32")] pub s_size: [isize; 4],
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 8],
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 8],
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 16],
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 16],
}


#[cfg(target_pointer_width = "128")]
#[derive(Copy, Clone)]
pub union SizeUnion
{
    pub this: usize,
    pub that: isize,
    pub u_size: usize,
    pub s_size: isize,
    pub ulonger: u128,
    pub slonger: i128,
    pub ulong: [u64; 2],
    pub slong: [i64; 2],
    pub uint: [u32; 4],
    pub sint: [i32; 4],
    pub ushort: [u16; 8],
    pub sshort: [i16; 8],
    pub ubyte: [u8; 16],
    pub sbyte: [i8; 16],
}

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub union SizeUnion
{
    pub this: usize,
    pub that: isize,
    pub u_size: usize,
    pub s_size: isize,
    pub ulong: u64,
    pub slong: i64,
    pub uint: [u32; 2],
    pub sint: [i32; 2],
    pub ushort: [u16; 4],
    pub sshort: [i16; 4],
    pub ubyte: [u8; 8],
    pub sbyte: [i8; 8],
}

#[cfg(target_pointer_width = "32")]
#[derive(Copy, Clone)]
pub union SizeUnion
{
    pub this: usize,
    pub that: isize,
    pub u_size: usize,
    pub s_size: isize,
    pub uint: u32,
    pub sint: i32,
    pub ushort: [u16; 2],
    pub sshort: [i16; 2],
    pub ubyte: [u8; 4],
    pub sbyte: [i8; 4],
}

#[cfg(target_pointer_width = "16")]
#[derive(Copy, Clone)]
pub union SizeUnion
{
    pub this: usize,
    pub that: isize,
    pub u_size: usize,
    pub s_size: isize,
    pub ushort: u16,
    pub sshort: i16,
    pub ubyte: [u8; 2],
    pub sbyte: [i8; 2],
}

#[cfg(target_pointer_width = "8")]
#[derive(Copy, Clone)]
pub union SizeUnion
{
    pub this: usize,
    pub that: isize,
    pub u_size: usize,
    pub s_size: isize,
    pub ubyte: u8,
    pub sbyte: i8,
}



macro_rules! get_set_byte {
    ($f:expr) => {
        const N: usize = $f;

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8 { unsafe { self.ubyte[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8 { unsafe { self.ubyte[Self::N-i] } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8 { unsafe { self.sbyte[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8 { unsafe { self.sbyte[Self::N-i] } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.ubyte[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.ubyte[Self::N-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
                { unsafe { Some(self.sbyte[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
                { unsafe { Some(self.sbyte[Self::N-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)  { unsafe { self.ubyte[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)  { unsafe { self.ubyte[Self::N-i] = val; } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)  { unsafe { self.sbyte[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)  { unsafe { self.sbyte[Self::N-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_ubyte(&mut self, i: usize, val: u8) -> bool
        {
            if i < Self::N
            { 
                unsafe { self.ubyte[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ubyte(&self, i: usize, val: u8) -> bool
        {
            if i < Self::N
            { 
                unsafe { self.ubyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "little")]
        pub fn set_sbyte(&mut self, i: usize, val: i8) -> bool
        {
            if i < Self::N
            { 
                unsafe { self.sbyte[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_sbyte(&self, i: usize, val: i8) -> bool
        {
            if i < Self::N
            { 
                unsafe { self.sbyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_short {
    ($f:expr) => {
        const M: usize = $f;

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[Self::M-i] } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16 { unsafe { self.sshort[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16 { unsafe { self.sshort[Self::M-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[Self::M-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
                { unsafe { Some(self.sshort[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
                { unsafe { Some(self.sshort[Self::M-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[Self::M-i] = val; } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)  { unsafe { self.sshort[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)  { unsafe { self.sshort[Self::M-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_ushort(&mut self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.ushort[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ushort(&self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.ushort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "little")]
        pub fn set_sshort(&mut self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.sshort[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_sshort(&self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.sshort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_int {
    ($f:expr) => {
        const L: usize = $f;

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[Self::L-i] } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sint_(&self, i: usize) -> i32 { unsafe { self.sint[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sint_(&self, i: usize) -> i32 { unsafe { self.sint[Self::L-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[Self::L-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
                { unsafe { Some(self.sint[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
                { unsafe { Some(self.sint[Self::L-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[Self::L-i] = val; } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)  { unsafe { self.sint[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)  { unsafe { self.sint[Self::L-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_uint(&mut self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.uint[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_uint(&self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.uint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "little")]
        pub fn set_sint(&mut self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.sint[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_sint(&self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.sint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_long {
    ($f:expr) => {
        const K: usize = $f;

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[Self::K-i] } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_slong_(&self, i: usize) -> i64 { unsafe { self.slong[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_slong_(&self, i: usize) -> i64 { unsafe { self.slong[Self::K-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[Self::K-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
                { unsafe { Some(self.slong[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
                { unsafe { Some(self.slong[Self::K-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[Self::K-i] = val; } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64)  { unsafe { self.slong[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64)  { unsafe { self.slong[Self::K-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_ulong(&mut self, i: usize, val: u64) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.ulong[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ulong(&self, i: usize, val: u64) -> bool
        {
            if i <= Self::K
            { 
                unsafe { self.ulong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "little")]
        pub fn set_slong(&mut self, i: usize, val: i64) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.slong[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ulong(&self, i: usize, val: i64) -> bool
        {
            if i <= Self::K
            { 
                unsafe { self.slong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_size {
    ($f:expr) => {
        const J: usize = $f;

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.u_size[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.u_size[Self::J-i] } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize { unsafe { self.s_size[i] } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize { unsafe { self.s_size[Self::J-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.u_size[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.u_size[Self::J-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
                { unsafe { Some(self.s_size[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
                { unsafe { Some(self.s_size[Self::J-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.u_size[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.u_size[Self::J-i] = val; } }

        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize)  { unsafe { self.s_size[i] = val; } }

        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize)  { unsafe { self.s_size[Self::J-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_usize(&mut self, i: usize, val: usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.u_size[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ssize(&self, i: usize, val: usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.s_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! integer_union_methods {
    ($f:ty) => {
        pub fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
        {
            let (r_this, c1) = self.get().overflowing_add(rhs.get());
            let cc = if carry { 1 as $f } else { 0 as $f };
            let (res_this, c2) = r_this.overflowing_add(cc);
            let res = Self::new_with(res_this);
            (res, c1 || c2)
        }

        #[inline] pub fn wrapping_add(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_add(rhs.get()) ) }
        #[inline] pub fn wrapping_add_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_add(rhs.get())); }

        pub fn overflowing_add(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_add(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_add(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_add(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn unchecked_add(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_add(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_add(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_add(rhs.get()) ) }

        pub fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
        {
            let (r_this, b1) = self.get().overflowing_sub(rhs.get());
            let (res_this, b2) = r_this.overflowing_sub(borrow as $f);
            (Self::new_with(res_this), b1 || b2)
        }

        #[inline] pub fn wrapping_sub(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_sub(rhs.get()) ) }
        #[inline] pub fn wrapping_sub_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_sub(rhs.get())); }
        
        pub fn overflowing_sub(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, borrow) = self.get().overflowing_sub(rhs.get());
            (Self::new_with(res_this), borrow)
        }

        pub fn checked_sub(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_sub(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }
        
        #[inline] pub fn unchecked_sub(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_sub(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_sub(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_sub(rhs.get()) ) }

        #[inline] pub fn wrapping_mul(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_mul(rhs.get()) ) }
        #[inline] pub fn wrapping_mul_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_mul(rhs.get())); }
        
        pub fn overflowing_mul(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_mul(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_mul(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_mul(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn unchecked_mul(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_mul(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_mul(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_mul(rhs.get()) ) }

        #[inline] pub fn wrapping_div(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_div(rhs.get()) ) }
        #[inline] pub fn wrapping_div_assign(&mut self, rhs: Self)  { self.this = self.get().wrapping_div(rhs.get()); }
        
        pub fn overflowing_div(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_div(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_div(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_div(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn saturating_div(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_div(rhs.get()) ) }

        #[inline] pub fn wrapping_rem(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_rem(rhs.get()) ) }
        #[inline] pub fn wrapping_rem_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_rem(rhs.get())); }

        pub fn overflowing_rem(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_rem(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_rem(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_rem(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn wrapping_pow(self, exp: u32) -> Self   { Self::new_with( self.get().wrapping_pow(exp) ) }
        
        pub fn checked_pow(self, exp: u32) -> Option<Self>
        {
            match self.get().checked_pow(exp)
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        pub fn overflowing_pow(self, exp: u32) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_pow(exp);
            (Self::new_with(res_this), carry)
        }

        #[inline] pub fn saturating_pow(self, exp: u32) -> Self { Self::new_with( self.get().saturating_pow(exp) ) }


        #[inline] pub fn abs_diff(self, other: Self) -> Self    { Self::new_with( self.get().abs_diff(other.get()) ) }

        #[inline] pub fn pow(self, exp: u32) -> Self    { Self::new_with( self.get().pow(exp) ) }

        #[inline] pub fn ilog(self, base: Self) -> u32  { self.get().ilog(base.get()) }
        #[inline] pub fn ilog10(self) -> u32            { self.get().ilog10() }
        #[inline] pub fn ilog2(self) -> u32             { self.get().ilog2() }

        #[inline] pub fn reverse_bits(self) -> Self     { Self::new_with( self.get().reverse_bits() ) }

        #[inline] pub fn rotate_left(self, n: u32) -> Self  { Self::new_with( self.get().rotate_left(n) ) }
        #[inline] pub fn rotate_right(self, n: u32) -> Self { Self::new_with( self.get().rotate_right(n) ) }

        #[inline] pub fn count_ones(self) -> u32        { self.get().count_ones() }
        #[inline] pub fn count_zeros(self) -> u32       { self.get().count_zeros() }
        #[inline] pub fn leading_ones(self) -> u32      { self.get().leading_ones() }
        #[inline] pub fn leading_zeros(self) -> u32     { self.get().leading_zeros() }
        #[inline] pub fn trailing_ones(self) -> u32     { self.get().trailing_ones() }
        #[inline] pub fn trailing_zeros(self) -> u32    { self.get().trailing_zeros() }

        #[inline] pub fn from_be(x: Self) -> Self   { Self::new_with( <$f>::from_be(x.get()) ) }
        #[inline] pub fn from_le(x: Self) -> Self   { Self::new_with( <$f>::from_le(x.get()) ) }
        #[inline] pub fn to_be(self) -> Self        { Self::new_with( self.get().to_be() ) }
        #[inline] pub fn to_le(self) -> Self        { Self::new_with( self.get().to_le() ) }
        #[inline] pub fn swap_bytes(self) -> Self   { Self::new_with( self.get().swap_bytes() ) }

        #[inline] pub fn is_power_of_two(self) -> bool    { self.get().is_power_of_two() }
        #[inline] pub fn next_power_of_two(self) -> Self  { Self::new_with( self.get().next_power_of_two() ) }

        #[inline] pub fn into_f64(self) -> f64      { self.get() as f64 }
        #[inline] pub fn into_f32(self) -> f32      { self.get() as f32 }
        #[inline] pub fn into_u128(self) -> u128    { self.get() as u128 }
        #[inline] pub fn into_u64(self) -> u64      { self.get() as u64 }
        #[inline] pub fn into_u32(self) -> u32      { self.get() as u32 }
        #[inline] pub fn into_u16(self) -> u16      { self.get() as u16 }
        #[inline] pub fn into_u8(self) -> u8        { self.get() as u8 }
        #[inline] pub fn into_usize(self) -> usize  { self.get() as usize }
        #[inline] pub fn into_bool(self) -> bool    { self.get() != 0 }
        #[inline] pub fn zero() -> Self             { Self::new_with(0 as $f) }
        #[inline] pub fn one() -> Self              { Self::new_with(1 as $f) }
        #[inline] pub fn max() -> Self              { Self::new_with(<$f>::MAX) }
        #[inline] pub fn min() -> Self              { Self::new_with(<$f>::MIN) }
        #[inline] pub fn num(n: u128) -> Self       { Self::new_with(n as $f) }
        #[inline] pub fn size_in_bytes() -> usize   { size_of::<Self>() }
        #[inline] pub fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
        #[inline] pub fn length_in_bytes(self) -> usize    { size_of_val(&self) }
        #[inline] pub fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
        #[inline] pub fn is_odd(self) -> bool       { (self.get() & 1) != 0 }
    }
}



impl ShortUnion
{
    pub fn new() -> Self                    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self    { Self { ushort } }
    pub fn new_with_singed(sshort: i16) -> Self { Self { sshort } }
    pub fn onoff(b: bool) -> Self           { Self { ushort: b as u16 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { sshort: b as i16 } }

    #[inline] pub fn get(self) -> u16                 { unsafe { self.ushort } }
    #[inline] pub fn get_signed(self) -> i16          { unsafe { self.sshort } }
    #[inline] pub fn set(&mut self, val: u16)         { self.ushort = val; }
    #[inline] pub fn set_signed(&mut self, val: i16)  { self.sshort = val; }
    get_set_byte!(2-1);

    #[cfg(target_pointer_width = "8")]      get_set_usize!(2-1);

    integer_union_methods!(u16);
}



impl IntUnion
{
    pub fn new() -> Self                { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self  { Self { uint } }
    pub fn new_with_singed(sint: i32) -> Self   { Self { sint } }
    pub fn onoff(b: bool) -> Self       { Self { uint: b as u32 } }
    pub fn onoff_singed(b: bool) -> Self    { Self { sint: b as i32 } }

    #[inline] pub fn get(self) -> u32             { unsafe { self.uint } }
    #[inline] pub fn get_singed(self) -> i32      { unsafe { self.sint } }
    #[inline] pub fn set(&mut self, val: u32)     { self.uint = val; }
    #[inline] pub fn set_singed(&mut self, val: i32)     { self.sint = val; }
    get_set_byte!(4-1);
    get_set_short!(2-1);

    #[cfg(target_pointer_width = "16")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(4-1);

    integer_union_methods!(u32);
}



impl LongUnion
{
    pub fn new() -> Self                    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self     { Self { ulong } }
    pub fn new_with_singed(slong: i64) -> Self  { Self { slong } }
    pub fn onoff(b: bool) -> Self           { Self { ulong: b as u64 } }
    pub fn onoff_singed(b: bool) -> Self    { Self { slong: b as i64 } }

    #[inline] pub fn get(self) -> u64           { unsafe { self.ulong } }
    #[inline] pub fn get_singed(self) -> i64    { unsafe { self.slong } }
    #[inline] pub fn set(&mut self, val: u64)   { self.ulong = val; }
    #[inline] pub fn set_singed(&mut self, val: i64)    { self.slong = val; }
    get_set_byte!(8-1);
    get_set_short!(4-1);
    get_set_int!(2-1);

    #[cfg(target_pointer_width = "32")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "16")]     get_set_usize!(4-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(8-1);

    integer_union_methods!(u64);
}



impl LongerUnion
{
    pub fn new() -> Self                    { Self { ulonger: 0 } }
    pub fn new_with(ulonger: u128) -> Self  { Self { ulonger } }
    pub fn new_with_singed(slonger: i128) -> Self   { Self { slonger } }
    pub fn onoff(b: bool) -> Self           { Self { ulonger: b as u128 } }
    pub fn onoff_singed(b: bool) -> Self    { Self { slonger: b as i128 } }

    #[inline] pub fn get(self) -> u128          { unsafe { self.ulonger } }
    #[inline] pub fn get_singed(self) -> i128   { unsafe { self.slonger } }
    #[inline] pub fn set(&mut self, val: u128)  { self.ulonger = val; }
    #[inline] pub fn set_singed(&mut self, val: i128)    { self.slonger = val; }
    get_set_byte!(16-1);
    get_set_short!(8-1);
    get_set_int!(4-1);
    get_set_long!(2-1);

    #[cfg(target_pointer_width = "64")]     get_set_size!(2-1);
    #[cfg(target_pointer_width = "32")]     get_set_size!(4-1);
    #[cfg(target_pointer_width = "16")]     get_set_size!(8-1);
    #[cfg(target_pointer_width = "8")]      get_set_size!(16-1);

    integer_union_methods!(u128);
}



impl SizeUnion
{
    pub fn new() -> Self                    { Self { u_size: 0 } }
    pub fn new_with(u_size: usize) -> Self  { Self { u_size } }
    pub fn new_with_singed(s_size: isize) -> Self   { Self { s_size } }
    pub fn onoff(b: bool) -> Self           { Self { u_size: b as usize } }
    pub fn onoff_singed(b: bool) -> Self    { Self { s_size: b as isize } }

    #[inline] pub fn get(self) -> usize         { unsafe { self.u_size } }
    #[inline] pub fn get_singed(self) -> isize  { unsafe { self.s_size } }
    #[inline] pub fn set(&mut self, val: usize) { self.u_size = val; }
    #[inline] pub fn set_singed(&mut self, val: isize)   { self.s_size = val; }
    #[cfg(target_pointer_width = "128")]    get_set_byte!(16-1);
    #[cfg(target_pointer_width = "64")]     get_set_byte!(8-1);
    #[cfg(target_pointer_width = "32")]     get_set_byte!(4-1);
    #[cfg(target_pointer_width = "16")]     get_set_byte!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_short!(8-1);
    #[cfg(target_pointer_width = "64")]     get_set_short!(4-1);
    #[cfg(target_pointer_width = "32")]     get_set_short!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_int!(4-1);
    #[cfg(target_pointer_width = "64")]     get_set_int!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_long!(2-1);

    integer_union_methods!(usize);
}




macro_rules! Uint_for_integer_unions_impl {
    ($f:ty, $g:ty) => {
        impl Uint for $f
        {
            fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
            {
                let (r1, c1) = self.overflowing_add(rhs);
                let (r2, c2) = r1.overflowing_add(Self::onoff(carry));
                (r2, c1 || c2)
            }

            #[inline] fn wrapping_add(self, rhs: Self) -> Self              { self.wrapping_add(rhs) }
            #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool)   { self.overflowing_add(rhs) }
            #[inline] fn checked_add(self, rhs: Self) -> Option<Self>       { self.checked_add(rhs) }
            #[inline] fn unchecked_add(self, rhs: Self) -> Self             { self.checked_add(rhs).unwrap() }
            #[inline] fn saturating_add(self, rhs: Self) -> Self            { self.saturating_add(rhs) }

            fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
            {
                let (r1, b1) = self.overflowing_sub(rhs);
                let (r2, b2) = r1.overflowing_sub(Self::onoff(borrow));
                (r2, b1 || b2)
            }

            #[inline] fn wrapping_sub(self, rhs: Self) -> Self              { self.wrapping_sub(rhs) }
            #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool)   { self.overflowing_sub(rhs) }
            #[inline] fn checked_sub(self, rhs: Self) -> Option<Self>       { self.checked_sub(rhs) }
            #[inline] fn unchecked_sub(self, rhs: Self) -> Self             { self.checked_sub(rhs).unwrap() }
            #[inline] fn saturating_sub(self, rhs: Self) -> Self            { self.saturating_sub(rhs) }

            #[inline] fn wrapping_mul(self, rhs: Self) -> Self              { self.wrapping_mul(rhs) }
            #[inline] fn overflowing_mul(self, rhs: Self) -> (Self, bool)   { self.overflowing_mul(rhs) }
            #[inline] fn checked_mul(self, rhs: Self) -> Option<Self>       { self.checked_mul(rhs) }
            #[inline] fn unchecked_mul(self, rhs: Self) -> Self             { self.checked_mul(rhs).unwrap() }
            #[inline] fn saturating_mul(self, rhs: Self) -> Self            { self.saturating_mul(rhs) }

            #[inline] fn wrapping_div(self, rhs: Self) -> Self              { self.wrapping_div(rhs) }
            #[inline] fn overflowing_div(self, rhs: Self) -> (Self, bool)   { self.overflowing_div(rhs) }
            #[inline] fn checked_div(self, rhs: Self) -> Option<Self>       { self.checked_div(rhs) }
            #[inline] fn saturating_div(self, rhs: Self) -> Self            { self.saturating_div(rhs) }

            #[inline] fn wrapping_rem(self, rhs: Self) -> Self              { self.wrapping_rem(rhs) }
            #[inline] fn overflowing_rem(self, rhs: Self) -> (Self, bool)   { self.overflowing_rem(rhs) }
            #[inline] fn checked_rem(self, rhs: Self) -> Option<Self>       { self.checked_rem(rhs) }

            #[inline] fn wrapping_pow(self, exp: u32) -> Self               { self.wrapping_pow(exp) }
            #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool)    { self.overflowing_pow(exp) }
            #[inline] fn checked_pow(self, exp: u32) -> Option<Self>        { self.checked_pow(exp) }
            #[inline] fn saturating_pow(self, exp: u32) -> Self             { self.saturating_pow(exp) }


            #[inline] fn abs_diff(self, other: Self) -> Self    { self.abs_diff(other) }

            #[inline] fn pow(self, exp: u32) -> Self    { self.pow(exp) }
            #[inline] fn ilog(self, base: Self) -> u32  { self.ilog(base) }
            #[inline] fn ilog10(self) -> u32            { self.ilog10() }
            #[inline] fn ilog2(self) -> u32             { self.ilog2() }

            #[inline] fn reverse_bits(self) -> Self     { self.reverse_bits() }

            #[inline] fn rotate_left(self, n: u32) -> Self  { self.rotate_left(n) }
            #[inline] fn rotate_right(self, n: u32) -> Self { self.rotate_right(n) }

            #[inline] fn count_ones(self) -> u32        { self.count_ones() }
            #[inline] fn count_zeros(self) -> u32       { self.count_zeros() }
            #[inline] fn leading_ones(self) -> u32      { self.leading_ones() }
            #[inline] fn leading_zeros(self) -> u32     { self.leading_zeros() }
            #[inline] fn trailing_ones(self) -> u32     { self.trailing_ones() }
            #[inline] fn trailing_zeros(self) -> u32    { self.trailing_zeros() }

            #[inline] fn from_be(x: Self) -> Self   { Self::from_be(x) }
            #[inline] fn from_le(x: Self) -> Self   { Self::from_le(x) }
            #[inline] fn to_be(self) -> Self        { self.to_be() }
            #[inline] fn to_le(self) -> Self        { self.to_le() }
            #[inline] fn swap_bytes(self) -> Self   { self.swap_bytes() }

            #[inline] fn is_power_of_two(self) -> bool      { self.is_power_of_two() }
            #[inline] fn next_power_of_two(self) -> Self    { self.next_power_of_two() }

            #[inline] fn into_f64(self) -> f64      { unsafe { self.this as f64 } }
            #[inline] fn into_f32(self) -> f32      { unsafe { self.this as f32 } }
            #[inline] fn into_u128(self) -> u128    { unsafe { self.this as u128 } }
            #[inline] fn into_u64(self) -> u64      { unsafe { self.this as u64 } }
            #[inline] fn into_u32(self) -> u32      { unsafe { self.this as u32 } }
            #[inline] fn into_u16(self) -> u16      { unsafe { self.this as u16 } }
            #[inline] fn into_u8(self) -> u8        { unsafe { self.this as u8 } }
            #[inline] fn into_usize(self) -> usize  { unsafe { self.this as usize } }
            #[inline] fn into_bool(self) -> bool    { unsafe { self.this != 0 } }
            #[inline] fn zero() -> Self             { Self::new_with(0) }
            #[inline] fn one() -> Self              { Self::new_with(1) }
            #[inline] fn max() -> Self              { Self::new_with(<$g>::MAX) }
            #[inline] fn min() -> Self              { Self::new_with(<$g>::MIN) }
            #[inline] fn num(n: u128) -> Self       { Self::new_with(n as $g) }
            #[inline] fn size_in_bytes() -> usize   { size_of::<Self>() }
            #[inline] fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
            #[inline] fn length_in_bytes(self) -> usize    { size_of_val(&self) }
            #[inline] fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
            #[inline] fn is_odd(self) -> bool      { unsafe { (self.this & 1) != 0 } }
        }
    }
}



macro_rules! operators_for_integer_unions_impl {
    ($f:ty) => {
        impl Add for $f
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self
            {
                self.wrapping_add(rhs)
            }
        }

        impl AddAssign for $f
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self)
            {
                self.wrapping_add_assign(rhs);
            }
        }

        impl Sub for $f
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self
            {
                self.wrapping_sub(rhs)
            }
        }

        impl SubAssign for $f
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self)
            {
                self.wrapping_sub_assign(rhs);
            }
        }

        impl Mul for $f
        {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self
            {
                self.wrapping_mul(rhs)
            }
        }

        impl MulAssign for $f
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self)
            {
                self.wrapping_mul_assign(rhs);
            }
        }

        impl Div for $f
        {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self
            {
                self.wrapping_div(rhs)
            }
        }

        impl DivAssign for $f
        {
            #[inline]
            fn div_assign(&mut self, rhs: Self)
            {
                self.wrapping_div_assign(rhs);
            }
        }

        impl Rem for $f
        {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self
            {
                self.wrapping_rem(rhs)
            }
        }

        impl RemAssign for $f
        {
            #[inline]
            fn rem_assign(&mut self, rhs: Self)
            {
                self.wrapping_rem_assign(rhs);
            }
        }

        impl BitAnd for $f
        {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s &= rhs;
                s
            }
        }

        impl BitAndAssign for $f
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self)
            {
                unsafe { self.this &= rhs.this; }
            }
        }

        impl BitOr for $f
        {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s |= rhs;
                s
            }
        }

        impl BitOrAssign for $f
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self)
            {
                unsafe { self.this |= rhs.this; }
            }
        }

        impl BitXor for $f
        {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s ^= rhs;
                s
            }
        }

        impl BitXorAssign for $f
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self)
            {
                unsafe { self.this ^= rhs.this; }
            }
        }

        impl Not for $f
        {
            type Output = Self;

            #[inline]
            fn not(self) -> Self
            {
                Self::new_with(! unsafe { self.this })
            }
        }

        impl PartialEq for $f
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool
            {
                unsafe { self.this == other.this }
            }
        }

        impl PartialOrd for $f
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering>
            {
                if unsafe { self.this > other.this }
                    { return Some(Ordering::Greater); }
                else if unsafe { self.this < other.this }
                    { return Some(Ordering::Less); }
                else
                    { Some(Ordering::Equal) }
            }
        }
    }
}



macro_rules! shift_ops_for_integer_unions_impl {
    ($u:ty, $f:ty) => {
        impl Shl<$f> for $u
        {
            type Output = Self;

            fn shl(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s <<= rhs;
                s
            }
        }

        impl ShlAssign<$f> for $u
        {
            #[inline]
            fn shl_assign(&mut self, rhs: $f)
            {
                unsafe { self.this <<= rhs }
            }
        }

        impl Shr<$f> for $u
        {
            type Output = Self;

            fn shr(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s >>= rhs;
                s
            }
        }

        impl ShrAssign<$f> for $u
        {
            #[inline]
            fn shr_assign(&mut self, rhs: $f)
            {
                unsafe { self.this >>= rhs }
            }
        }
    }
}



Uint_for_integer_unions_impl! { ShortUnion, u16 }
Uint_for_integer_unions_impl! { IntUnion, u32 }
Uint_for_integer_unions_impl! { LongUnion, u64 }
Uint_for_integer_unions_impl! { LongerUnion, u128 }
Uint_for_integer_unions_impl! { SizeUnion, usize }

operators_for_integer_unions_impl! { ShortUnion }
operators_for_integer_unions_impl! { IntUnion }
operators_for_integer_unions_impl! { LongUnion }
operators_for_integer_unions_impl! { LongerUnion }

shift_ops_for_integer_unions_impl! { ShortUnion, i8 }
shift_ops_for_integer_unions_impl! { ShortUnion, i16 }
shift_ops_for_integer_unions_impl! { ShortUnion, i32 }
shift_ops_for_integer_unions_impl! { ShortUnion, i64 }
shift_ops_for_integer_unions_impl! { ShortUnion, i128 }
shift_ops_for_integer_unions_impl! { ShortUnion, isize }

shift_ops_for_integer_unions_impl! { ShortUnion, u8 }
shift_ops_for_integer_unions_impl! { ShortUnion, u16 }
shift_ops_for_integer_unions_impl! { ShortUnion, u32 }
shift_ops_for_integer_unions_impl! { ShortUnion, u64 }
shift_ops_for_integer_unions_impl! { ShortUnion, u128 }
shift_ops_for_integer_unions_impl! { ShortUnion, usize }

shift_ops_for_integer_unions_impl! { IntUnion, i8 }
shift_ops_for_integer_unions_impl! { IntUnion, i16 }
shift_ops_for_integer_unions_impl! { IntUnion, i32 }
shift_ops_for_integer_unions_impl! { IntUnion, i64 }
shift_ops_for_integer_unions_impl! { IntUnion, i128 }
shift_ops_for_integer_unions_impl! { IntUnion, isize }

shift_ops_for_integer_unions_impl! { IntUnion, u8 }
shift_ops_for_integer_unions_impl! { IntUnion, u16 }
shift_ops_for_integer_unions_impl! { IntUnion, u32 }
shift_ops_for_integer_unions_impl! { IntUnion, u64 }
shift_ops_for_integer_unions_impl! { IntUnion, u128 }
shift_ops_for_integer_unions_impl! { IntUnion, usize }

shift_ops_for_integer_unions_impl! { LongUnion, i8 }
shift_ops_for_integer_unions_impl! { LongUnion, i16 }
shift_ops_for_integer_unions_impl! { LongUnion, i32 }
shift_ops_for_integer_unions_impl! { LongUnion, i64 }
shift_ops_for_integer_unions_impl! { LongUnion, i128 }
shift_ops_for_integer_unions_impl! { LongUnion, isize }

shift_ops_for_integer_unions_impl! { LongUnion, u8 }
shift_ops_for_integer_unions_impl! { LongUnion, u16 }
shift_ops_for_integer_unions_impl! { LongUnion, u32 }
shift_ops_for_integer_unions_impl! { LongUnion, u64 }
shift_ops_for_integer_unions_impl! { LongUnion, u128 }
shift_ops_for_integer_unions_impl! { LongUnion, usize }

shift_ops_for_integer_unions_impl! { LongerUnion, i8 }
shift_ops_for_integer_unions_impl! { LongerUnion, i16 }
shift_ops_for_integer_unions_impl! { LongerUnion, i32 }
shift_ops_for_integer_unions_impl! { LongerUnion, i64 }
shift_ops_for_integer_unions_impl! { LongerUnion, i128 }
shift_ops_for_integer_unions_impl! { LongerUnion, isize }

shift_ops_for_integer_unions_impl! { LongerUnion, u8 }
shift_ops_for_integer_unions_impl! { LongerUnion, u16 }
shift_ops_for_integer_unions_impl! { LongerUnion, u32 }
shift_ops_for_integer_unions_impl! { LongerUnion, u64 }
shift_ops_for_integer_unions_impl! { LongerUnion, u128 }
shift_ops_for_integer_unions_impl! { LongerUnion, usize }




/// union array for transforming from one type into anther type
pub union Share<D, S>
where D: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd,
      S: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    pub des: D,
    pub src: S,
}

impl<D, S> Share<D, S>
where D: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd,
      S: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    pub fn new() -> Self
    {
        if size_of::<D>() >= size_of::<S>()
            { Self { des: D::zero() } }
        else
            { Self { src: S::zero() } }
    }

    pub fn from_src(src: S) -> Self
    {
        let mut me = Share::<D, S>::new();
        unsafe { me.src = src; }
        me
    }

    #[cfg(target_endian = "little")]
    pub fn into_des(&mut self, pos: usize) -> Option<D>
    {
        let bit_pos = pos * size_of::<D>() * 8;
        unsafe { self.src >>= S::num(bit_pos as u128); }
        if (bit_pos > 0) && self.is_src_zero()
            { None }
        else
            { unsafe { Some(self.des) } }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des1(&mut self, pos: usize) -> Option<D>
    {
        let des_size = size_of::<D>();
        let src_size = size_of::<S>();
        let bit_pos = pos * size_of::<D>() * 8;
        unsafe { self.src <<= S::num(bit_pos as u128); }
        if des_size > src_size
            { unsafe { self.des >>= D::num((des_size - src_size).into_u128() * 8); } }
        else if src_size > des_size
            { unsafe { self.src <<= S::num((src_size - des_size).into_u128() * 8); } }
        Some( unsafe { self.des } )
    }

    pub fn is_src_zero(&self) -> bool    { unsafe { self.src == S::zero() } }
}

/// union array for transforming from one type into anther type
pub union Common<D, const N: usize, S, const M: usize>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + Rem<Output=S> + RemAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub des: [D; N],
    pub src: [S; M],
}

impl<D, const N: usize, S, const M: usize> Common<D, N, S, M>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub fn new() -> Self
    {
        if Self::size_of_des() >= Self::size_of_src()
            { Self { des: [D::zero(); N] } }
        else
            { Self { src: [S::zero(); M] } }
    }

    pub fn from_src(src: &[S; M]) -> Self
    {
        let mut me = Common::<D, N, S, M>::new();
        unsafe { me.src.copy_from_slice(src); }
        me
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub fn into_des(&mut self, des: &mut [D; N])
    {
        unsafe { des.copy_from_slice(&self.des); }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des(&mut self, des: &mut [D; N])
    {
        let des_size = Self::size_of_des();
        let src_size = Self::size_of_src();
        if src_size > des_size
            { self.shl_assign_src((src_size - des_size) * 8); }
        else
            { self.shr_assign_des((des_size - src_size) * 8); }
        des.copy_from_slice(&self.des);
    }

    #[cfg(target_endian = "big")]
    fn shl_assign_src(&mut self, rhs: usize)
    {
        let TSIZE_BIT = size_of::<D>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = S::zero();
        if chunk_num > 0
        {
            self.src.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.src[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: S;
        let mut carry = zero;
        for idx in 0..N-chunk_num
        {
            num = (self.src[idx] << S::num(piece_num.into_u128())) | carry;
            carry = self.src[idx] >> S::num((TSIZE_BIT - piece_num).into_u128());
            self.src[idx] = num;
        }
    }

    #[cfg(target_endian = "big")]
    fn shr_assign_des(&mut self, rhs: usize)
    {
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = D::zero();
        if chunk_num > 0
        {
            self.des.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.des[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: D;
        let mut carry = D::zero();
        let mut idx = 0;
        loop
        {
            num = (self.des[idx] >> D::num(piece_num.into_u128())) | carry;
            carry = self.des[idx] << D::num((TSIZE_BIT - piece_num).into_u128());
            self.des[idx] = num;
            if idx == N - 1 - chunk_num
                { break; }
            idx += 1;
        }
    }

    pub fn size_of_des() -> usize   { size_of::<D>() * N }
    pub fn size_of_src() -> usize   { size_of::<S>() * M }
}

