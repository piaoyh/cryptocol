// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]

///// For test
// use cryptocol::symmetric::DES;
// use cryptocol::number::{ IntUnion };
//
// trait TestDes
// {
//     fn get_block(&self) -> u64;
//     fn set_block(&mut self, block: u64);
//     fn permutate_initially(&mut self);
//     fn permutate_finally(&mut self);
//     fn expand(&self, right: u32) -> u64;
//     fn compress_into_56bits(&self) -> u64;
//     fn split(&self) -> (IntUnion, IntUnion);
//     fn make_round_keys(&mut self);
//     fn get_round_key(&self, round: usize) -> u64;
//     fn slice_indices(&self, indices: u64, array: &mut [usize; 8]);
//     fn combine(&self, collector: &mut u32, piece: u32);
//     fn f(&mut self, round: usize, right: u32) -> u32;
// }
//
// impl TestDes for DES
// {
//     fn get_block(&self) -> u64          { self.test_get_block() }
//     fn set_block(&mut self, block: u64) { self.test_set_block(block); }
//     fn permutate_initially(&mut self)   { self.test_permutate_initially(); }
//     fn permutate_finally(&mut self)     { self.test_permutate_finally(); }
//     fn expand(&self, right: u32) -> u64     { self.test_expand(right) }
//     fn compress_into_56bits(&self) -> u64   { self.test_compress_into_56bits() }
//     fn split(&self) -> (IntUnion, IntUnion)     { self.test_split() }
//     fn make_round_keys(&mut self)    { self.test_make_round_keys(); }
//     fn get_round_key(&self, round: usize) -> u64  { self.test_get_round_key(round) }
//     fn slice_indices(&self, indices: u64, array: &mut [usize; 8])   { self.test_slice_indices(indices, array) }
//     fn combine(&self, collector: &mut u32, piece: u32) { self.test_combine(collector, piece); }
//     fn f(&mut self, round: usize, right: u32) -> u32   { self.test_f(round, right) }
// }

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]
pub fn main()
{
    // des_private_functions_main();
    des_encrypt_decrypt_u64_array_u64_main();
    des_crypt_with_padding_pkcs7_main();
    des_crypt_with_padding_iso_main();
    des_crypt_with_padding_pkcs7_ecb_main();
    des_crypt_with_padding_iso_ecb_main();
    des_crypt_with_padding_pkcs7_cbc_main();
    des_crypt_with_padding_iso_cbc_main();
    des_crypt_with_padding_pkcs7_pcbc_main();
    des_crypt_with_padding_iso_pcbc_main();
    des_crypt_cfb_main();
    des_crypt_ofb_main();
    des_crypt_ctr_main();
}

// fn des_private_functions_main()
// {
//     des_permutate_initially_finally();
//     des_permutate_expansion();
//     des_split();
//     des_make_round_keys();
//     des_slice_indices_combine();
//     des_f();
// }

// fn des_permutate_initially_finally()
// {
//     println!("des_permutate_initially_finally");
//     use std::fmt::Write;
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;

//     let mut a_des = DES::new();
//     let block = (1_u64 << (8-2)) | (1_u64 << ((50-1) / 8 * 8 + (7 - (50-1) % 8)));
//     a_des.set_block(block);
//     a_des.permutate_initially();
//     let out = a_des.get_block();
//     let bu = LongUnion::new_with(block);
//     print!("block =\t");
//     for i in 0..8
//         { print!("{:08b} ", bu.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", bu.get_ubyte_(i)); }
//     assert_eq!(txt, "01000000 00000000 00000000 00000000 00000000 00000000 01000000 00000000 ");

//     let ou = LongUnion::new_with(out);
//     print!("out =\t");
//     for i in 0..8
//         { print!("{:08b} ", ou.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", ou.get_ubyte_(i)); }
//     assert_eq!(txt, "01000001 00000000 00000000 00000000 00000000 00000000 00000000 00000000 ");

//     a_des.permutate_finally();
//     let back = a_des.get_block();
//     let cu = LongUnion::new_with(back);
//     print!("back =\t");
//     for i in 0..8
//         { print!("{:08b} ", cu.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", cu.get_ubyte_(i)); }
//     assert_eq!(txt, "01000000 00000000 00000000 00000000 00000000 00000000 01000000 00000000 ");
//     println!("-------------------------------");
// }

