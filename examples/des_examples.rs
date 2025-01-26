// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use cryptocol::symmetric::DES;

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]
pub fn main()
{
    des_permutate_initially_finally();
    des_permutate_expansion();
    des_permutate_translation();
}

fn des_permutate_initially_finally()
{
    println!("des_permutate_initially_finally");
    use cryptocol::number::LongUnion;
    /*
    let mut a_des = DES::new();
    let block = (1_u64 << (8-2)) | (1_u64 << ((50-1) / 8 * 8 + (7 - (50-1) % 8)));
    a_des.set_block(block);
    a_des.permutate_initially();
    let out = a_des.get_block();
    let bu = LongUnion::new_with(block);
    print!("block =\t");
    for i in 0..8
        { print!("{:08b} ", bu.get_ubyte_(i)); }
    println!();
    let ou = LongUnion::new_with(out);
    print!("out =\t");
    for i in 0..8
        { print!("{:08b} ", ou.get_ubyte_(i)); }
    println!();

    a_des.permutate_finally();
    let back = a_des.get_block();
    let cu = LongUnion::new_with(back);
    print!("back =\t");
    for i in 0..8
        { print!("{:08b} ", cu.get_ubyte_(i)); }
    println!();
    */
    println!("-------------------------------");
}

fn des_permutate_expansion()
{
    println!("des_permutate_initially_finally");
    use cryptocol::number::{ IntUnion, LongUnion };
    /*
    let mut a_des = DES::new();
    let right = (0b1_u32 << 24) | (1_u32); //(32-32);
    let out = a_des.expand(right);

    let bu = IntUnion::new_with(right);
    print!("right =\t");
    for i in 0..4
        { print!("{:08b} ", bu.get_ubyte_(i)); }
    println!();

    let ou = LongUnion::new_with(out);
    print!("out =\t");
    for i in 0..6
        { print!("{:08b} ", ou.get_ubyte_(i)); }
    println!();
    */
    println!("-------------------------------");
}

fn des_permutate_translation()
{

}