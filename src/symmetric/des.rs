// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::ptr::copy_nonoverlapping;
// use std::slice::from_raw_parts;
// use std::fmt::{ self, Debug, Display, Formatter };
// use std::collections::HashMap;

use crate::number::{ SmallUint, IntUnion, LongUnion, SmallUInt };

// Converts bit number into 0-based bit number in Little Endianness.
macro_rules! convert {
    ($p:expr) => {
        (($p - 1) / 8) * 8 + (7 - (($p - 1) % 8))
    };
}

macro_rules! make_FP {
    () => {
        if true
        {
            let mut out = [0_u8; 64];
            out[Self::IP[00] as usize] = 00;
            out[Self::IP[01] as usize] = 01;
            out[Self::IP[02] as usize] = 02;
            out[Self::IP[03] as usize] = 03;
            out[Self::IP[04] as usize] = 04;
            out[Self::IP[05] as usize] = 05;
            out[Self::IP[06] as usize] = 06;
            out[Self::IP[07] as usize] = 07;
            out[Self::IP[08] as usize] = 08;
            out[Self::IP[09] as usize] = 09;
            out[Self::IP[10] as usize] = 10;
            out[Self::IP[11] as usize] = 11;
            out[Self::IP[12] as usize] = 12;
            out[Self::IP[13] as usize] = 13;
            out[Self::IP[14] as usize] = 14;
            out[Self::IP[15] as usize] = 15;
            out[Self::IP[16] as usize] = 16;
            out[Self::IP[17] as usize] = 17;
            out[Self::IP[18] as usize] = 18;
            out[Self::IP[19] as usize] = 19;
            out[Self::IP[20] as usize] = 20;
            out[Self::IP[21] as usize] = 21;
            out[Self::IP[22] as usize] = 22;
            out[Self::IP[23] as usize] = 23;
            out[Self::IP[24] as usize] = 24;
            out[Self::IP[25] as usize] = 25;
            out[Self::IP[26] as usize] = 26;
            out[Self::IP[27] as usize] = 27;
            out[Self::IP[28] as usize] = 28;
            out[Self::IP[29] as usize] = 29;
            out[Self::IP[30] as usize] = 30;
            out[Self::IP[31] as usize] = 31;
            out[Self::IP[32] as usize] = 32;
            out[Self::IP[33] as usize] = 33;
            out[Self::IP[34] as usize] = 34;
            out[Self::IP[35] as usize] = 35;
            out[Self::IP[36] as usize] = 36;
            out[Self::IP[37] as usize] = 37;
            out[Self::IP[38] as usize] = 38;
            out[Self::IP[39] as usize] = 39;
            out[Self::IP[40] as usize] = 40;
            out[Self::IP[41] as usize] = 41;
            out[Self::IP[42] as usize] = 42;
            out[Self::IP[43] as usize] = 43;
            out[Self::IP[44] as usize] = 44;
            out[Self::IP[45] as usize] = 45;
            out[Self::IP[46] as usize] = 46;
            out[Self::IP[47] as usize] = 47;
            out[Self::IP[48] as usize] = 48;
            out[Self::IP[49] as usize] = 49;
            out[Self::IP[50] as usize] = 50;
            out[Self::IP[51] as usize] = 51;
            out[Self::IP[52] as usize] = 52;
            out[Self::IP[53] as usize] = 53;
            out[Self::IP[54] as usize] = 54;
            out[Self::IP[55] as usize] = 55;
            out[Self::IP[56] as usize] = 56;
            out[Self::IP[57] as usize] = 57;
            out[Self::IP[58] as usize] = 58;
            out[Self::IP[59] as usize] = 59;
            out[Self::IP[60] as usize] = 60;
            out[Self::IP[61] as usize] = 61;
            out[Self::IP[62] as usize] = 62;
            out[Self::IP[63] as usize] = 63;
            out
        }
        else
        {
            [0_u8; 64]
        }
    }
}

macro_rules! permutate_data {
    ($me:expr, $permut:expr) => {
        let data = $me.block.get();
        let mut permuted = 0_u64;
        let mut idx = 0_usize;
        // $permut and pos are already changed to be 0-based in little endianness.
        for pos in $permut
        {
            if data.is_bit_set_(pos as usize)
                { permuted |= 1_u64 << idx; } // ((7 - idx % 8) + (idx / 8) * 8); }
            idx += 1;
        }
        $me.block.set(permuted);
    };
    // permutate_data!(Self::IP);
    //
    // let data = self.block.get();
    // let mut permuted = 0_u64;
    // let mut idx = 0_usize;
    // // IP and pos are already changed to be 0-based in little endianness.
    // for pos in Self::IP
    // {
    //     if data.is_bit_set_(pos as usize)
    //         { permuted |= 1_u64 << idx; } // ((7 - idx % 8) + (idx / 8) * 8); }
    //     idx += 1;
    // }
    // self.block.set(permuted);

    ($permut:expr, $type:ty, $right:expr) => {
        if true
        {
            let mut permuted = 0 as $type;
            let mut idx = 0_usize;
            // $permut and pos are already changed to be 0-based in little endianness.
            for pos in $permut
            {
                if $right.is_bit_set_(pos as usize)
                    { permuted |= 1 as $type << idx; } // ((7 - idx % 8) + (idx / 8) * 8); }
                idx += 1;
            }
            permuted
        }
        else
        {
            0 as $type
        }
    };
    // permutate_data!(Self::TP, u32, right);
    //
    // let mut permuted = 0_u32;
    // let mut idx = 0_usize;
    // // TP and pos are already changed to be 0-based in little endianness.
    // for pos in Self::TP
    // {
    //     if right.is_bit_set_(pos as usize)
    //         { permuted |= 1 << idx; }
    //     idx += 1;
    // }
    // permuted
}

macro_rules! shift_right_union {
    ($u:expr, $n:expr) => {
        let mut carry = 0_u8;
        for i in 0..4
        {
            let tmp = $u.get_ubyte_(i) << (8 - $n);
            $u.set_ubyte_(i, ($u.get_ubyte_(i) >> $n) | carry);
            carry = tmp;
        }
    };
}

macro_rules! shift_left_union {
    ($u:expr, $n:expr) => {
        let mut carry = 0_u8;
        for i in 0..4
        {
            let tmp = $u.get_ubyte_(3 - i) >> (8 - $n);
            $u.set_ubyte_(3 - i, ($u.get_ubyte_(3 - i) << $n) | carry);
            carry = tmp;
        }
    };
    ($u:expr, $n:expr, $size:expr) => {
        let mut carry = 0_u8;
        for i in 0..$size
        {
            let tmp = $u.get_ubyte_($size - 1 - i) >> (8 - $n);
            $u.set_ubyte_($size - 1 - i, ($u.get_ubyte_($size - 1 - i) << $n) | carry);
            carry = tmp;
        }
    };
}

macro_rules! rotate_halfkey {
    ($u:expr, $even:expr) => {
        let n = if $even {2} else {1};
        let mut carry = ($u.get_ubyte_(0) >> (4 - n)) & 0b11110000;
        for i in 0..4
        {
            let tmp = $u.get_ubyte_(3 - i) >> (8 - n);
            $u.set_ubyte_(3 - i, ($u.get_ubyte_(3 - i) << n) | carry);
            carry = tmp;
        }
    };
}

macro_rules! slice_index {
    ($indices:expr, $array:expr) => {
        let mut idx = LongUnion::new_with($indices);
        for i in 0..8_usize
        {
            $array[i] = ((idx.get_ubyte_(0) & 0b_111_111_00_u8) >> 2) as usize;
            shift_left_union!(idx, 6, 6);
        }
    };
}

macro_rules! combine_pieces {
    ($body:expr, $piece:expr) => {
        $body >>= 8;
        $body |= ($piece << 24);
    };
}

// /// You have freedom of changing H0 ~ H3, and ROUND.
// #[allow(non_camel_case_types)]
// pub type DES_Expanded<const N: usize = 4,
//                         const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
//                         const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
//                         const ROUND: usize = 48>
//                 = DES_Generic<N, H0, H1, H2, H3, ROUND>;

// /// You have freedom of changing ROUND, and K0 ~ K2.
// #[allow(non_camel_case_types)]
// pub type DES_Generic_HR_fixed<const N: usize, const ROUND: usize,
//                                 const K0: u32, const K1: u32, const K2: u32>
//     = DES_Generic<N, const S700: u8 = 0x67452301, const S700: u8 = 0xefcdab89, const S700: u8 = 0x98badcfe, const S700: u8 = 0x10325476, ROUND, K0, K1, K2>;

/// The official DES symmetric-key algorithm for the encryption of digital data
#[allow(non_camel_case_types)]
pub type DES = DES_Generic;    // equivalent to `pub type DES = DES_Expanded;`