// fn des_permutate_expansion()
// {
//     println!("des_permutate_expansion");
//     use std::fmt::Write;
//     use cryptocol::number::{ IntUnion, LongUnion };
//     use cryptocol::symmetric::DES;
    
//     let mut right = IntUnion::new();
//     let mut i = 0;
//     for val in [0b_1111_0000_u8, 0b_1010_1010, 0b_1111_0000, 0b_1010_1010]
//     {
//         right.set_ubyte_(i, val);
//         i += 1;
//     }
//     print!("right =\t");
//     for i in 0..4
//         { print!("{:08b} ", right.get_ubyte_(i)); }
//     println!();

//     let a_des = DES::new();
//     let out = a_des.expand(right.get());

//     let ou = LongUnion::new_with(out);
//     print!("out =\t");
//     for i in 0..6
//         { print!("{:08b} ", ou.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..6
//         { write!(txt, "{:08b} ", ou.get_ubyte_(i)); }
//     assert_eq!(txt, "01111010 00010101 01010101 01111010 00010101 01010101 ");
//     println!("-------------------------------");
// }

// fn des_split()
// {
//     println!("des_split");
//     use std::fmt::Write;
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;
    
//     let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
//     print!("K =\t");
//     for i in 0..8
//         { print!("{:08b} ", key[i]); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", key[i]); }
//     assert_eq!(txt, "00010011 00110100 01010111 01111001 10011011 10111100 11011111 11110001 ");

//     let a_des = DES::new_with_key(key.clone());
//     let key_56bit = LongUnion::new_with(a_des.compress_into_56bits());
//     print!("K+ =\t");
//     for i in 0..7
//         { print!("{:08b} ", key_56bit.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..7
//         { write!(txt, "{:08b} ", key_56bit.get_ubyte_(i)); }
//     assert_eq!(txt, "11110000 11001100 10101010 11110101 01010110 01100111 10001111 ");

//     let a_des = DES::new_with_key(key.clone());
//     let (left, right) = a_des.split();
//     print!("L =\t");
//     for i in 0..4
//         { print!("{:08b} ", left.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..4
//         { write!(txt, "{:08b} ", left.get_ubyte_(i)); }
//     assert_eq!(txt, "11110000 11001100 10101010 11110000 ");

//     print!("R =\t");
//     for i in 0..4
//         { print!("{:08b} ", right.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..4
//         { write!(txt, "{:08b} ", right.get_ubyte_(i)); }
//     assert_eq!(txt, "01010101 01100110 01111000 11110000 ");
//     println!("-------------------------------");
// }

// fn des_make_round_keys()
// {
//     println!("des_make_round_keys");
//     use std::fmt::Write;
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;

//     let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
//     print!("K =\t");
//     for i in 0..8
//         { print!("{:08b} ", key[i]); }
//     println!();

//     let a_des = DES::new_with_key(key);
//     for i in 0..16
//     {
//         let round_key = LongUnion::new_with(a_des.get_round_key(i));
//         print!("K({}) =\t", i);
//         for j in 0..6
//             { print!("{:08b} ", round_key.get_ubyte_(j)); }
//         println!();
//     }

//     let round_key = LongUnion::new_with(a_des.get_round_key(0));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "00011011 00000010 11101111 11111100 01110000 01110010 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(1));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01111001 10101110 11011001 11011011 11001001 11100101 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(2));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01010101 11111100 10001010 01000010 11001111 10011001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(3));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01110010 10101101 11010110 11011011 00110101 00011101 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(4));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01111100 11101100 00000111 11101011 01010011 10101000 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(5));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01100011 10100101 00111110 01010000 01111011 00101111 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(6));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11101100 10000100 10110111 11110110 00011000 10111100 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(7));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11110111 10001010 00111010 11000001 00111011 11111011 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(8));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11100000 11011011 11101011 11101101 11100111 10000001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(9));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "10110001 11110011 01000111 10111010 01000110 01001111 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(10));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "00100001 01011111 11010011 11011110 11010011 10000110 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(11));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01110101 01110001 11110101 10010100 01100111 11101001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(12));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "10010111 11000101 11010001 11111010 10111010 01000001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(13));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01011111 01000011 10110111 11110010 11100111 00111010 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(14));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "10111111 10010001 10001101 00111101 00111111 00001010 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(15));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11001011 00111101 10001011 00001110 00010111 11110101 ");
//     println!("-------------------------------");
// }

