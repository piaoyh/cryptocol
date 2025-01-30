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
    // des_permutate_initially_finally();
    // des_permutate_expansion();
    // des_split();
    // des_make_round_keys();
    // des_slice_indices_combine();
    // des_f();
    des_encrypt_decrypt_u64_array_u64();
    des_encrypt_with_padding_pkcs7();
    des_encrypt_with_padding_iso();
    des_decrypt_with_padding_pkcs7();
    des_decrypt_with_padding_iso();
}

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

fn des_encrypt_decrypt_u64_array_u64()
{
    println!("des_encrypt_decrypt_u64_array_u64");
    use std::fmt::Write;
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

    let m = a_des.decrypt_u64(c);
    println!("M_u64 =\t{:016X}", m);
    assert_eq!(m, 0x_EFCDAB8967452301_u64);

    print!("M =\t");
    for i in 0..8
        { print!("{:02X}", message[i]); }
    println!();

    let m = [plain.get()];
    let mut c = [0_u64; 1];
    let mut a_des = DES::new_with_key(key);
    a_des.encrypt_array_u64(&m, &mut c);
    let cipher = LongUnion::new_with(c[0]);

    print!("C =\t");
    for i in 0..8
        { print!("{:02X}", cipher.get_ubyte_(i)); }
    println!();

    let mut txt = String::new();
    for i in 0..8
        { write!(txt, "{:02X}", cipher.get_ubyte_(i)); }
    assert_eq!(txt, "85E813540F0AB405");

    let mut m = [0_u64; 1];
    let mut a_des = DES::new_with_key(key);
    a_des.decrypt_array_u64(&c, &mut m);
    let message = LongUnion::new_with(m[0]);

    print!("M =\t");
    for i in 0..8
        { print!("{:02X}", message.get_ubyte_(i)); }
    println!();

    let mut txt = String::new();
    for i in 0..8
        { write!(txt, "{:02X}", message.get_ubyte_(i)); }
    assert_eq!(txt, "0123456789ABCDEF");
    println!("-------------------------------");
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
    
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");
    println!("-------------------------------");
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
    
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", cipher.uu8[i]); } }
    // assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", cipher.uu8[i]); } }
    // assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");
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
    
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", cipher.uu8[i]); } }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_pkcs7(cipher.uu8.as_ptr(), length, back.uu8.as_mut_ptr()) as usize};
    print!("B =\t");
    for i in 0..length
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", cipher.uu8[i]); } }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_pkcs7(cipher.uu8.as_ptr(), length,back.uu8.as_mut_ptr()) as usize };
    print!("B =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
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
    
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", cipher.uu8[i]); } }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_iso(cipher.uu8.as_ptr(), length, back.uu8.as_mut_ptr()) as usize};
    print!("B =\t");
    for i in 0..length
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, cipher.uu8.as_mut_ptr()) };

    print!("C =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", cipher.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", cipher.uu8[i]); } }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_iso(cipher.uu8.as_ptr(), length,back.uu8.as_mut_ptr()) as usize };
    print!("B =\t");
    for i in 0..length as usize
        { unsafe { print!("{:02X}", back.uu8[i]); } }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { unsafe { write!(&mut txt, "{:02X}", back.uu8[i]); } }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}