/// A DES symmetric-key algorithm for the encryption of digital data
/// # Note
/// *This descryption about DES is according to big endianness.*
/// MSB (Most Significant Bit) is the first bit and LSB (Least Significant Bit)
/// is the 64th bit in this descryption.
/// 
/// # Introduction
/// // Todo
/// 
/// # Vulnerability
/// // Todo
/// 
/// # Use of DES and its variants
/// // Todo
/// 
/// # Generic Parameters
/// - ROUND: You can determine how many times the Fiestel network will be
///   repeated. Its maximum value will 128.
/// - SHIFT: You can determine how many bits the half keys will be rotated left
///   for the corresponding round in the key generator.
///   If SHIFT is '0b10011001', the half keys will be rotated left by one bit
///   for the first round, and will be rotated left by 2 bits for the second
///   round, and will be rotated left by 2 bits for the third round, and so on.
///   The LSB is for the first round and the MSB is for the 128th round. `0`
///   means that the half keys will be rotated left by two bits and `1` means
///   that the half keys will be rotated left by one bit. Up to only `ROUND`
///   bits from the LSB of SHIFT will be meaningful. For example, if `ROUND` is
///   5 and SHIFT is '0b10011001', only '0b11001' out of '0b10011001' is
///   meaningful. If `ROUND` is 16 and SHIFT is '0b10011001', SHIFT will be
///   understood to be '0b0000000010011001'.
/// - PC101 ~ PC248: Permutation compression.
/// - IP01 ~ IP64: Inital permutation constants. They are 1-based.
///   For example, `IP01 = 58` means that the 58th bit of data
///   is moved to the first bit of the data which is MSB at initial permutation.
///   You can change inital permutation wire by changing these constants
///   The change of these constants does not change the security level.
///   Final permutation constants is automatically calculated from Inital
///   permutation constants. FP01 ~ FP64 is inverse version of IP01 ~ IP64.
///   So, FP01 ~ FP64 will be automagically calculated. You don't have to
///   determine them.
/// - S000 ~ S763: S-Box constants, and its index such as 000, 212, etc. is
///   0-based. S0XX means S-Box 0, S1XX means S-Box 1, and so on. S000 is the
///   first element of S-Box 0.
///   According to [the document](https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm),
///   the input six bits determines the output of S-Box. The first and the last
///   bit of the six bits represent in base 2 a number in the decimal range 0 to
///   3 (or binary 00 to 11) which is row number. The rest middle four bits
///   represent in base 2 a number in the decimal range 0 to 15 (binary 0000 to
///   1111) which is column number. However, in this crate, S-boxes are
///   implemented to be two-dimensional array which is an array of S-box.
///   Each S-box is an array of 64 four-bit numbers. The input six-bit number
///   is used as the index of the array of these 64 four-bit numbers. So, the
///   S-box tables have been rearranged to be the 2-dimensional array.
///   You can cange S-Box by changing these constants.
///   *The change of these constants may hurt the security a lot.*
/// - EP01 ~ EP48: Expansion permutation constants.
/// - TP01 ~ TP32: Translation permutation constans.
/// 
/// // Todo
/// 
/// About the parameters and their default values,
/// read [more](https://datatracker.ietf.org/doc/html/rfc1320)
/// 
/// # Security of your own expanded version
/// // Todo
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/Data_Encryption_Standard)
/// about DES in detail.
/// 
/// # Quick Start
/// // Todo
/// 
#[allow(non_camel_case_types)]
pub struct DES_Generic<const ROUND: usize = 16, const SHIFT: u128 = 0b_1000000100000011,
const PC101: u8 = 57, const PC102: u8 = 49, const PC103: u8 = 41, const PC104: u8 = 33,
const PC105: u8 = 25, const PC106: u8 = 17, const PC107: u8 = 09, const PC108: u8 = 01,
const PC109: u8 = 58, const PC110: u8 = 50, const PC111: u8 = 42, const PC112: u8 = 34,
const PC113: u8 = 26, const PC114: u8 = 18, const PC115: u8 = 10, const PC116: u8 = 02,
const PC117: u8 = 59, const PC118: u8 = 51, const PC119: u8 = 43, const PC120: u8 = 35,
const PC121: u8 = 27, const PC122: u8 = 19, const PC123: u8 = 11, const PC124: u8 = 03,
const PC125: u8 = 60, const PC126: u8 = 52, const PC127: u8 = 44, const PC128: u8 = 36,
const PC129: u8 = 63, const PC130: u8 = 55, const PC131: u8 = 47, const PC132: u8 = 39,
const PC133: u8 = 31, const PC134: u8 = 23, const PC135: u8 = 15, const PC136: u8 = 07,
const PC137: u8 = 62, const PC138: u8 = 54, const PC139: u8 = 46, const PC140: u8 = 38,
const PC141: u8 = 30, const PC142: u8 = 22, const PC143: u8 = 14, const PC144: u8 = 06,
const PC145: u8 = 61, const PC146: u8 = 53, const PC147: u8 = 45, const PC148: u8 = 37,
const PC149: u8 = 29, const PC150: u8 = 21, const PC151: u8 = 13, const PC152: u8 = 05,
const PC153: u8 = 28, const PC154: u8 = 20, const PC155: u8 = 12, const PC156: u8 = 04,
const PC201: u8 = 14, const PC202: u8 = 17, const PC203: u8 = 11, const PC204: u8 = 24,
const PC205: u8 = 01, const PC206: u8 = 05, const PC207: u8 = 03, const PC208: u8 = 28,
const PC209: u8 = 15, const PC210: u8 = 06, const PC211: u8 = 21, const PC212: u8 = 10,
const PC213: u8 = 23, const PC214: u8 = 19, const PC215: u8 = 12, const PC216: u8 = 04,
const PC217: u8 = 26, const PC218: u8 = 08, const PC219: u8 = 16, const PC220: u8 = 07,
const PC221: u8 = 27, const PC222: u8 = 20, const PC223: u8 = 13, const PC224: u8 = 02,
const PC225: u8 = 41, const PC226: u8 = 52, const PC227: u8 = 31, const PC228: u8 = 37,
const PC229: u8 = 47, const PC230: u8 = 55, const PC231: u8 = 30, const PC232: u8 = 40,
const PC233: u8 = 51, const PC234: u8 = 45, const PC235: u8 = 33, const PC236: u8 = 48,
const PC237: u8 = 44, const PC238: u8 = 49, const PC239: u8 = 39, const PC240: u8 = 56,
const PC241: u8 = 34, const PC242: u8 = 53, const PC243: u8 = 46, const PC244: u8 = 42,
const PC245: u8 = 50, const PC246: u8 = 36, const PC247: u8 = 29, const PC248: u8 = 32,
const IP01: u8 = 58, const IP02: u8 = 50, const IP03: u8 = 42, const IP04: u8 = 34,
const IP05: u8 = 26, const IP06: u8 = 18, const IP07: u8 = 10, const IP08: u8 = 02,
const IP09: u8 = 60, const IP10: u8 = 52, const IP11: u8 = 44, const IP12: u8 = 36,
const IP13: u8 = 28, const IP14: u8 = 20, const IP15: u8 = 12, const IP16: u8 = 04,
const IP17: u8 = 62, const IP18: u8 = 54, const IP19: u8 = 46, const IP20: u8 = 38,
const IP21: u8 = 30, const IP22: u8 = 22, const IP23: u8 = 14, const IP24: u8 = 06,
const IP25: u8 = 64, const IP26: u8 = 56, const IP27: u8 = 48, const IP28: u8 = 40,
const IP29: u8 = 32, const IP30: u8 = 24, const IP31: u8 = 16, const IP32: u8 = 08,
const IP33: u8 = 57, const IP34: u8 = 49, const IP35: u8 = 41, const IP36: u8 = 33,
const IP37: u8 = 25, const IP38: u8 = 17, const IP39: u8 = 09, const IP40: u8 = 01,
const IP41: u8 = 59, const IP42: u8 = 51, const IP43: u8 = 43, const IP44: u8 = 35,
const IP45: u8 = 27, const IP46: u8 = 19, const IP47: u8 = 11, const IP48: u8 = 03,
const IP49: u8 = 61, const IP50: u8 = 53, const IP51: u8 = 45, const IP52: u8 = 37,
const IP53: u8 = 29, const IP54: u8 = 21, const IP55: u8 = 13, const IP56: u8 = 05,
const IP57: u8 = 63, const IP58: u8 = 55, const IP59: u8 = 47, const IP60: u8 = 39,
const IP61: u8 = 31, const IP62: u8 = 23, const IP63: u8 = 15, const IP64: u8 = 07,
const EP01: u8 = 32, const EP02: u8 = 01, const EP03: u8 = 02, const EP04: u8 = 03,
const EP05: u8 = 04, const EP06: u8 = 05, const EP07: u8 = 04, const EP08: u8 = 05,
const EP09: u8 = 06, const EP10: u8 = 07, const EP11: u8 = 08, const EP12: u8 = 09,
const EP13: u8 = 08, const EP14: u8 = 09, const EP15: u8 = 10, const EP16: u8 = 11,
const EP17: u8 = 12, const EP18: u8 = 13, const EP19: u8 = 12, const EP20: u8 = 13,
const EP21: u8 = 14, const EP22: u8 = 15, const EP23: u8 = 16, const EP24: u8 = 17,
const EP25: u8 = 16, const EP26: u8 = 17, const EP27: u8 = 18, const EP28: u8 = 19,
const EP29: u8 = 20, const EP30: u8 = 21, const EP31: u8 = 20, const EP32: u8 = 21,
const EP33: u8 = 22, const EP34: u8 = 23, const EP35: u8 = 24, const EP36: u8 = 25,
const EP37: u8 = 24, const EP38: u8 = 25, const EP39: u8 = 26, const EP40: u8 = 27,
const EP41: u8 = 28, const EP42: u8 = 29, const EP43: u8 = 28, const EP44: u8 = 29,
const EP45: u8 = 30, const EP46: u8 = 31, const EP47: u8 = 32, const EP48: u8 = 01,
const TP01: u8 = 16, const TP02: u8 = 07, const TP03: u8 = 20, const TP04: u8 = 21,
const TP05: u8 = 29, const TP06: u8 = 12, const TP07: u8 = 28, const TP08: u8 = 17,
const TP09: u8 = 01, const TP10: u8 = 15, const TP11: u8 = 23, const TP12: u8 = 26,
const TP13: u8 = 05, const TP14: u8 = 18, const TP15: u8 = 31, const TP16: u8 = 10,
const TP17: u8 = 02, const TP18: u8 = 08, const TP19: u8 = 24, const TP20: u8 = 14,
const TP21: u8 = 32, const TP22: u8 = 27, const TP23: u8 = 03, const TP24: u8 = 09,
const TP25: u8 = 19, const TP26: u8 = 13, const TP27: u8 = 30, const TP28: u8 = 06,
const TP29: u8 = 22, const TP30: u8 = 11, const TP31: u8 = 04, const TP32: u8 = 25,
// https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
// https://m.blog.naver.com/wnrjsxo/221708511553
const S000: u8 = 0xe, const S001: u8 = 0x0, const S002: u8 = 0x4, const S003: u8 = 0xf,
const S004: u8 = 0xd, const S005: u8 = 0x7, const S006: u8 = 0x1, const S007: u8 = 0x4,
const S008: u8 = 0x2, const S009: u8 = 0xe, const S010: u8 = 0xf, const S011: u8 = 0x2,
const S012: u8 = 0xb, const S013: u8 = 0xd, const S014: u8 = 0x8, const S015: u8 = 0x1,
const S016: u8 = 0x3, const S017: u8 = 0xa, const S018: u8 = 0xa, const S019: u8 = 0x6,
const S020: u8 = 0x6, const S021: u8 = 0xc, const S022: u8 = 0xc, const S023: u8 = 0xb,
const S024: u8 = 0x5, const S025: u8 = 0x9, const S026: u8 = 0x9, const S027: u8 = 0x5,
const S028: u8 = 0x0, const S029: u8 = 0x3, const S030: u8 = 0x7, const S031: u8 = 0x8,
const S032: u8 = 0x4, const S033: u8 = 0xf, const S034: u8 = 0x1, const S035: u8 = 0xc,
const S036: u8 = 0xe, const S037: u8 = 0x8, const S038: u8 = 0x8, const S039: u8 = 0x2,
const S040: u8 = 0xd, const S041: u8 = 0x4, const S042: u8 = 0x6, const S043: u8 = 0x9,
const S044: u8 = 0x2, const S045: u8 = 0x1, const S046: u8 = 0xb, const S047: u8 = 0x7,
const S048: u8 = 0xf, const S049: u8 = 0x5, const S050: u8 = 0xc, const S051: u8 = 0xb,
const S052: u8 = 0x9, const S053: u8 = 0x3, const S054: u8 = 0x7, const S055: u8 = 0xe,
const S056: u8 = 0x3, const S057: u8 = 0xa, const S058: u8 = 0xa, const S059: u8 = 0x0,
const S060: u8 = 0x5, const S061: u8 = 0x6, const S062: u8 = 0x0, const S063: u8 = 0xd,
const S100: u8 = 0xf, const S101: u8 = 0x3, const S102: u8 = 0x1, const S103: u8 = 0xd,
const S104: u8 = 0x8, const S105: u8 = 0x4, const S106: u8 = 0xe, const S107: u8 = 0x7,
const S108: u8 = 0x6, const S109: u8 = 0xf, const S110: u8 = 0xb, const S111: u8 = 0x2,
const S112: u8 = 0x3, const S113: u8 = 0x8, const S114: u8 = 0x4, const S115: u8 = 0xe,
const S116: u8 = 0x9, const S117: u8 = 0xc, const S118: u8 = 0x7, const S119: u8 = 0x0,
const S120: u8 = 0x2, const S121: u8 = 0x1, const S122: u8 = 0xd, const S123: u8 = 0xa,
const S124: u8 = 0xc, const S125: u8 = 0x6, const S126: u8 = 0x0, const S127: u8 = 0x9,
const S128: u8 = 0x5, const S129: u8 = 0xb, const S130: u8 = 0xa, const S131: u8 = 0x5,
const S132: u8 = 0x0, const S133: u8 = 0xd, const S134: u8 = 0xe, const S135: u8 = 0x8,
const S136: u8 = 0x7, const S137: u8 = 0xa, const S138: u8 = 0xb, const S139: u8 = 0x1,
const S140: u8 = 0xa, const S141: u8 = 0x3, const S142: u8 = 0x4, const S143: u8 = 0xf,
const S144: u8 = 0xd, const S145: u8 = 0x4, const S146: u8 = 0x1, const S147: u8 = 0x2,
const S148: u8 = 0x5, const S149: u8 = 0xb, const S150: u8 = 0x8, const S151: u8 = 0x6,
const S152: u8 = 0xc, const S153: u8 = 0x7, const S154: u8 = 0x6, const S155: u8 = 0xc,
const S156: u8 = 0x9, const S157: u8 = 0x0, const S158: u8 = 0x3, const S159: u8 = 0x5,
const S160: u8 = 0x2, const S161: u8 = 0xe, const S162: u8 = 0xf, const S163: u8 = 0x9,
const S200: u8 = 0xa, const S201: u8 = 0xd, const S202: u8 = 0x0, const S203: u8 = 0x7,
const S204: u8 = 0x9, const S205: u8 = 0x0, const S206: u8 = 0xe, const S207: u8 = 0x9,
const S208: u8 = 0x6, const S209: u8 = 0x3, const S210: u8 = 0x3, const S211: u8 = 0x4,
const S212: u8 = 0xf, const S213: u8 = 0x6, const S214: u8 = 0x5, const S215: u8 = 0xa,
const S216: u8 = 0x1, const S217: u8 = 0x2, const S218: u8 = 0xd, const S219: u8 = 0x8,
const S220: u8 = 0xc, const S221: u8 = 0x5, const S222: u8 = 0x7, const S223: u8 = 0xe,
const S224: u8 = 0xb, const S225: u8 = 0xc, const S226: u8 = 0x4, const S227: u8 = 0xb,
const S228: u8 = 0x2, const S229: u8 = 0xf, const S230: u8 = 0x8, const S231: u8 = 0x1,
const S232: u8 = 0xd, const S233: u8 = 0x1, const S234: u8 = 0x6, const S235: u8 = 0xa,
const S236: u8 = 0x4, const S237: u8 = 0xd, const S238: u8 = 0x9, const S239: u8 = 0x0,
const S240: u8 = 0x8, const S241: u8 = 0x6, const S242: u8 = 0xf, const S243: u8 = 0x9,
const S244: u8 = 0x3, const S245: u8 = 0x8, const S246: u8 = 0x0, const S247: u8 = 0x7,
const S248: u8 = 0xb, const S249: u8 = 0x4, const S250: u8 = 0x1, const S251: u8 = 0xf,
const S252: u8 = 0x2, const S253: u8 = 0xe, const S254: u8 = 0xc, const S255: u8 = 0x3,
const S256: u8 = 0x5, const S257: u8 = 0xb, const S258: u8 = 0xa, const S259: u8 = 0x5,
const S260: u8 = 0xe, const S261: u8 = 0x2, const S262: u8 = 0x7, const S263: u8 = 0xc,
const S300: u8 = 0x7, const S301: u8 = 0xd, const S302: u8 = 0xd, const S303: u8 = 0x8,
const S304: u8 = 0xe, const S305: u8 = 0xb, const S306: u8 = 0x3, const S307: u8 = 0x5,
const S308: u8 = 0x0, const S309: u8 = 0x6, const S310: u8 = 0x6, const S311: u8 = 0xf,
const S312: u8 = 0x9, const S313: u8 = 0x0, const S314: u8 = 0xa, const S315: u8 = 0x3,
const S316: u8 = 0x1, const S317: u8 = 0x4, const S318: u8 = 0x2, const S319: u8 = 0x7,
const S320: u8 = 0x8, const S321: u8 = 0x2, const S322: u8 = 0x5, const S323: u8 = 0xc,
const S324: u8 = 0xb, const S325: u8 = 0x1, const S326: u8 = 0xc, const S327: u8 = 0xa,
const S328: u8 = 0x4, const S329: u8 = 0xe, const S330: u8 = 0xf, const S331: u8 = 0x9,
const S332: u8 = 0xa, const S333: u8 = 0x3, const S334: u8 = 0x6, const S335: u8 = 0xf,
const S336: u8 = 0x9, const S337: u8 = 0x0, const S338: u8 = 0x0, const S339: u8 = 0x6,
const S340: u8 = 0xc, const S341: u8 = 0xa, const S342: u8 = 0xb, const S343: u8 = 0x1,
const S344: u8 = 0x7, const S345: u8 = 0xd, const S346: u8 = 0xd, const S347: u8 = 0x8,
const S348: u8 = 0xf, const S349: u8 = 0x9, const S350: u8 = 0x1, const S351: u8 = 0x4,
const S352: u8 = 0x3, const S353: u8 = 0x5, const S354: u8 = 0xe, const S355: u8 = 0xb,
const S356: u8 = 0x5, const S357: u8 = 0xc, const S358: u8 = 0x2, const S359: u8 = 0x7,
const S360: u8 = 0x8, const S361: u8 = 0x2, const S362: u8 = 0x4, const S363: u8 = 0xe,
const S400: u8 = 0x2, const S401: u8 = 0xe, const S402: u8 = 0xc, const S403: u8 = 0xb,
const S404: u8 = 0x4, const S405: u8 = 0x2, const S406: u8 = 0x1, const S407: u8 = 0xc,
const S408: u8 = 0x7, const S409: u8 = 0x4, const S410: u8 = 0xa, const S411: u8 = 0x7,
const S412: u8 = 0xb, const S413: u8 = 0xd, const S414: u8 = 0x6, const S415: u8 = 0x1,
const S416: u8 = 0x8, const S417: u8 = 0x5, const S418: u8 = 0x5, const S419: u8 = 0x0,
const S420: u8 = 0x3, const S421: u8 = 0xf, const S422: u8 = 0xf, const S423: u8 = 0xa,
const S424: u8 = 0xd, const S425: u8 = 0x3, const S426: u8 = 0x0, const S427: u8 = 0x9,
const S428: u8 = 0xe, const S429: u8 = 0x8, const S430: u8 = 0x9, const S431: u8 = 0x6,
const S432: u8 = 0x4, const S433: u8 = 0xb, const S434: u8 = 0x2, const S435: u8 = 0x8,
const S436: u8 = 0x1, const S437: u8 = 0xc, const S438: u8 = 0xb, const S439: u8 = 0x7,
const S440: u8 = 0xa, const S441: u8 = 0x1, const S442: u8 = 0xd, const S443: u8 = 0xe,
const S444: u8 = 0x7, const S445: u8 = 0x2, const S446: u8 = 0x8, const S447: u8 = 0xd,
const S448: u8 = 0xf, const S449: u8 = 0x6, const S450: u8 = 0x9, const S451: u8 = 0xf,
const S452: u8 = 0xc, const S453: u8 = 0x0, const S454: u8 = 0x5, const S455: u8 = 0x9,
const S456: u8 = 0x6, const S457: u8 = 0xa, const S458: u8 = 0x3, const S459: u8 = 0x4,
const S460: u8 = 0x0, const S461: u8 = 0x5, const S462: u8 = 0xe, const S463: u8 = 0x3,
const S500: u8 = 0xc, const S501: u8 = 0xa, const S502: u8 = 0x1, const S503: u8 = 0xf,
const S504: u8 = 0xa, const S505: u8 = 0x4, const S506: u8 = 0xf, const S507: u8 = 0x2,
const S508: u8 = 0x9, const S509: u8 = 0x7, const S510: u8 = 0x2, const S511: u8 = 0xc,
const S512: u8 = 0x6, const S513: u8 = 0x9, const S514: u8 = 0x8, const S515: u8 = 0x5,
const S516: u8 = 0x0, const S517: u8 = 0x6, const S518: u8 = 0xd, const S519: u8 = 0x1,
const S520: u8 = 0x3, const S521: u8 = 0xd, const S522: u8 = 0x4, const S523: u8 = 0xe,
const S524: u8 = 0xe, const S525: u8 = 0x0, const S526: u8 = 0x7, const S527: u8 = 0xb,
const S528: u8 = 0x5, const S529: u8 = 0x3, const S530: u8 = 0xb, const S531: u8 = 0x8,
const S532: u8 = 0x9, const S533: u8 = 0x4, const S534: u8 = 0xe, const S535: u8 = 0x3,
const S536: u8 = 0xf, const S537: u8 = 0x2, const S538: u8 = 0x5, const S539: u8 = 0xc,
const S540: u8 = 0x2, const S541: u8 = 0x9, const S542: u8 = 0x8, const S543: u8 = 0x5,
const S544: u8 = 0xc, const S545: u8 = 0xf, const S546: u8 = 0x3, const S547: u8 = 0xa,
const S548: u8 = 0x7, const S549: u8 = 0xb, const S550: u8 = 0x0, const S551: u8 = 0xe,
const S552: u8 = 0x4, const S553: u8 = 0x1, const S554: u8 = 0xa, const S555: u8 = 0x7,
const S556: u8 = 0x1, const S557: u8 = 0x6, const S558: u8 = 0xd, const S559: u8 = 0x0,
const S560: u8 = 0xb, const S561: u8 = 0x8, const S562: u8 = 0x6, const S563: u8 = 0xd,
const S600: u8 = 0x4, const S601: u8 = 0xd, const S602: u8 = 0xb, const S603: u8 = 0x0,
const S604: u8 = 0x2, const S605: u8 = 0xb, const S606: u8 = 0xe, const S607: u8 = 0x7,
const S608: u8 = 0xf, const S609: u8 = 0x4, const S610: u8 = 0x0, const S611: u8 = 0x9,
const S612: u8 = 0x8, const S613: u8 = 0x1, const S614: u8 = 0xd, const S615: u8 = 0xa,
const S616: u8 = 0x3, const S617: u8 = 0xe, const S618: u8 = 0xc, const S619: u8 = 0x3,
const S620: u8 = 0x9, const S621: u8 = 0x5, const S622: u8 = 0x7, const S623: u8 = 0xc,
const S624: u8 = 0x5, const S625: u8 = 0x2, const S626: u8 = 0xa, const S627: u8 = 0xf,
const S628: u8 = 0x6, const S629: u8 = 0x8, const S630: u8 = 0x1, const S631: u8 = 0x6,
const S632: u8 = 0x1, const S633: u8 = 0x6, const S634: u8 = 0x4, const S635: u8 = 0xb,
const S636: u8 = 0xb, const S637: u8 = 0xd, const S638: u8 = 0xd, const S639: u8 = 0x8,
const S640: u8 = 0xc, const S641: u8 = 0x1, const S642: u8 = 0x3, const S643: u8 = 0x4,
const S644: u8 = 0x7, const S645: u8 = 0xa, const S646: u8 = 0xe, const S647: u8 = 0x7,
const S648: u8 = 0xa, const S649: u8 = 0x9, const S650: u8 = 0xf, const S651: u8 = 0x5,
const S652: u8 = 0x6, const S653: u8 = 0x0, const S654: u8 = 0x8, const S655: u8 = 0xf,
const S656: u8 = 0x0, const S657: u8 = 0xe, const S658: u8 = 0x5, const S659: u8 = 0x2,
const S660: u8 = 0x9, const S661: u8 = 0x3, const S662: u8 = 0x2, const S663: u8 = 0xc,
const S700: u8 = 0xd, const S701: u8 = 0x1, const S702: u8 = 0x2, const S703: u8 = 0xf,
const S704: u8 = 0x8, const S705: u8 = 0xd, const S706: u8 = 0x4, const S707: u8 = 0x8,
const S708: u8 = 0x6, const S709: u8 = 0xa, const S710: u8 = 0xf, const S711: u8 = 0x3,
const S712: u8 = 0xb, const S713: u8 = 0x7, const S714: u8 = 0x1, const S715: u8 = 0x4,
const S716: u8 = 0xa, const S717: u8 = 0xc, const S718: u8 = 0x9, const S719: u8 = 0x5,
const S720: u8 = 0x3, const S721: u8 = 0x6, const S722: u8 = 0xe, const S723: u8 = 0xb,
const S724: u8 = 0x5, const S725: u8 = 0x0, const S726: u8 = 0x0, const S727: u8 = 0xe,
const S728: u8 = 0xc, const S729: u8 = 0x9, const S730: u8 = 0x7, const S731: u8 = 0x2,
const S732: u8 = 0x7, const S733: u8 = 0x2, const S734: u8 = 0xb, const S735: u8 = 0x1,
const S736: u8 = 0x4, const S737: u8 = 0xe, const S738: u8 = 0x1, const S739: u8 = 0x7,
const S740: u8 = 0x9, const S741: u8 = 0x4, const S742: u8 = 0xc, const S743: u8 = 0xa,
const S744: u8 = 0xe, const S745: u8 = 0x8, const S746: u8 = 0x2, const S747: u8 = 0xd,
const S748: u8 = 0x0, const S749: u8 = 0xf, const S750: u8 = 0x6, const S751: u8 = 0xc,
const S752: u8 = 0xa, const S753: u8 = 0x9, const S754: u8 = 0xd, const S755: u8 = 0x0,
const S756: u8 = 0xf, const S757: u8 = 0x3, const S758: u8 = 0x3, const S759: u8 = 0x5,
const S760: u8 = 0x5, const S761: u8 = 0x6, const S762: u8 = 0x8, const S763: u8 = 0xb
>
{
    key: LongUnion,
    block: LongUnion,
    round_key: [u64; ROUND],
}