// fn des_slice_indices_combine()
// {
//     println!("des_slice_indices_combine");
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;

//     let a_des = DES::new();
//     let mut indices = LongUnion::new();
//     indices.set_ubyte_(0, 0b_111111_00);
//     indices.set_ubyte_(1, 0b_0000_1010);
//     indices.set_ubyte_(2, 0b_10_100100);
//     indices.set_ubyte_(3, 0b_010101_00);
//     indices.set_ubyte_(4, 0b_1001_1101);
//     indices.set_ubyte_(5, 0b_10_011011);

//     let mut index = [0_usize; 8];
//     a_des.slice_indices(indices.get(), &mut index);
//     for i in 0..8
//         { println!("idx({}) = {:06b}", i, index[i]); }
//     assert_eq!(index[0], 0b111111);
//     assert_eq!(index[1], 0b000000);
//     assert_eq!(index[2], 0b101010);
//     assert_eq!(index[3], 0b100100);
//     assert_eq!(index[4], 0b010101);
//     assert_eq!(index[5], 0b001001);
//     assert_eq!(index[6], 0b110110);
//     assert_eq!(index[7], 0b011011);

//     let mut collector = 0_u32;
//     let small = [0b1111_u32, 0b0101, 0b1000, 0b0111, 0b1100, 0b0011, 0b0001, 0b1001];
//     let piece = [(small[0] << 4) | small[1], (small[2] << 4) | small[3],
//                             (small[4] << 4) | small[5], (small[6] << 4) | small[7]];
//     for i in 0..8
//         { println!("{:04b} ", small[i]); }
    
//     a_des.combine(&mut collector, piece[0]);
//     a_des.combine(&mut collector, piece[1]);
//     a_des.combine(&mut collector, piece[2]);
//     a_des.combine(&mut collector, piece[3]);
//     let col = IntUnion::new_with(collector);
//     for i in 0..4
//         { println!("{:08b} ", col.get_ubyte_(i)); }
//     assert_eq!(col.get_ubyte_(0), 0b11110101);
//     assert_eq!(col.get_ubyte_(1), 0b10000111);
//     assert_eq!(col.get_ubyte_(2), 0b11000011);
//     assert_eq!(col.get_ubyte_(3), 0b00011001);
//     println!("-------------------------------");
// }

// fn des_f()
// {
//     println!("des_f");
//     use cryptocol::number::IntUnion;
//     use cryptocol::symmetric::DES;

//     let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
//     print!("K =\t");
//     for i in 0..8
//         { print!("{:08b} ", key[i]); }
//     println!();

//     let mut right = IntUnion::new();
//     right.set_ubyte_(0, 0b_1111_0000_u8);
//     right.set_ubyte_(1, 0b_1010_1010_u8);
//     right.set_ubyte_(2, 0b_1111_0000_u8);
//     right.set_ubyte_(3, 0b_1010_1010_u8);
//     print!("R =\t");
//     for i in 0..4
//         { print!("{:08b} ", right.get_ubyte_(i)); }
//     println!();

//     let mut a_des = DES::new_with_key(key);
//     let c = a_des.f(0, right.get());
//     let cipher = IntUnion::new_with(c);

//     print!("F =\t");
//     for i in 0..4
//         { print!("{:08b} ", cipher.get_ubyte_(i)); }
//     println!();
//     println!("-------------------------------");
// }

fn des_encrypt_decrypt_u64_array_u64_main()
{
    des_encrypt_u64();
    des_decrypt_u64();
    des_encrypt_array_u64();
    des_decrypt_array_u64();
}

fn des_encrypt_u64()
{
    println!("des_encrypt_u64");
    use cryptocol::number::LongUnion;
    use cryptocol::symmetric::DES;

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();
    
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let mut plain = LongUnion::new();
    for i in 0..8
        { plain.set_ubyte_(i, message[i]); }
    let m = plain.get();
    println!("M_u64 =\t{:016X}", m);
    assert_eq!(m, 0x_EFCDAB8967452301_u64);

    let mut a_des = DES::new_with_key(key);
    let c = a_des.encrypt_u64(m);
    println!("C_u64 =\t{:016X}", c);
    assert_eq!(c, 0x_05B40A0F5413E885_u64);
    println!("-------------------------------");
}

