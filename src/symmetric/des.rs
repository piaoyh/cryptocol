// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]


// use std::ptr::copy_nonoverlapping;
// use std::slice::from_raw_parts;
// use std::fmt::{ self, Debug, Display, Formatter };
// use std::collections::HashMap;

use crate::number::{ SmallUInt, LongUnion };

macro_rules! make_FP {
    () => {
        if true
        {
            let mut out = [0_u8; 64];
            out[Self::IP[0] as usize] = 0;
            out[Self::IP[1] as usize] = 1;
            out[Self::IP[2] as usize] = 2;
            out[Self::IP[3] as usize] = 3;
            out[Self::IP[4] as usize] = 4;
            out[Self::IP[5] as usize] = 5;
            out[Self::IP[6] as usize] = 6;
            out[Self::IP[7] as usize] = 7;
            out[Self::IP[8] as usize] = 8;
            out[Self::IP[9] as usize] = 9;
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
            let out = [0_u8; 64];
            out
        }
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
/// - PC101 ~ PC248: 
/// - IP01 ~ IP64: Inital permutation constants, and is 1-based.
/// For example, `IP01 = 58` means that the 58th bit of data from the first
/// bit is moved to the first bit of the data at initial permutation.
/// You can change inital permutation wire by changing these constants
/// The change of these constants does not change the security level.
/// Final permutation constants is automatically calculated from Inital
/// permutation constants.
/// - S000 ~ S763: S-Box constants, and its index such as 000, 212, etc. is
/// 0-based. S0XX means S-Box 0, S1XX means S-Box 1, and so on. S000 is the
/// first element of S-Box 0. You cange S-Box by changing these constants.
/// The change of these constants DOES change the security level a lot.
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
pub struct DES_Generic<
const PC101: u8 = 57, const PC102: u8 = 49, const PC103: u8 = 41, const PC104: u8 = 33,
const PC105: u8 = 25, const PC106: u8 = 17, const PC107: u8 = 9,  const PC108: u8 = 1,
const PC109: u8 = 58, const PC110: u8 = 50, const PC111: u8 = 42, const PC112: u8 = 34,
const PC113: u8 = 26, const PC114: u8 = 18, const PC115: u8 = 10, const PC116: u8 = 2,
const PC117: u8 = 59, const PC118: u8 = 51, const PC119: u8 = 43, const PC120: u8 = 35,
const PC121: u8 = 27, const PC122: u8 = 19, const PC123: u8 = 11, const PC124: u8 = 3,
const PC125: u8 = 60, const PC126: u8 = 52, const PC127: u8 = 44, const PC128: u8 = 36,
const PC129: u8 = 63, const PC130: u8 = 55, const PC131: u8 = 47, const PC132: u8 = 39,
const PC133: u8 = 31, const PC134: u8 = 23, const PC135: u8 = 15, const PC136: u8 = 7,
const PC137: u8 = 62, const PC138: u8 = 54, const PC139: u8 = 46, const PC140: u8 = 38,
const PC141: u8 = 30, const PC142: u8 = 22, const PC143: u8 = 14, const PC144: u8 = 6,
const PC145: u8 = 61, const PC146: u8 = 53, const PC147: u8 = 45, const PC148: u8 = 37,
const PC149: u8 = 29, const PC150: u8 = 21, const PC151: u8 = 13, const PC152: u8 = 5,
const PC153: u8 = 28, const PC154: u8 = 20, const PC155: u8 = 12, const PC156: u8 = 4,
const PC201: u8 = 14, const PC202: u8 = 17, const PC203: u8 = 11, const PC204: u8 = 24,
const PC205: u8 = 1,  const PC206: u8 = 5,  const PC207: u8 = 3,  const PC208: u8 = 28,
const PC209: u8 = 15, const PC210: u8 = 6,  const PC211: u8 = 21, const PC212: u8 = 10,
const PC213: u8 = 23, const PC214: u8 = 19, const PC215: u8 = 12, const PC216: u8 = 4,
const PC217: u8 = 26, const PC218: u8 = 8,  const PC219: u8 = 16, const PC220: u8 = 7,
const PC221: u8 = 27, const PC222: u8 = 20, const PC223: u8 = 13, const PC224: u8 = 2,
const PC225: u8 = 41, const PC226: u8 = 52, const PC227: u8 = 31, const PC228: u8 = 37,
const PC229: u8 = 47, const PC230: u8 = 55, const PC231: u8 = 30, const PC232: u8 = 40,
const PC233: u8 = 51, const PC234: u8 = 45, const PC235: u8 = 33, const PC236: u8 = 48,
const PC237: u8 = 44, const PC238: u8 = 49, const PC239: u8 = 39, const PC240: u8 = 56,
const PC241: u8 = 34, const PC242: u8 = 53, const PC243: u8 = 46, const PC244: u8 = 42,
const PC245: u8 = 50, const PC246: u8 = 36, const PC247: u8 = 29, const PC248: u8 = 32,
const IP01: u8 = 58, const IP02: u8 = 50, const IP03: u8 = 42, const IP04: u8 = 34,
const IP05: u8 = 26, const IP06: u8 = 18, const IP07: u8 = 10, const IP08: u8 = 2,
const IP09: u8 = 60, const IP10: u8 = 52, const IP11: u8 = 44, const IP12: u8 = 36,
const IP13: u8 = 28, const IP14: u8 = 20, const IP15: u8 = 12, const IP16: u8 = 4,
const IP17: u8 = 62, const IP18: u8 = 54, const IP19: u8 = 46, const IP20: u8 = 38,
const IP21: u8 = 30, const IP22: u8 = 22, const IP23: u8 = 14, const IP24: u8 = 6,
const IP25: u8 = 64, const IP26: u8 = 56, const IP27: u8 = 48, const IP28: u8 = 40,
const IP29: u8 = 32, const IP30: u8 = 24, const IP31: u8 = 16, const IP32: u8 = 8,
const IP33: u8 = 57, const IP34: u8 = 49, const IP35: u8 = 41, const IP36: u8 = 33,
const IP37: u8 = 25, const IP38: u8 = 17, const IP39: u8 = 9,  const IP40: u8 = 1,
const IP41: u8 = 59, const IP42: u8 = 51, const IP43: u8 = 43, const IP44: u8 = 35,
const IP45: u8 = 27, const IP46: u8 = 19, const IP47: u8 = 11, const IP48: u8 = 3,
const IP49: u8 = 61, const IP50: u8 = 53, const IP51: u8 = 45, const IP52: u8 = 37,
const IP53: u8 = 29, const IP54: u8 = 21, const IP55: u8 = 13, const IP56: u8 = 5,
const IP57: u8 = 63, const IP58: u8 = 55, const IP59: u8 = 47, const IP60: u8 = 39,
const IP61: u8 = 31, const IP62: u8 = 23, const IP63: u8 = 15, const IP64: u8 = 7,
// https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
// https://m.blog.naver.com/wnrjsxo/221708511553
// const FP01: u8 = 40, const FP02: u8 = 8,  const FP03: u8 = 48, const FP04: u8 = 16,
// const FP05: u8 = 56, const FP06: u8 = 24, const FP07: u8 = 64, const FP08: u8 = 32,
// const FP09: u8 = 39, const FP10: u8 = 7,  const FP11: u8 = 47, const FP12: u8 = 15,
// const FP13: u8 = 55, const FP14: u8 = 23, const FP15: u8 = 63, const FP16: u8 = 31,
// const FP17: u8 = 38, const FP18: u8 = 6,  const FP19: u8 = 46, const FP20: u8 = 14,
// const FP21: u8 = 54, const FP22: u8 = 22, const FP23: u8 = 62, const FP24: u8 = 30,
// const FP25: u8 = 37, const FP26: u8 = 5,  const FP27: u8 = 45, const FP28: u8 = 13,
// const FP29: u8 = 53, const FP30: u8 = 21, const FP31: u8 = 61, const FP32: u8 = 29,
// const FP33: u8 = 36, const FP34: u8 = 4,  const FP35: u8 = 44, const FP36: u8 = 12,
// const FP37: u8 = 52, const FP38: u8 = 20, const FP39: u8 = 60, const FP40: u8 = 28,
// const FP41: u8 = 35, const FP42: u8 = 3,  const FP43: u8 = 43, const FP44: u8 = 11,
// const FP45: u8 = 51, const FP46: u8 = 19, const FP47: u8 = 59, const FP48: u8 = 27,
// const FP49: u8 = 34, const FP50: u8 = 2,  const FP51: u8 = 42, const FP52: u8 = 10,
// const FP53: u8 = 50, const FP54: u8 = 18, const FP55: u8 = 58, const FP56: u8 = 26,
// const FP57: u8 = 33, const FP58: u8 = 1,  const FP59: u8 = 41, const FP60: u8 = 9,
// const FP61: u8 = 49, const FP62: u8 = 17, const FP63: u8 = 57, const FP64: u8 = 25,
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
const S760: u8 = 0x5, const S761: u8 = 0x6, const S762: u8 = 0x8, const S763: u8 = 0xb,
>
{
#[allow(dead_code)]
    key: LongUnion,
#[allow(dead_code)]
    block: LongUnion,
}

impl <
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
// const FP01: u8, const FP02: u8, const FP03: u8, const FP04: u8,
// const FP05: u8, const FP06: u8, const FP07: u8, const FP08: u8,
// const FP09: u8, const FP10: u8, const FP11: u8, const FP12: u8,
// const FP13: u8, const FP14: u8, const FP15: u8, const FP16: u8,
// const FP17: u8, const FP18: u8, const FP19: u8, const FP20: u8,
// const FP21: u8, const FP22: u8, const FP23: u8, const FP24: u8,
// const FP25: u8, const FP26: u8, const FP27: u8, const FP28: u8,
// const FP29: u8, const FP30: u8, const FP31: u8, const FP32: u8,
// const FP33: u8, const FP34: u8, const FP35: u8, const FP36: u8,
// const FP37: u8, const FP38: u8, const FP39: u8, const FP40: u8,
// const FP41: u8, const FP42: u8, const FP43: u8, const FP44: u8,
// const FP45: u8, const FP46: u8, const FP47: u8, const FP48: u8,
// const FP49: u8, const FP50: u8, const FP51: u8, const FP52: u8,
// const FP53: u8, const FP54: u8, const FP55: u8, const FP56: u8,
// const FP57: u8, const FP58: u8, const FP59: u8, const FP60: u8,
// const FP61: u8, const FP62: u8, const FP63: u8, const FP64: u8,
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
const S760: u8, const S761: u8, const S762: u8, const S763: u8,
>
DES_Generic<
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
// FP01, FP02, FP03, FP04, FP05, FP06, FP07, FP08,
// FP09, FP10, FP11, FP12, FP13, FP14, FP15, FP16,
// FP17, FP18, FP19, FP20, FP21, FP22, FP23, FP24,
// FP25, FP26, FP27, FP28, FP29, FP30, FP31, FP32,
// FP33, FP34, FP35, FP36, FP37, FP38, FP39, FP40,
// FP41, FP42, FP43, FP44, FP45, FP46, FP47, FP48,
// FP49, FP50, FP51, FP52, FP53, FP54, FP55, FP56,
// FP57, FP58, FP59, FP60, FP61, FP62, FP63, FP64,
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
S756, S757, S758, S759, S760, S761, S762, S763,
>
{
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    const IP: [u8; 64] = [
              64-IP01, 64-IP02, 64-IP03, 64-IP04, 64-IP05, 64-IP06, 64-IP07, 64-IP08,
              64-IP09, 64-IP10, 64-IP11, 64-IP12, 64-IP13, 64-IP14, 64-IP15, 64-IP16,
              64-IP17, 64-IP18, 64-IP19, 64-IP20, 64-IP21, 64-IP22, 64-IP23, 64-IP24,
              64-IP25, 64-IP26, 64-IP27, 64-IP28, 64-IP29, 64-IP30, 64-IP31, 64-IP32,
              64-IP33, 64-IP34, 64-IP35, 64-IP36, 64-IP37, 64-IP38, 64-IP39, 64-IP40,
              64-IP41, 64-IP42, 64-IP43, 64-IP44, 64-IP45, 64-IP46, 64-IP47, 64-IP48,
              64-IP49, 64-IP50, 64-IP51, 64-IP52, 64-IP53, 64-IP54, 64-IP55, 64-IP56,
              64-IP57, 64-IP58, 64-IP59, 64-IP60, 64-IP61, 64-IP62, 64-IP63, 64-IP64
    ];
    #[allow(dead_code)]
    const FP: [u8; 64] = make_FP!();
    // const FP: [u8; 64] = [  
    //           64-FP01, 64-FP02, 64-FP03, 64-FP04, 64-FP05, 64-FP06, 64-FP07, 64-FP08,
    //           64-FP09, 64-FP10, 64-FP11, 64-FP12, 64-FP13, 64-FP14, 64-FP15, 64-FP16,
    //           64-FP17, 64-FP18, 64-FP19, 64-FP20, 64-FP21, 64-FP22, 64-FP23, 64-FP24,
    //           64-FP25, 64-FP26, 64-FP27, 64-FP28, 64-FP29, 64-FP30, 64-FP31, 64-FP32,
    //           64-FP33, 64-FP34, 64-FP35, 64-FP36, 64-FP37, 64-FP38, 64-FP39, 64-FP40,
    //           64-FP41, 64-FP42, 64-FP43, 64-FP44, 64-FP45, 64-FP46, 64-FP47, 64-FP48,
    //           64-FP49, 64-FP50, 64-FP51, 64-FP52, 64-FP53, 64-FP54, 64-FP55, 64-FP56,
    //           64-FP57, 64-FP58, 64-FP59, 64-FP60, 64-FP61, 64-FP62, 64-FP63, 64-FP64
    // ];
    #[allow(dead_code)]
    const PC1: [u8; 56] = [
        64-PC101, 64-PC102, 64-PC103, 64-PC104, 64-PC105, 64-PC106, 64-PC107, 64-PC108,
        64-PC109, 64-PC110, 64-PC111, 64-PC112, 64-PC113, 64-PC114, 64-PC115, 64-PC116,
        64-PC117, 64-PC118, 64-PC119, 64-PC120, 64-PC121, 64-PC122, 64-PC123, 64-PC124,
        64-PC125, 64-PC126, 64-PC127, 64-PC128, 64-PC129, 64-PC130, 64-PC131, 64-PC132,
        64-PC133, 64-PC134, 64-PC135, 64-PC136, 64-PC137, 64-PC138, 64-PC139, 64-PC140,
        64-PC141, 64-PC142, 64-PC143, 64-PC144, 64-PC145, 64-PC146, 64-PC147, 64-PC148,
        64-PC149, 64-PC150, 64-PC151, 64-PC152, 64-PC153, 64-PC154, 64-PC155, 64-PC156,
    ];
    #[allow(dead_code)]
    const PC2: [u8; 48] = [
        64-PC201, 64-PC202, 64-PC203, 64-PC204, 64-PC205, 64-PC206, 64-PC207, 64-PC208,
        64-PC209, 64-PC210, 64-PC211, 64-PC212, 64-PC213, 64-PC214, 64-PC215, 64-PC216,
        64-PC217, 64-PC218, 64-PC219, 64-PC220, 64-PC221, 64-PC222, 64-PC223, 64-PC224,
        64-PC225, 64-PC226, 64-PC227, 64-PC228, 64-PC229, 64-PC230, 64-PC231, 64-PC232,
        64-PC233, 64-PC234, 64-PC235, 64-PC236, 64-PC237, 64-PC238, 64-PC239, 64-PC240,
        64-PC241, 64-PC242, 64-PC243, 64-PC244, 64-PC245, 64-PC246, 64-PC247, 64-PC248,
    ];
    /// Constructs a new object DES_Generic.
    /// 
    #[inline]
    #[allow(dead_code)]
    pub fn new(key: [u8; 8]) -> Self
    {
        Self { key: LongUnion::new_with_ubytes(key), block: LongUnion::new(), }
    }

    #[allow(dead_code)]
    fn permutate_initially(&mut self)
    {
        let data = self.block.get().to_be();
        let mut permuted = 0_u64;
        for pos in Self::IP
        {
            permuted <<= 1;
            permuted |= data.is_bit_set_(pos as usize) as u64;
        }
        self.block.set(permuted.to_be());
    }

    #[allow(dead_code)]
    fn permutate_finally(&mut self)
    {
        let data = self.block.get().to_be();
        let mut permuted = 0_u64;
        for pos in Self::FP
        {
            permuted <<= 1;
            permuted |= data.is_bit_set_(pos as usize) as u64;
        }
        self.block.set(permuted.to_be());
    }

    #[allow(dead_code)]
    fn permutate_key1(&mut self)
    {
        let key = self.key.get().to_be();
        let mut permuted = 0_u64;
        for pos in Self::PC1
        {
            permuted <<= 1;
            permuted |= key.is_bit_set_(pos as usize) as u64;
        }
        self.key.set(permuted.to_be());
    }

    #[allow(dead_code)]
    fn permutate_key2(&self, key: &[u8; 7]) -> [u8; 6]
    {
        let mut key_union = LongUnion::new();
        for i in 0..7
            { key_union.set_ubyte_(i, key[i]); }
        let key_bar = key_union.get().to_be();
        let mut permuted = 0_u64;
        for pos in Self::PC2
        {
            permuted <<= 1;
            permuted |= key_bar.is_bit_set_(pos as usize) as u64;
        }
        let key_out = LongUnion::new_with((permuted << 16).to_be());
        let mut out = [0_u8; 6];
        for i in 0..6
            { out[i] = key_out.get_ubyte_(i); }
        out
    }


}