impl <const ROUND: usize, const SHIFT: u128,
const PC101: u8, const PC102: u8, const PC103: u8, const PC104: u8,
const PC105: u8, const PC106: u8, const PC107: u8, const PC108: u8,
const PC109: u8, const PC110: u8, const PC111: u8, const PC112: u8,
const PC113: u8, const PC114: u8, const PC115: u8, const PC116: u8,
const PC117: u8, const PC118: u8, const PC119: u8, const PC120: u8,
const PC121: u8, const PC122: u8, const PC123: u8, const PC124: u8,
const PC125: u8, const PC126: u8, const PC127: u8, const PC128: u8,
const PC129: u8, const PC130: u8, const PC131: u8, const PC132: u8,
const PC133: u8, const PC134: u8, const PC135: u8, const PC136: u8,
const PC137: u8, const PC138: u8, const PC139: u8, const PC140: u8,
const PC141: u8, const PC142: u8, const PC143: u8, const PC144: u8,
const PC145: u8, const PC146: u8, const PC147: u8, const PC148: u8,
const PC149: u8, const PC150: u8, const PC151: u8, const PC152: u8,
const PC153: u8, const PC154: u8, const PC155: u8, const PC156: u8,
const PC201: u8, const PC202: u8, const PC203: u8, const PC204: u8,
const PC205: u8, const PC206: u8, const PC207: u8, const PC208: u8,
const PC209: u8, const PC210: u8, const PC211: u8, const PC212: u8,
const PC213: u8, const PC214: u8, const PC215: u8, const PC216: u8,
const PC217: u8, const PC218: u8, const PC219: u8, const PC220: u8,
const PC221: u8, const PC222: u8, const PC223: u8, const PC224: u8,
const PC225: u8, const PC226: u8, const PC227: u8, const PC228: u8,
const PC229: u8, const PC230: u8, const PC231: u8, const PC232: u8,
const PC233: u8, const PC234: u8, const PC235: u8, const PC236: u8,
const PC237: u8, const PC238: u8, const PC239: u8, const PC240: u8,
const PC241: u8, const PC242: u8, const PC243: u8, const PC244: u8,
const PC245: u8, const PC246: u8, const PC247: u8, const PC248: u8,
const IP01: u8, const IP02: u8, const IP03: u8, const IP04: u8,
const IP05: u8, const IP06: u8, const IP07: u8, const IP08: u8,
const IP09: u8, const IP10: u8, const IP11: u8, const IP12: u8,
const IP13: u8, const IP14: u8, const IP15: u8, const IP16: u8,
const IP17: u8, const IP18: u8, const IP19: u8, const IP20: u8,
const IP21: u8, const IP22: u8, const IP23: u8, const IP24: u8,
const IP25: u8, const IP26: u8, const IP27: u8, const IP28: u8,
const IP29: u8, const IP30: u8, const IP31: u8, const IP32: u8,
const IP33: u8, const IP34: u8, const IP35: u8, const IP36: u8,
const IP37: u8, const IP38: u8, const IP39: u8, const IP40: u8,
const IP41: u8, const IP42: u8, const IP43: u8, const IP44: u8,
const IP45: u8, const IP46: u8, const IP47: u8, const IP48: u8,
const IP49: u8, const IP50: u8, const IP51: u8, const IP52: u8,
const IP53: u8, const IP54: u8, const IP55: u8, const IP56: u8,
const IP57: u8, const IP58: u8, const IP59: u8, const IP60: u8,
const IP61: u8, const IP62: u8, const IP63: u8, const IP64: u8,
const EP01: u8, const EP02: u8, const EP03: u8, const EP04: u8,
const EP05: u8, const EP06: u8, const EP07: u8, const EP08: u8,
const EP09: u8, const EP10: u8, const EP11: u8, const EP12: u8,
const EP13: u8, const EP14: u8, const EP15: u8, const EP16: u8,
const EP17: u8, const EP18: u8, const EP19: u8, const EP20: u8,
const EP21: u8, const EP22: u8, const EP23: u8, const EP24: u8,
const EP25: u8, const EP26: u8, const EP27: u8, const EP28: u8,
const EP29: u8, const EP30: u8, const EP31: u8, const EP32: u8,
const EP33: u8, const EP34: u8, const EP35: u8, const EP36: u8,
const EP37: u8, const EP38: u8, const EP39: u8, const EP40: u8,
const EP41: u8, const EP42: u8, const EP43: u8, const EP44: u8,
const EP45: u8, const EP46: u8, const EP47: u8, const EP48: u8,
const TP01: u8, const TP02: u8, const TP03: u8, const TP04: u8,
const TP05: u8, const TP06: u8, const TP07: u8, const TP08: u8,
const TP09: u8, const TP10: u8, const TP11: u8, const TP12: u8,
const TP13: u8, const TP14: u8, const TP15: u8, const TP16: u8,
const TP17: u8, const TP18: u8, const TP19: u8, const TP20: u8,
const TP21: u8, const TP22: u8, const TP23: u8, const TP24: u8,
const TP25: u8, const TP26: u8, const TP27: u8, const TP28: u8,
const TP29: u8, const TP30: u8, const TP31: u8, const TP32: u8,
const S000: u8, const S001: u8, const S002: u8, const S003: u8,
const S004: u8, const S005: u8, const S006: u8, const S007: u8,
const S008: u8, const S009: u8, const S010: u8, const S011: u8,
const S012: u8, const S013: u8, const S014: u8, const S015: u8,
const S016: u8, const S017: u8, const S018: u8, const S019: u8,
const S020: u8, const S021: u8, const S022: u8, const S023: u8,
const S024: u8, const S025: u8, const S026: u8, const S027: u8,
const S028: u8, const S029: u8, const S030: u8, const S031: u8,
const S032: u8, const S033: u8, const S034: u8, const S035: u8,
const S036: u8, const S037: u8, const S038: u8, const S039: u8,
const S040: u8, const S041: u8, const S042: u8, const S043: u8,
const S044: u8, const S045: u8, const S046: u8, const S047: u8,
const S048: u8, const S049: u8, const S050: u8, const S051: u8,
const S052: u8, const S053: u8, const S054: u8, const S055: u8,
const S056: u8, const S057: u8, const S058: u8, const S059: u8,
const S060: u8, const S061: u8, const S062: u8, const S063: u8,
const S100: u8, const S101: u8, const S102: u8, const S103: u8,
const S104: u8, const S105: u8, const S106: u8, const S107: u8,
const S108: u8, const S109: u8, const S110: u8, const S111: u8,
const S112: u8, const S113: u8, const S114: u8, const S115: u8,
const S116: u8, const S117: u8, const S118: u8, const S119: u8,
const S120: u8, const S121: u8, const S122: u8, const S123: u8,
const S124: u8, const S125: u8, const S126: u8, const S127: u8,
const S128: u8, const S129: u8, const S130: u8, const S131: u8,
const S132: u8, const S133: u8, const S134: u8, const S135: u8,
const S136: u8, const S137: u8, const S138: u8, const S139: u8,
const S140: u8, const S141: u8, const S142: u8, const S143: u8,
const S144: u8, const S145: u8, const S146: u8, const S147: u8,
const S148: u8, const S149: u8, const S150: u8, const S151: u8,
const S152: u8, const S153: u8, const S154: u8, const S155: u8,
const S156: u8, const S157: u8, const S158: u8, const S159: u8,
const S160: u8, const S161: u8, const S162: u8, const S163: u8,
const S200: u8, const S201: u8, const S202: u8, const S203: u8,
const S204: u8, const S205: u8, const S206: u8, const S207: u8,
const S208: u8, const S209: u8, const S210: u8, const S211: u8,
const S212: u8, const S213: u8, const S214: u8, const S215: u8,
const S216: u8, const S217: u8, const S218: u8, const S219: u8,
const S220: u8, const S221: u8, const S222: u8, const S223: u8,
const S224: u8, const S225: u8, const S226: u8, const S227: u8,
const S228: u8, const S229: u8, const S230: u8, const S231: u8,
const S232: u8, const S233: u8, const S234: u8, const S235: u8,
const S236: u8, const S237: u8, const S238: u8, const S239: u8,
const S240: u8, const S241: u8, const S242: u8, const S243: u8,
const S244: u8, const S245: u8, const S246: u8, const S247: u8,
const S248: u8, const S249: u8, const S250: u8, const S251: u8,
const S252: u8, const S253: u8, const S254: u8, const S255: u8,
const S256: u8, const S257: u8, const S258: u8, const S259: u8,
const S260: u8, const S261: u8, const S262: u8, const S263: u8,
const S300: u8, const S301: u8, const S302: u8, const S303: u8,
const S304: u8, const S305: u8, const S306: u8, const S307: u8,
const S308: u8, const S309: u8, const S310: u8, const S311: u8,
const S312: u8, const S313: u8, const S314: u8, const S315: u8,
const S316: u8, const S317: u8, const S318: u8, const S319: u8,
const S320: u8, const S321: u8, const S322: u8, const S323: u8,
const S324: u8, const S325: u8, const S326: u8, const S327: u8,
const S328: u8, const S329: u8, const S330: u8, const S331: u8,
const S332: u8, const S333: u8, const S334: u8, const S335: u8,
const S336: u8, const S337: u8, const S338: u8, const S339: u8,
const S340: u8, const S341: u8, const S342: u8, const S343: u8,
const S344: u8, const S345: u8, const S346: u8, const S347: u8,
const S348: u8, const S349: u8, const S350: u8, const S351: u8,
const S352: u8, const S353: u8, const S354: u8, const S355: u8,
const S356: u8, const S357: u8, const S358: u8, const S359: u8,
const S360: u8, const S361: u8, const S362: u8, const S363: u8,
const S400: u8, const S401: u8, const S402: u8, const S403: u8,
const S404: u8, const S405: u8, const S406: u8, const S407: u8,
const S408: u8, const S409: u8, const S410: u8, const S411: u8,
const S412: u8, const S413: u8, const S414: u8, const S415: u8,
const S416: u8, const S417: u8, const S418: u8, const S419: u8,
const S420: u8, const S421: u8, const S422: u8, const S423: u8,
const S424: u8, const S425: u8, const S426: u8, const S427: u8,
const S428: u8, const S429: u8, const S430: u8, const S431: u8,
const S432: u8, const S433: u8, const S434: u8, const S435: u8,
const S436: u8, const S437: u8, const S438: u8, const S439: u8,
const S440: u8, const S441: u8, const S442: u8, const S443: u8,
const S444: u8, const S445: u8, const S446: u8, const S447: u8,
const S448: u8, const S449: u8, const S450: u8, const S451: u8,
const S452: u8, const S453: u8, const S454: u8, const S455: u8,
const S456: u8, const S457: u8, const S458: u8, const S459: u8,
const S460: u8, const S461: u8, const S462: u8, const S463: u8,
const S500: u8, const S501: u8, const S502: u8, const S503: u8,
const S504: u8, const S505: u8, const S506: u8, const S507: u8,
const S508: u8, const S509: u8, const S510: u8, const S511: u8,
const S512: u8, const S513: u8, const S514: u8, const S515: u8,
const S516: u8, const S517: u8, const S518: u8, const S519: u8,
const S520: u8, const S521: u8, const S522: u8, const S523: u8,
const S524: u8, const S525: u8, const S526: u8, const S527: u8,
const S528: u8, const S529: u8, const S530: u8, const S531: u8,
const S532: u8, const S533: u8, const S534: u8, const S535: u8,
const S536: u8, const S537: u8, const S538: u8, const S539: u8,
const S540: u8, const S541: u8, const S542: u8, const S543: u8,
const S544: u8, const S545: u8, const S546: u8, const S547: u8,
const S548: u8, const S549: u8, const S550: u8, const S551: u8,
const S552: u8, const S553: u8, const S554: u8, const S555: u8,
const S556: u8, const S557: u8, const S558: u8, const S559: u8,
const S560: u8, const S561: u8, const S562: u8, const S563: u8,
const S600: u8, const S601: u8, const S602: u8, const S603: u8,
const S604: u8, const S605: u8, const S606: u8, const S607: u8,
const S608: u8, const S609: u8, const S610: u8, const S611: u8,
const S612: u8, const S613: u8, const S614: u8, const S615: u8,
const S616: u8, const S617: u8, const S618: u8, const S619: u8,
const S620: u8, const S621: u8, const S622: u8, const S623: u8,
const S624: u8, const S625: u8, const S626: u8, const S627: u8,
const S628: u8, const S629: u8, const S630: u8, const S631: u8,
const S632: u8, const S633: u8, const S634: u8, const S635: u8,
const S636: u8, const S637: u8, const S638: u8, const S639: u8,
const S640: u8, const S641: u8, const S642: u8, const S643: u8,
const S644: u8, const S645: u8, const S646: u8, const S647: u8,
const S648: u8, const S649: u8, const S650: u8, const S651: u8,
const S652: u8, const S653: u8, const S654: u8, const S655: u8,
const S656: u8, const S657: u8, const S658: u8, const S659: u8,
const S660: u8, const S661: u8, const S662: u8, const S663: u8,
const S700: u8, const S701: u8, const S702: u8, const S703: u8,
const S704: u8, const S705: u8, const S706: u8, const S707: u8,
const S708: u8, const S709: u8, const S710: u8, const S711: u8,
const S712: u8, const S713: u8, const S714: u8, const S715: u8,
const S716: u8, const S717: u8, const S718: u8, const S719: u8,
const S720: u8, const S721: u8, const S722: u8, const S723: u8,
const S724: u8, const S725: u8, const S726: u8, const S727: u8,
const S728: u8, const S729: u8, const S730: u8, const S731: u8,
const S732: u8, const S733: u8, const S734: u8, const S735: u8,
const S736: u8, const S737: u8, const S738: u8, const S739: u8,
const S740: u8, const S741: u8, const S742: u8, const S743: u8,
const S744: u8, const S745: u8, const S746: u8, const S747: u8,
const S748: u8, const S749: u8, const S750: u8, const S751: u8,
const S752: u8, const S753: u8, const S754: u8, const S755: u8,
const S756: u8, const S757: u8, const S758: u8, const S759: u8,
const S760: u8, const S761: u8, const S762: u8, const S763: u8
>
DES_Generic<ROUND, SHIFT,
PC101, PC102, PC103, PC104, PC105, PC106, PC107, PC108,
PC109, PC110, PC111, PC112, PC113, PC114, PC115, PC116,
PC117, PC118, PC119, PC120, PC121, PC122, PC123, PC124,
PC125, PC126, PC127, PC128, PC129, PC130, PC131, PC132,
PC133, PC134, PC135, PC136, PC137, PC138, PC139, PC140,
PC141, PC142, PC143, PC144, PC145, PC146, PC147, PC148,
PC149, PC150, PC151, PC152, PC153, PC154, PC155, PC156,
PC201, PC202, PC203, PC204, PC205, PC206, PC207, PC208,
PC209, PC210, PC211, PC212, PC213, PC214, PC215, PC216,
PC217, PC218, PC219, PC220, PC221, PC222, PC223, PC224,
PC225, PC226, PC227, PC228, PC229, PC230, PC231, PC232,
PC233, PC234, PC235, PC236, PC237, PC238, PC239, PC240,
PC241, PC242, PC243, PC244, PC245, PC246, PC247, PC248,
IP01, IP02, IP03, IP04, IP05, IP06, IP07, IP08,
IP09, IP10, IP11, IP12, IP13, IP14, IP15, IP16,
IP17, IP18, IP19, IP20, IP21, IP22, IP23, IP24,
IP25, IP26, IP27, IP28, IP29, IP30, IP31, IP32,
IP33, IP34, IP35, IP36, IP37, IP38, IP39, IP40,
IP41, IP42, IP43, IP44, IP45, IP46, IP47, IP48,
IP49, IP50, IP51, IP52, IP53, IP54, IP55, IP56,
IP57, IP58, IP59, IP60, IP61, IP62, IP63, IP64,
EP01, EP02, EP03, EP04, EP05, EP06, EP07, EP08,
EP09, EP10, EP11, EP12, EP13, EP14, EP15, EP16,
EP17, EP18, EP19, EP20, EP21, EP22, EP23, EP24,
EP25, EP26, EP27, EP28, EP29, EP30, EP31, EP32,
EP33, EP34, EP35, EP36, EP37, EP38, EP39, EP40,
EP41, EP42, EP43, EP44, EP45, EP46, EP47, EP48,
TP01, TP02, TP03, TP04, TP05, TP06, TP07, TP08,
TP09, TP10, TP11, TP12, TP13, TP14, TP15, TP16,
TP17, TP18, TP19, TP20, TP21, TP22, TP23, TP24,
TP25, TP26, TP27, TP28, TP29, TP30, TP31, TP32,
S000, S001, S002, S003, S004, S005, S006, S007,
S008, S009, S010, S011, S012, S013, S014, S015,
S016, S017, S018, S019, S020, S021, S022, S023,
S024, S025, S026, S027, S028, S029, S030, S031,
S032, S033, S034, S035, S036, S037, S038, S039,
S040, S041, S042, S043, S044, S045, S046, S047,
S048, S049, S050, S051, S052, S053, S054, S055,
S056, S057, S058, S059, S060, S061, S062, S063,
S100, S101, S102, S103, S104, S105, S106, S107,
S108, S109, S110, S111, S112, S113, S114, S115,
S116, S117, S118, S119, S120, S121, S122, S123,
S124, S125, S126, S127, S128, S129, S130, S131,
S132, S133, S134, S135, S136, S137, S138, S139,
S140, S141, S142, S143, S144, S145, S146, S147,
S148, S149, S150, S151, S152, S153, S154, S155,
S156, S157, S158, S159, S160, S161, S162, S163,
S200, S201, S202, S203, S204, S205, S206, S207,
S208, S209, S210, S211, S212, S213, S214, S215,
S216, S217, S218, S219, S220, S221, S222, S223,
S224, S225, S226, S227, S228, S229, S230, S231,
S232, S233, S234, S235, S236, S237, S238, S239,
S240, S241, S242, S243, S244, S245, S246, S247,
S248, S249, S250, S251, S252, S253, S254, S255,
S256, S257, S258, S259, S260, S261, S262, S263,
S300, S301, S302, S303, S304, S305, S306, S307,
S308, S309, S310, S311, S312, S313, S314, S315,
S316, S317, S318, S319, S320, S321, S322, S323,
S324, S325, S326, S327, S328, S329, S330, S331,
S332, S333, S334, S335, S336, S337, S338, S339,
S340, S341, S342, S343, S344, S345, S346, S347,
S348, S349, S350, S351, S352, S353, S354, S355,
S356, S357, S358, S359, S360, S361, S362, S363,
S400, S401, S402, S403, S404, S405, S406, S407,
S408, S409, S410, S411, S412, S413, S414, S415,
S416, S417, S418, S419, S420, S421, S422, S423,
S424, S425, S426, S427, S428, S429, S430, S431,
S432, S433, S434, S435, S436, S437, S438, S439,
S440, S441, S442, S443, S444, S445, S446, S447,
S448, S449, S450, S451, S452, S453, S454, S455,
S456, S457, S458, S459, S460, S461, S462, S463,
S500, S501, S502, S503, S504, S505, S506, S507,
S508, S509, S510, S511, S512, S513, S514, S515,
S516, S517, S518, S519, S520, S521, S522, S523,
S524, S525, S526, S527, S528, S529, S530, S531,
S532, S533, S534, S535, S536, S537, S538, S539,
S540, S541, S542, S543, S544, S545, S546, S547,
S548, S549, S550, S551, S552, S553, S554, S555,
S556, S557, S558, S559, S560, S561, S562, S563,
S600, S601, S602, S603, S604, S605, S606, S607,
S608, S609, S610, S611, S612, S613, S614, S615,
S616, S617, S618, S619, S620, S621, S622, S623,
S624, S625, S626, S627, S628, S629, S630, S631,
S632, S633, S634, S635, S636, S637, S638, S639,
S640, S641, S642, S643, S644, S645, S646, S647,
S648, S649, S650, S651, S652, S653, S654, S655,
S656, S657, S658, S659, S660, S661, S662, S663,
S700, S701, S702, S703, S704, S705, S706, S707,
S708, S709, S710, S711, S712, S713, S714, S715,
S716, S717, S718, S719, S720, S721, S722, S723,
S724, S725, S726, S727, S728, S729, S730, S731,
S732, S733, S734, S735, S736, S737, S738, S739,
S740, S741, S742, S743, S744, S745, S746, S747,
S748, S749, S750, S751, S752, S753, S754, S755,
S756, S757, S758, S759, S760, S761, S762, S763
>
{
    const SBOX: [[u8; 64]; 8] = [
              [ S000, S001, S002, S003, S004, S005, S006, S007,
                S008, S009, S010, S011, S012, S013, S014, S015,
                S016, S017, S018, S019, S020, S021, S022, S023,
                S024, S025, S026, S027, S028, S029, S030, S031,
                S032, S033, S034, S035, S036, S037, S038, S039,
                S040, S041, S042, S043, S044, S045, S046, S047,
                S048, S049, S050, S051, S052, S053, S054, S055,
                S056, S057, S058, S059, S060, S061, S062, S063 ],
              [ S100, S101, S102, S103, S104, S105, S106, S107,
                S108, S109, S110, S111, S112, S113, S114, S115,
                S116, S117, S118, S119, S120, S121, S122, S123,
                S124, S125, S126, S127, S128, S129, S130, S131,
                S132, S133, S134, S135, S136, S137, S138, S139,
                S140, S141, S142, S143, S144, S145, S146, S147,
                S148, S149, S150, S151, S152, S153, S154, S155,
                S156, S157, S158, S159, S160, S161, S162, S163 ],
              [ S200, S201, S202, S203, S204, S205, S206, S207,
                S208, S209, S210, S211, S212, S213, S214, S215,
                S216, S217, S218, S219, S220, S221, S222, S223,
                S224, S225, S226, S227, S228, S229, S230, S231,
                S232, S233, S234, S235, S236, S237, S238, S239,
                S240, S241, S242, S243, S244, S245, S246, S247,
                S248, S249, S250, S251, S252, S253, S254, S255,
                S256, S257, S258, S259, S260, S261, S262, S263 ],
              [ S300, S301, S302, S303, S304, S305, S306, S307,
                S308, S309, S310, S311, S312, S313, S314, S315,
                S316, S317, S318, S319, S320, S321, S322, S323,
                S324, S325, S326, S327, S328, S329, S330, S331,
                S332, S333, S334, S335, S336, S337, S338, S339,
                S340, S341, S342, S343, S344, S345, S346, S347,
                S348, S349, S350, S351, S352, S353, S354, S355,
                S356, S357, S358, S359, S360, S361, S362, S363 ],
              [ S400, S401, S402, S403, S404, S405, S406, S407,
                S408, S409, S410, S411, S412, S413, S414, S415,
                S416, S417, S418, S419, S420, S421, S422, S423,
                S424, S425, S426, S427, S428, S429, S430, S431,
                S432, S433, S434, S435, S436, S437, S438, S439,
                S440, S441, S442, S443, S444, S445, S446, S447,
                S448, S449, S450, S451, S452, S453, S454, S455,
                S456, S457, S458, S459, S460, S461, S462, S463 ],
              [ S500, S501, S502, S503, S504, S505, S506, S507,
                S508, S509, S510, S511, S512, S513, S514, S515,
                S516, S517, S518, S519, S520, S521, S522, S523,
                S524, S525, S526, S527, S528, S529, S530, S531,
                S532, S533, S534, S535, S536, S537, S538, S539,
                S540, S541, S542, S543, S544, S545, S546, S547,
                S548, S549, S550, S551, S552, S553, S554, S555,
                S556, S557, S558, S559, S560, S561, S562, S563 ],
              [ S600, S601, S602, S603, S604, S605, S606, S607,
                S608, S609, S610, S611, S612, S613, S614, S615,
                S616, S617, S618, S619, S620, S621, S622, S623,
                S624, S625, S626, S627, S628, S629, S630, S631,
                S632, S633, S634, S635, S636, S637, S638, S639,
                S640, S641, S642, S643, S644, S645, S646, S647,
                S648, S649, S650, S651, S652, S653, S654, S655,
                S656, S657, S658, S659, S660, S661, S662, S663 ],
              [ S700, S701, S702, S703, S704, S705, S706, S707,
                S708, S709, S710, S711, S712, S713, S714, S715,
                S716, S717, S718, S719, S720, S721, S722, S723,
                S724, S725, S726, S727, S728, S729, S730, S731,
                S732, S733, S734, S735, S736, S737, S738, S739,
                S740, S741, S742, S743, S744, S745, S746, S747,
                S748, S749, S750, S751, S752, S753, S754, S755,
                S756, S757, S758, S759, S760, S761, S762, S763 ]
    ];

    // Initial Permutation Table changed to be 0-based in little endianness
    // So, LSB is 0-th bit and MSB is 63-rd bit in u64.
    const IP: [u8; 64] = [
            convert!(IP08), convert!(IP07), convert!(IP06), convert!(IP05),
            convert!(IP04), convert!(IP03), convert!(IP02), convert!(IP01),
            convert!(IP16), convert!(IP15), convert!(IP14), convert!(IP13),
            convert!(IP12), convert!(IP11), convert!(IP10), convert!(IP09),
            convert!(IP24), convert!(IP23), convert!(IP22), convert!(IP21),
            convert!(IP20), convert!(IP19), convert!(IP18), convert!(IP17),
            convert!(IP32), convert!(IP31), convert!(IP30), convert!(IP29),
            convert!(IP28), convert!(IP27), convert!(IP26), convert!(IP25),
            convert!(IP40), convert!(IP39), convert!(IP38), convert!(IP37),
            convert!(IP36), convert!(IP35), convert!(IP34), convert!(IP33),
            convert!(IP48), convert!(IP47), convert!(IP46), convert!(IP45),
            convert!(IP44), convert!(IP43), convert!(IP42), convert!(IP41),
            convert!(IP56), convert!(IP55), convert!(IP54), convert!(IP53),
            convert!(IP52), convert!(IP51), convert!(IP50), convert!(IP49),
            convert!(IP64), convert!(IP63), convert!(IP62), convert!(IP61),
            convert!(IP60), convert!(IP59), convert!(IP58), convert!(IP57)
    ];

    // Final Permutation Table changed to be 0-based in little endianness
    // So, LSB is 0-th bit and MSB is 63-rd bit in u64.
    const FP: [u8; 64] = make_FP!();

    // Expansion Permutation Table changed to be 0-based in little endianness
    // So, LSB is 0-th bit and MSB is 48-th bit in u48.
    const EP: [u8; 48] = [
        convert!(EP08), convert!(EP07), convert!(EP06), convert!(EP05),
        convert!(EP04), convert!(EP03), convert!(EP02), convert!(EP01),
        convert!(EP16), convert!(EP15), convert!(EP14), convert!(EP13),
        convert!(EP12), convert!(EP11), convert!(EP10), convert!(EP09),
        convert!(EP24), convert!(EP23), convert!(EP22), convert!(EP21),
        convert!(EP20), convert!(EP19), convert!(EP18), convert!(EP17),
        convert!(EP32), convert!(EP31), convert!(EP30), convert!(EP29),
        convert!(EP28), convert!(EP27), convert!(EP26), convert!(EP25),
        convert!(EP40), convert!(EP39), convert!(EP38), convert!(EP37),
        convert!(EP36), convert!(EP35), convert!(EP34), convert!(EP33),
        convert!(EP48), convert!(EP47), convert!(EP46), convert!(EP45),
        convert!(EP44), convert!(EP43), convert!(EP42), convert!(EP41)
    ];

    // Initial Permutation Table changed to be 0-based in little endianness
    // So, LSB is 0-th bit and MSB is 63-rd bit in u64.
    const TP: [u8; 32] = [
        convert!(TP08), convert!(TP07), convert!(TP06), convert!(TP05),
        convert!(TP04), convert!(TP03), convert!(TP02), convert!(TP01),
        convert!(TP16), convert!(TP15), convert!(TP14), convert!(TP13),
        convert!(TP12), convert!(TP11), convert!(TP10), convert!(TP09),
        convert!(TP24), convert!(TP23), convert!(TP22), convert!(TP21),
        convert!(TP20), convert!(TP19), convert!(TP18), convert!(TP17),
        convert!(TP32), convert!(TP31), convert!(TP30), convert!(TP29),
        convert!(TP28), convert!(TP27), convert!(TP26), convert!(TP25)
    ];

    const PC1: [u8; 56] = [
        convert!(PC108), convert!(PC107), convert!(PC106), convert!(PC105),
        convert!(PC104), convert!(PC103), convert!(PC102), convert!(PC101),
        convert!(PC116), convert!(PC115), convert!(PC114), convert!(PC113),
        convert!(PC112), convert!(PC111), convert!(PC110), convert!(PC109),
        convert!(PC124), convert!(PC123), convert!(PC122), convert!(PC121),
        convert!(PC120), convert!(PC119), convert!(PC118), convert!(PC117),
        convert!(PC132), convert!(PC131), convert!(PC130), convert!(PC129),
        convert!(PC128), convert!(PC127), convert!(PC126), convert!(PC125),
        convert!(PC140), convert!(PC139), convert!(PC138), convert!(PC137),
        convert!(PC136), convert!(PC135), convert!(PC134), convert!(PC133),
        convert!(PC148), convert!(PC147), convert!(PC146), convert!(PC145),
        convert!(PC144), convert!(PC143), convert!(PC142), convert!(PC141),
        convert!(PC156), convert!(PC155), convert!(PC154), convert!(PC153),
        convert!(PC152), convert!(PC151), convert!(PC150), convert!(PC149)
    ];

    const PC2: [u8; 48] = [
        convert!(PC208), convert!(PC207), convert!(PC206), convert!(PC205),
        convert!(PC204), convert!(PC203), convert!(PC202), convert!(PC201),
        convert!(PC216), convert!(PC215), convert!(PC214), convert!(PC213),
        convert!(PC212), convert!(PC211), convert!(PC210), convert!(PC209),
        convert!(PC224), convert!(PC223), convert!(PC222), convert!(PC221),
        convert!(PC220), convert!(PC219), convert!(PC218), convert!(PC217),
        convert!(PC232), convert!(PC231), convert!(PC230), convert!(PC229),
        convert!(PC228), convert!(PC227), convert!(PC226), convert!(PC225),
        convert!(PC240), convert!(PC239), convert!(PC238), convert!(PC237),
        convert!(PC236), convert!(PC235), convert!(PC234), convert!(PC233),
        convert!(PC248), convert!(PC247), convert!(PC246), convert!(PC245),
        convert!(PC244), convert!(PC243), convert!(PC242), convert!(PC241)
    ];

    /// Constructs a new object DES_Generic.
    /// 
    #[inline]
    pub fn new() -> Self
    {
        Self::new_with_key([0_u8; 8])
    }

    /// Constructs a new object DES_Generic.
    /// 
    pub fn new_with_key(key: [u8; 8]) -> Self
    {
        let mut des = Self
        {
            key:        LongUnion::new_with_ubytes(key),
            block:      LongUnion::new(),
            round_key:  [0_u64; ROUND],
        };
        des.make_round_keys();
        des
    }

    /// Constructs a new object DES_Generic.
    /// 
    pub fn new_with_key_u64(key: u64) -> Self
    {
        let mut des = Self
        {
            key:        LongUnion::new_with(key),
            block:      LongUnion::new(),
            round_key:  [0_u64; ROUND],
        };
        des.make_round_keys();
        des
    }

    pub fn set_key(&mut self, key: [u8; 8])
    {
        let mut i = 0_usize;
        for val in key
        {
            self.key.set_ubyte_(i, val);
            i += 1;
        }
    }

    #[inline]
    pub fn set_key_u64(&mut self, key: u64)
    {
        self.key.set(key);
    }

    pub fn encrypt_with_padding_pkcs7(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded: u64;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                encoded = self.encrypt_u64(block);
                unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0x_08_08_08_08__08_08_08_08);
        if progress != length_in_bytes
        {
            if progress != 0
                { progress -= 8; }
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let padding = 8 - tail as u8;
            block_union.set(block);
            for i in tail..8
                { block_union.set_ubyte_(i, padding); }
        }
        encoded = self.encrypt_u64(block_union.get());
        unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
        progress + 8
    }

    pub fn decrypt_with_padding_pkcs7(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        for _ in 0..length_in_bytes as usize / 8 - 1
        {
            block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
            decoded = self.decrypt_u64(block);
            unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), 8); }
            progress += 8;
        }
        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        let padding_bytes = decoded_union.get_ubyte_(7);
        let message_bytes = 8 - padding_bytes as usize;
        for i in (message_bytes)..8
        {
            if decoded_union.get_ubyte_(i) != padding_bytes
                { return 0; }
        }
        unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), message_bytes); }
        progress + message_bytes as u64
    }

    pub fn encrypt_with_padding_iso(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded: u64;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                encoded = self.encrypt_u64(block);
                unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }
        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0b_1000_0000);
        if progress != length_in_bytes
        {
            if progress != 0
                { progress -= 8; }
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            block_union.set(block);
            block_union.set_ubyte_(tail, 0b_1000_0000);
        }
        encoded = self.encrypt_u64(block_union.get());
        unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
        progress + 8
    }

    pub fn decrypt_with_padding_iso(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        for i in 0..(length_in_bytes as usize / 8 - 1)
        {
            block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
            decoded = self.decrypt_u64(block);
            unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), 8); }
            progress += 8;
        }

        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        for i in 0..8_usize
        {
            if decoded_union.get_ubyte_(7-i) == 0
                { continue; }
            if decoded_union.get_ubyte_(7-i) == 0b_1000_0000_u8
            {
                let message_bytes = 7-i;
                unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), message_bytes); }
                return progress + message_bytes as u64
            }
            else
            {
                return 0;
            }
        }
        return 0;
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7<T, const M: usize>(&mut self, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso<T, const M: usize>(&mut self, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso( message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso( message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_with_padding_pkcs7_ecb(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_ecb(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.decrypt_with_padding_pkcs7(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn encrypt_with_padding_iso_ecb(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_ecb(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.decrypt_with_padding_iso(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_ecb(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_ecb(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_ecb(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_ecb(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_ecb<T, const M: usize>(&mut self, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_ecb<T, const M: usize>(&mut self, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso( message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_ecb<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_ecb<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso( message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    pub fn encrypt_with_padding_pkcs7_cbc(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                encoded = self.encrypt_u64(block ^ encoded);
                unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }
        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0x_08_08_08_08__08_08_08_08);
        if progress != length_in_bytes
        {
            if progress != 0
                { progress -= 8; }
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let padding = 8 - tail as u8;
            block_union.set(block);
            for i in tail..8
                { block_union.set_ubyte_(i, padding); }
        }
        encoded = self.encrypt_u64(block_union.get() * encoded);
        unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
        progress + 8
    }

    pub fn decrypt_with_padding_pkcs7_cbc(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        for _ in 0..length_in_bytes as usize / 8 - 1
        {
            block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
            decoded = iv ^ self.decrypt_u64(block);
            iv = block;
            unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), 8); }
            progress += 8;
        }
        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = iv ^ self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        let padding_bytes = decoded_union.get_ubyte_(7);
        let message_bytes = 8 - padding_bytes as usize;
        for i in (message_bytes)..8
        {
            if decoded_union.get_ubyte_(i) != padding_bytes
                { return 0; }
        }
        unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), message_bytes); }
        progress + message_bytes as u64
    }

    pub fn encrypt_with_padding_iso_cbc(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                encoded = self.encrypt_u64(block ^ encoded);
                unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }
        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0b_1000_0000);
        if progress != length_in_bytes
        {
            if progress != 0
                { progress -= 8; }
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            block_union.set(block);
            block_union.set_ubyte_(tail, 0b_1000_0000);
        }
        encoded = self.encrypt_u64(block_union.get() ^ encoded);
        unsafe { copy_nonoverlapping(&mut encoded as *mut u64 as *mut u8, cipher.add(progress as usize), 8); }
        progress + 8
    }

    pub fn decrypt_with_padding_iso_cbc(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        for i in 0..(length_in_bytes as usize / 8 - 1)
        {
            block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
            decoded = iv ^ self.decrypt_u64(block);
            iv = block;
            unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), 8); }
            progress += 8;
        }

        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = iv ^ self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        for i in 0..8_usize
        {
            if decoded_union.get_ubyte_(7-i) == 0
                { continue; }
            if decoded_union.get_ubyte_(7-i) == 0b_1000_0000_u8
            {
                let message_bytes = 7-i;
                unsafe { copy_nonoverlapping(&mut decoded as *mut u64 as *mut u8, message.add(progress as usize), message_bytes); }
                return progress + message_bytes as u64
            }
            else
            {
                return 0;
            }
        }
        return 0;
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_cbc(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_cbc(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_cbc(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_cbc(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_cbc<T, const M: usize>(&mut self, iv: u64, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_cbc<T, const M: usize>(&mut self, iv: u64, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_cbc<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_cbc<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    pub fn encrypt_cfb(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                encoded = block ^ self.encrypt_u64(encoded);
                unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let mut tail = 8_usize;
        let mut block: u64;
        if progress + 8 == length_in_bytes
        {
            block = unsafe { *(message.add(progress as usize - 8) as *const u64 ) };
        }
        else
        {
            block = 0_u64;
            tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
        }
        encoded = block ^ self.encrypt_u64(encoded);
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), tail); }
        progress + tail as u64
    }

    pub fn decrypt_cfb(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                let decoded = block ^ self.encrypt_u64(iv);
                iv = block;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let mut tail = 8_usize;
        let mut block: u64;
        if progress + 8 == length_in_bytes
        {
            block = unsafe { *(message.add(progress as usize - 8) as *const u64 ) };
        }
        else
        {
            block = 0_u64;
            tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
        }
        let decoded = block ^ self.encrypt_u64(iv);
        unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), tail); }
        progress + tail as u64
    }

    #[inline]
    pub fn encrypt_str_cfb(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_cfb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_cfb(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_cfb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_cfb<T, const M: usize>(&mut self, iv: u64, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb(iv, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_cfb<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb(iv, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    pub fn crypt_ofb(&mut self, mut iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                iv = self.encrypt_u64(iv);
                let coded = block ^ iv;
                unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let mut tail = 8_usize;
        let mut block: u64;
        if progress + 8 == length_in_bytes
        {
            block = unsafe { *(message.add(progress as usize - 8) as *const u64 ) };
        }
        else
        {
            block = 0_u64;
            tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
        }
        let coded = block ^ self.encrypt_u64(iv);
        unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, cipher.add(progress as usize), tail); }
        progress + tail as u64
    }

    #[inline]
    pub fn encrypt_ofb(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.crypt_ofb(iv, message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn decrypt_ofb(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.crypt_ofb(iv, cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn encrypt_str_ofb(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_ofb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ofb(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_ofb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ofb<T, const M: usize>(&mut self, iv: u64, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb(iv, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ofb<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb(iv, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    fn crypt_ctr(&mut self, mut nonce: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        nonce = nonce.wrapping_add(1);
        if length_in_bytes > progress + 8
        {
            while length_in_bytes > progress
            {
                let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
                let coded = block ^ self.encrypt_u64(nonce);
                nonce = nonce.wrapping_add(1);
                unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let mut tail = 8_usize;
        let mut block: u64;
        if progress + 8 == length_in_bytes
        {
            block = unsafe { *(message.add(progress as usize - 8) as *const u64 ) };
        }
        else
        {
            block = 0_u64;
            tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
        }
        let coded = block ^ self.encrypt_u64(nonce);
        unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, cipher.add(progress as usize), tail); }
        progress + tail as u64
    }

    #[inline]
    pub fn encrypt_ctr(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.crypt_ctr(nonce, message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn decrypt_ctr(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.crypt_ctr(nonce, cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn encrypt_str_ctr(&mut self, nonce: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_ctr(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ctr(&mut self, nonce: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_ctr(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ctr<T, const M: usize>(&mut self, nonce: u64, message: &[T; M], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr(nonce, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ctr<T>(&mut self, nonce: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr(nonce, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64, cipher)
    }

    pub fn encrypt_array_u64<const N: usize>(&mut self, message: &[u64; N], cipher: &mut [u64; N])
    {
        for i in 0..N
        {
            self.set_block(message[i]);
            self.encrypt_block();
            cipher[i] = self.get_block();
        }
    }

    pub fn decrypt_array_u64<const N: usize>(&mut self, cipher: &[u64; N], message: &mut [u64; N])
    {
        for i in 0..N
        {
            self.set_block(cipher[i]);
            self.decrypt_block();
            message[i] = self.get_block();
        }
    }

    pub fn encrypt_u64(&mut self, message: u64) -> u64
    {
        self.set_block(message);
        self.encrypt_block();
        self.get_block()
    }

    pub fn decrypt_u64(&mut self, cioher: u64) -> u64
    {
        self.set_block(cioher);
        self.decrypt_block();
        self.get_block()
    }

    fn encrypt_block(&mut self)
    {
        self.permutate_initially();
        for round in 0..ROUND
            { self.feistel(round); }
        let left = self.block.get_uint_(0);
        let right = self.block.get_uint_(1);
        self.block.set_uint_(0, right);
        self.block.set_uint_(1, left);
        self.permutate_finally();
    }

    fn decrypt_block(&mut self)
    {
        self.permutate_initially();
        for round in 0..ROUND
            { self.feistel(ROUND - 1 - round); }
        let left = self.block.get_uint_(0);
        let right = self.block.get_uint_(1);
        self.block.set_uint_(0, right);
        self.block.set_uint_(1, left);
        self.permutate_finally();
    }

    #[allow(dead_code)]
    fn feistel(&mut self, round: usize)
    {
        const LEFT: usize = 0;
        const RIGHT: usize = 1;
        let right = self.block.get_uint_(RIGHT);
        let left = self.block.get_uint_(LEFT) ^ self.f(round, right);
        self.block.set_uint_(LEFT, right);
        self.block.set_uint_(RIGHT, left);
    }

    #[allow(dead_code)]
    fn f(&mut self, round: usize, right: u32) -> u32
    {
        let expanded = self.expand(right);
        let indices = expanded ^ self.round_key[round];
        let mut idx = [0_usize; 8];
        slice_index!(indices, idx);
        let mut out = 0_u32;
        for i in 0..4
        {
            let left = i * 2;
            let right = left + 1;
            combine_pieces!(out, ((Self::SBOX[left][idx[left]] << 4) | Self::SBOX[right][idx[right]]) as u32);
        }
        self.translate(out)
    }

    fn make_round_keys(&mut self)
    {
        let (mut left, mut right) = self.split();
        let mut shift = SHIFT;
        for i in 0..ROUND
        {
            rotate_halfkey!(left, shift.is_even());
            rotate_halfkey!(right, shift.is_even());
            self.make_a_round_key(i, left, right);
            shift >>= 1;
        }
    }

    fn make_a_round_key(&mut self, round: usize, left: IntUnion, mut right: IntUnion)
    {
        let tail = right.get_ubyte_(0) >> 4;
        shift_left_union!(right, 4);
        let mut whole = LongUnion::new_with_uints([left.get(), right.get()]);
        whole.set_ubyte_(3, whole.get_ubyte_(3) | tail);
        self.round_key[round] = self.compress_into_48bits(whole.get());
    }

    fn split(&self) -> (IntUnion, IntUnion)
    {
        let key_56bit = self.compress_into_56bits();
        let key = LongUnion::new_with(key_56bit);
        let mut left = IntUnion::new_with(key.get_uint_(0));
        left.set_ubyte_(3, left.get_ubyte_(3) & 0b_1111_0000);
        let mut right = IntUnion::new_with(key.get_uint_(1));
        shift_right_union!(right, 4);
        right.set_ubyte_(0, (key.get_ubyte_(3) << 4) | right.get_ubyte_(0));
        (left, right)
    }

    fn permutate_initially(&mut self)   { permutate_data!(self, Self::IP); }
    fn permutate_finally(&mut self)     { permutate_data!(self, Self::FP); }
    fn compress_into_56bits(&self) -> u64   { return permutate_data!(Self::PC1, u64, self.key.get()); }
    fn compress_into_48bits(&self, whole: u64) -> u64   { return permutate_data!(Self::PC2, u64, whole); }
    fn expand(&self, right: u32) -> u64     { return permutate_data!(Self::EP, u64, right);}
    fn translate(&self, right: u32) -> u32  { return permutate_data!(Self::TP, u32, right); }

    #[inline] fn get_block(&self) -> u64            { self.block.get() }
    #[inline] fn set_block(&mut self, block: u64)   { self.block.set(block); }




    ////// for unit test /////
    // #[inline] pub fn test_get_block(&self) -> u64           { self.block.get() }
    // #[inline] pub fn test_set_block(&mut self, block: u64)  { self.block.set(block); }
    //
    // pub fn test_set_key(&mut self, key: [u8; 8])
    // {
    //     for i in 0..8
    //         { self.key.set_ubyte_(i, key[i]); }
    // }
    //
    // pub fn test_permutate_initially(&mut self)    { permutate_data!(self, Self::IP); }
    // pub fn test_permutate_finally(&mut self)      { permutate_data!(self, Self::FP); }
    // #[inline] pub fn test_expand(&self, right: u32) -> u64  { self.expand(right) }
    // #[inline] pub fn test_compress_into_56bits(&self) -> u64    { self.compress_into_56bits() }
    // #[inline] pub fn test_split(&self) -> (IntUnion, IntUnion)  { self.split() }
    // #[inline] pub fn test_make_round_keys(&mut self)    { self.make_round_keys(); }
    // #[inline] pub fn test_get_round_key(&self, round: usize) -> u64  { self.round_key[round] }
    // pub fn test_slice_indices(&self, indices: u64, array: &mut [usize; 8])   { slice_index!(indices, array); }
    // pub fn test_combine(&self, collector: &mut u32, piece: u32) { combine_pieces!(*collector, piece); }
    // pub fn test_f(&mut self, round: usize, right: u32) -> u32   { self.f(round, right) }
}