fn des_decrypt_u64()
{
    println!("des_decrypt_u64");
    use cryptocol::number::LongUnion;
    use cryptocol::symmetric::DES;

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();
    
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05];
    let mut secret = LongUnion::new();
    for i in 0..8
        { secret.set_ubyte_(i, cipher[i]); }
    let c = secret.get();
    println!("C_u64 =\t{:016X}", c);
    assert_eq!(c, 0x_05B40A0F5413E885_u64);

    let mut a_des = DES::new_with_key(key);
    let m = a_des.decrypt_u64(c);
    println!("M_u64 =\t{:016X}", m);
    assert_eq!(m, 0x_EFCDAB8967452301_u64);
    println!("-------------------------------");
}

fn des_encrypt_array_u64()
{
    println!("des_encrypt_array_u64");
    use std::fmt::Write;
    use cryptocol::number::LongerUnion;
    use cryptocol::symmetric::DES;

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();
    
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10];
    let mut plain = LongerUnion::new();
    for i in 0..16
        { plain.set_ubyte_(i, message[i]); }
    let m = plain.get();
    println!("M_u128 =\t{:016X}", m);
    assert_eq!(m, 0x_1032547698BADCFEEFCDAB8967452301_u128);

    let m_arr = [plain.get_ulong_(0), plain.get_ulong_(1)];
    let mut c_arr = [0_u64; 2];
    let mut a_des = DES::new_with_key(key);
    a_des.encrypt_array_u64(&m_arr, &mut c_arr);
    let mut cipher = LongerUnion::new();
    cipher.set_ulong_(0, c_arr[0]);
    cipher.set_ulong_(1, c_arr[1]);

    print!("C =\t");
    for i in 0..16
        { print!("{:02X}", cipher.get_ubyte_(i)); }
    println!();
    assert_eq!(cipher.get(), 0x_1815064B3D5BB64A05B40A0F5413E885_u128);

    let mut txt = String::new();
    for i in 0..16
        { write!(txt, "{:02X}", cipher.get_ubyte_(i)); }
    assert_eq!(txt, "85E813540F0AB4054AB65B3D4B061518");
    println!("-------------------------------");
}

fn des_decrypt_array_u64()
{
    println!("des_decrypt_array_u64");
    use std::fmt::Write;
    use cryptocol::number::LongerUnion;
    use cryptocol::symmetric::DES;

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();
    
    let c_arr = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x4A, 0xB6, 0x5B, 0x3D, 0x4B, 0x06, 0x15, 0x18];
    let mut cipher = LongerUnion::new();
    for i in 0..16
        { cipher.set_ubyte_(i, c_arr[i]); }
    let c = cipher.get();
    println!("C_u128 =\t{:016X}", c);
    assert_eq!(c, 0x_1815064B3D5BB64A05B40A0F5413E885_u128);

    let c_arr = [cipher.get_ulong_(0), cipher.get_ulong_(1)];
    let mut m_arr = [0_u64; 2];
    let mut a_des = DES::new_with_key(key);
    a_des.decrypt_array_u64(&c_arr, &mut m_arr);
    let mut message = LongerUnion::new();
    message.set_ulong_(0,m_arr[0]);
    message.set_ulong_(1,m_arr[1]);

    print!("M =\t");
    for i in 0..16
        { print!("{:02X}", message.get_ubyte_(i)); }
    println!();

    let mut txt = String::new();
    for i in 0..16
        { write!(txt, "{:02X}", message.get_ubyte_(i)); }
    assert_eq!(txt, "0123456789ABCDEFFEDCBA9876543210");
    println!("-------------------------------");
}

fn des_crypt_with_padding_pkcs7_main()
{
    des_encrypt_with_padding_pkcs7();
    des_decrypt_with_padding_pkcs7();
    des_encrypt_str_with_padding_pkcs7();
    des_encrypt_string_with_padding_pkcs7();
    des_encrypt_array_with_padding_pkcs7();
    des_encrypt_vec_with_padding_pkcs7();
}

