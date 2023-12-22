// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]

use Cryptocol::hash;

fn main()
{
    Hash_MD5_main();
}

fn Hash_MD5_main()
{
    MD5_Quick_Start();
    MD5_new();
    MD5_digest();
    MD5_digest_str();
    MD5_digest_string();
    MD5_digest_array();
    MD5_digest_vec();
    MD5_get_HashValue();
    MD5_get_HashValue_in_string();
    MD5_get_HashValue_in_array();
    MD5_get_HashValue_in_vec();
    MD5_fmt_for_to_string();
    MD5_fmt_for_println();
}

fn MD5_Quick_Start()
{
    println!("MD5_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::MD5;
    let mut hash = MD5::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "D41D8CD98F00B204E9800998ECF8427E");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "7FC56270E7A70FA81A5935B72EACBE29");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "49DC5E45FBEC1433E2C612E5AA809C10");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "17ED1DB5CD96184041659D84BB36D76B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "C3EB6D4A1071E1A9C5E08FEF6E8F3FBF");

    txt = "I am testing MD5 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "6C33614E6317DC4641573E0EBC287F98");

    let mut txt = "I am testing MD5 for the data whose length is sixty-four bytes..";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "200F9A19EA45A830284342114483172B");

    txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "9831162AB272AE1D85245B75726D215E");
    println!("-------------------------------");
}


fn MD5_new()
{
    println!("MD5_new");
    use Cryptocol::hash::MD5;
    let mut hash = MD5::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    println!("-------------------------------");
}

fn MD5_digest()
{
    println!("MD5_digest");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method digest().";
    let mut hash = MD5::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "336EA91DD3216BD0FC841E86F9E722D8");
    println!("-------------------------------");
}

fn MD5_digest_str()
{
    println!("MD5_digest_str");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method digest_str().";
    let mut hash = MD5::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F2E455CEB5FB993A980E67D3FA8A3961");
    println!("-------------------------------");
}

fn MD5_digest_string()
{
    println!("MD5_digest_string");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = MD5::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "40929E789D2F5880B85456E289F704C0");
    println!("-------------------------------");
}

fn MD5_digest_array()
{
    println!("MD5_digest_array");
    use Cryptocol::hash::MD5;

    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = MD5::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");
    println!("-------------------------------");
}

fn MD5_digest_vec()
{
    println!("MD5_digest_vec");
    use Cryptocol::hash::MD5;

    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = MD5::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");
    println!("-------------------------------");
}

fn MD5_get_HashValue()
{
    println!("MD5_get_HashValue");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 16];
    let mut hash = MD5::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[D9, FB, 90, AB, DD, 2E, 1E, 48, D8, 5E, E5, 08, 4B, AE, 2C, 39]");
    println!("-------------------------------");
}

fn MD5_get_HashValue_in_string()
{
    println!("MD5_get_HashValue_in_string");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = MD5::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "7BB1ED16E2E302AA3B16CD24EC3E3093");
    println!("-------------------------------");
}

fn MD5_get_HashValue_in_array()
{
    println!("MD5_get_HashValue_in_array");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = MD5::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[A4BE6EEF, C9A5DFBA, 558B5ADF, 3B1035F9]");
    println!("-------------------------------");
}

fn MD5_get_HashValue_in_vec()
{
    println!("MD5_get_HashValue_in_vec");
    use Cryptocol::hash::MD5;

    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = MD5::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[C24C5F26, D87BBAC8, D66148F4, 4D7DE209]");
    println!("-------------------------------");
}

fn MD5_fmt_for_to_string()
{
    println!("MD5_fmt_for_to_string");
    use Cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.to_string());
    assert_eq!(hash.to_string(), "ED085603C2CDE77DD0C6FED3EC1A8ADB");
}

fn MD5_fmt_for_println()
{
    println!("MD5_fmt_for_println");
    use Cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "6C9494A4A5C313001695262D72571F74");
    println!("-------------------------------");
}