fn des_encrypt_with_padding_pkcs7()
{
    println!("des_encrypt_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7()
{
    println!("des_decrypt_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0xFD, 0xF2, 0xE1, 0x74, 0x49, 0x29, 0x22, 0xF8];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x29, 0x7C, 0x75, 0x3D, 0x2D, 0xBC, 0xC8, 0xC9];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7()
{
    println!("des_encrypt_str_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7()
{
    println!("des_encrypt_string_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7()
{
    println!("des_encrypt_array_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7()
{
    println!("des_encrypt_vec_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_main()
{
    des_encrypt_with_padding_iso();
    des_decrypt_with_padding_iso();
    des_encrypt_str_with_padding_iso();
    des_encrypt_string_with_padding_iso();
    des_encrypt_array_with_padding_iso();
    des_encrypt_vec_with_padding_iso();
}

fn des_encrypt_with_padding_iso()
{
    println!("des_encrypt_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso()
{
    println!("des_decrypt_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0x87, 0xAB, 0x78, 0xD1, 0x1E, 0x18, 0x8D, 0xF6];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x00, 0xD8, 0xD8, 0xF2, 0x97, 0x61, 0xF1, 0x9E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_iso(&cipher as *const u8, cipher.len() as u64, message.uu8.as_mut_ptr()) };

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso()
{
    println!("des_encrypt_str_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso()
{
    println!("des_encrypt_string_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso()
{
    println!("des_encrypt_array_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso()
{
    println!("des_encrypt_vec_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_pkcs7_ecb_main()
{
    des_encrypt_with_padding_pkcs7_ecb();
    des_decrypt_with_padding_pkcs7_ecb();
    des_encrypt_str_with_padding_pkcs7_ecb();
    des_encrypt_string_with_padding_pkcs7_ecb();
    des_encrypt_array_with_padding_pkcs7_ecb();
    des_encrypt_vec_with_padding_pkcs7_ecb();
}

fn des_encrypt_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_ecb()
{
    println!("des_decrypt_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0xFD, 0xF2, 0xE1, 0x74, 0x49, 0x29, 0x22, 0xF8];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x29, 0x7C, 0x75, 0x3D, 0x2D, 0xBC, 0xC8, 0xC9];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_str_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_string_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_array_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_vec_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_ecb_main()
{
    des_encrypt_with_padding_iso_ecb();
    des_decrypt_with_padding_iso_ecb();
    des_encrypt_str_with_padding_iso_ecb();
    des_encrypt_string_with_padding_iso_ecb();
    des_encrypt_array_with_padding_iso_ecb();
    des_encrypt_vec_with_padding_iso_ecb();
}

fn des_encrypt_with_padding_iso_ecb()
{
    println!("des_encrypt_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_ecb()
{
    println!("des_decrypt_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0x87, 0xAB, 0x78, 0xD1, 0x1E, 0x18, 0x8D, 0xF6];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x00, 0xD8, 0xD8, 0xF2, 0x97, 0x61, 0xF1, 0x9E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_iso_ecb(&cipher as *const u8, cipher.len() as u64, message.uu8.as_mut_ptr()) };

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_ecb()
{
    println!("des_encrypt_str_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_ecb()
{
    println!("des_encrypt_string_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_ecb()
{
    println!("des_encrypt_array_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_ecb()
{
    println!("des_encrypt_vec_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_pkcs7_cbc_main()
{
    des_encrypt_with_padding_pkcs7_cbc();
    des_decrypt_with_padding_pkcs7_cbc();
    des_encrypt_str_with_padding_pkcs7_cbc();
    des_encrypt_string_with_padding_pkcs7_cbc();
    des_encrypt_array_with_padding_pkcs7_cbc();
    des_encrypt_vec_with_padding_pkcs7_cbc();
}

fn des_encrypt_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B33EB4FDA247DB78E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CC5502C9C37698343");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_cbc()
{
    println!("des_decrypt_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0x9A, 0xFF, 0x7F, 0x59, 0xD8, 0x1A, 0xE5, 0x9B, 0x33, 0xEB, 0x4F, 0xDA, 0x24, 0x7D, 0xB7, 0x8E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0xC5, 0x50, 0x2C, 0x9C, 0x37, 0x69, 0x83, 0x43];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_str_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B0A4C55A1B41A67A88");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_string_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B0A4C55A1B41A67A88");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_array_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B33EB4FDA247DB78E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CC5502C9C37698343");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_vec_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B33EB4FDA247DB78E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CC5502C9C37698343");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_cbc_main()
{
    des_encrypt_with_padding_iso_cbc();
    des_decrypt_with_padding_iso_cbc();
    des_encrypt_str_with_padding_iso_cbc();
    des_encrypt_string_with_padding_iso_cbc();
    des_encrypt_array_with_padding_iso_cbc();
    des_encrypt_vec_with_padding_iso_cbc();
}

fn des_encrypt_with_padding_iso_cbc()
{
    println!("des_encrypt_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B9F4B15B7DDDFE2B8");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CE80D8F56AA71F3D0");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_cbc()
{
    println!("des_decrypt_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0x9A, 0xFF, 0x7F, 0x59, 0xD8, 0x1A, 0xE5, 0x9B, 0x9F, 0x4B, 0x15, 0xB7, 0xDD, 0xDF, 0xE2, 0xB8];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0xE8, 0x0D, 0x8F, 0x56, 0xAA, 0x71, 0xF3, 0xD0];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_cbc()
{
    println!("des_encrypt_str_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B05278468F4BED756A");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_cbc()
{
    println!("des_encrypt_string_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B05278468F4BED756A");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_cbc()
{
    println!("des_encrypt_array_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B9F4B15B7DDDFE2B8");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CE80D8F56AA71F3D0");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_cbc()
{
    println!("des_encrypt_vec_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B9F4B15B7DDDFE2B8");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CE80D8F56AA71F3D0");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_pkcs7_pcbc_main()
{
    des_encrypt_with_padding_pkcs7_pcbc();
    des_decrypt_with_padding_pkcs7_pcbc();
    des_encrypt_str_with_padding_pkcs7_pcbc();
    des_encrypt_string_with_padding_pkcs7_pcbc();
    des_encrypt_array_with_padding_pkcs7_pcbc();
    des_encrypt_vec_with_padding_pkcs7_pcbc();
}

fn des_encrypt_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B136D0AAB7F464DDE6");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA613F0A58ED52F62A2E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_pcbc()
{
    println!("des_decrypt_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0xBA, 0xED, 0x14, 0x04, 0x11, 0x41, 0x66, 0xB1, 0x36, 0xD0, 0xAA, 0xB7, 0xF4, 0x64, 0xDD, 0xE6];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0x3F, 0x0A, 0x58, 0xED, 0x52, 0xF6, 0x2A, 0x2E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_str_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC4F64B13AE4FAA77E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_string_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC4F64B13AE4FAA77E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_array_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B136D0AAB7F464DDE6");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA613F0A58ED52F62A2E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_vec_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B136D0AAB7F464DDE6");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA613F0A58ED52F62A2E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_pcbc_main()
{
    des_encrypt_with_padding_iso_pcbc();
    des_decrypt_with_padding_iso_pcbc();
    des_encrypt_str_with_padding_iso_pcbc();
    des_encrypt_string_with_padding_iso_pcbc();
    des_encrypt_array_with_padding_iso_pcbc();
    des_encrypt_vec_with_padding_iso_pcbc();
}

fn des_encrypt_with_padding_iso_pcbc()
{
    println!("des_encrypt_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B16FF807E714DB65C2");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61EB9C35C902B7B4BC");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_pcbc()
{
    println!("des_decrypt_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0xBA, 0xED, 0x14, 0x04, 0x11, 0x41, 0x66, 0xB1, 0x6F, 0xF8, 0x07, 0xE7, 0x14, 0xDB, 0x65, 0xC2];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0xEB, 0x9C, 0x35, 0xC9, 0x02, 0xB7, 0xB4, 0xBC];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_pcbc()
{
    println!("des_encrypt_str_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC348BC507FFF5613E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_pcbc()
{
    println!("des_encrypt_string_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC348BC507FFF5613E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_pcbc()
{
    println!("des_encrypt_array_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B16FF807E714DB65C2");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61EB9C35C902B7B4BC");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_pcbc()
{
    println!("des_encrypt_vec_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B16FF807E714DB65C2");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61EB9C35C902B7B4BC");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_cfb_main()
{
    des_encrypt_cfb();
    des_decrypt_cfb();
    des_encrypt_str_cfb();
    des_encrypt_string_cfb();
    des_encrypt_array_cfb();
    des_encrypt_vec_cfb();
}

fn des_encrypt_cfb()
{
    println!("des_encrypt_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_cfb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7A3BD8C");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_cfb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_cfb()
{
    println!("des_decrypt_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x49, 0xF4, 0x18, 0x1D, 0x45, 0xE4, 0x9A, 0x9A, 0xB0, 0x3A, 0xF9, 0xF0, 0xD7, 0xA3, 0xBD, 0x8C];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x49, 0xF4, 0x18, 0x1D, 0x45, 0xE4, 0x9A, 0x9A, 0xB0, 0x3A, 0xF9, 0xF0, 0xD7];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_cfb()
{
    println!("des_encrypt_str_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67B952FFF079F4D0E32AD95AD79006FEFD7F9B62FE1ADBEBFF5FBD9EF56FCBBE30A21505F3A87DE1A4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_cfb()
{
    println!("des_encrypt_string_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67B952FFF079F4D0E32AD95AD79006FEFD7F9B62FE1ADBEBFF5FBD9EF56FCBBE30A21505F3A87DE1A4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_cfb()
{
    println!("des_encrypt_array_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7A3BD8C");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_cfb()
{
    println!("des_encrypt_vec_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7A3BD8C");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_ofb_main()
{
    des_encrypt_ofb();
    des_decrypt_ofb();
    des_encrypt_str_ofb();
    des_encrypt_string_ofb();
    des_encrypt_array_ofb();
    des_encrypt_vec_ofb();
}

fn des_encrypt_ofb()
{
    println!("des_encrypt_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ofb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B830434");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ofb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_ofb()
{
    println!("des_decrypt_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x8A, 0x74, 0x9A, 0xF7, 0x6E, 0x5B, 0xC8, 0x4B, 0xD6, 0x6F, 0xDD, 0xAA, 0x3B, 0x83, 0x04, 0x34];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x8A, 0x74, 0x9A, 0xF7, 0x6E, 0x5B, 0xC8, 0x4B, 0xD6, 0x6F, 0xDD, 0xAA, 0x3B];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_ofb()
{
    println!("des_encrypt_str_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67FA76DDD648499FA520B74622A33ED5BCFF82DE7AB6984F36DE4C1FB6AF2F5556B1D1C5BAF9A7F9C4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_ofb()
{
    println!("des_encrypt_string_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67FA76DDD648499FA520B74622A33ED5BCFF82DE7AB6984F36DE4C1FB6AF2F5556B1D1C5BAF9A7F9C4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_ofb()
{
    println!("des_encrypt_array_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B830434");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_ofb()
{
    println!("des_encrypt_vec_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B830434");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_ctr_main()
{
    des_encrypt_ctr();
    des_decrypt_ctr();
    des_encrypt_str_ctr();
    des_encrypt_string_ctr();
    des_encrypt_array_ctr();
    des_encrypt_vec_ctr();
}

fn des_encrypt_ctr()
{
    println!("des_encrypt_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ctr(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC54761AA55");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ctr(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC547");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_ctr()
{
    println!("des_decrypt_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x86_u8, 0xDC, 0xC1, 0x1C, 0x30, 0x3A, 0x6B, 0xF4, 0xC0, 0x43, 0xCB, 0x5B, 0xCE, 0x42, 0x09, 0x35, 0xBE, 0x31, 0x0D, 0xC5, 0x47, 0x61, 0xAA, 0x55];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x86_u8, 0xDC, 0xC1, 0x1C, 0x30, 0x3A, 0x6B, 0xF4, 0xC0, 0x43, 0xCB, 0x5B, 0xCE, 0x42, 0x09, 0x35, 0xBE, 0x31, 0x0D, 0xC5, 0x47];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_ctr()
{
    println!("des_encrypt_str_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D397ED0899F8D53BB0418C7AE8505EDB48E9964DDFDC7BDD617347F37F85252F9EABABF5F1CF75FE7E88C352EA53B5F9");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_ctr()
{
    println!("des_encrypt_string_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D397ED0899F8D53BB0418C7AE8505EDB48E9964DDFDC7BDD617347F37F85252F9EABABF5F1CF75FE7E88C352EA53B5F9");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_ctr()
{
    println!("des_encrypt_array_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC54761AA55");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC547");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_ctr()
{
    println!("des_encrypt_vec_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC54761AA55");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC547");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}
