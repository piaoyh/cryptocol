// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]


fn main()
{
    Hash_MD5_main();
    Hash_SHA1_main();
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

fn Hash_SHA1_main()
{
    SHA1_Quick_Start();
    SHA1_new();
    SHA1_digest();
    SHA1_digest_str();
    SHA1_digest_string();
    SHA1_digest_array();
    SHA1_digest_vec();
    SHA1_get_HashValue();
    SHA1_get_HashValue_in_string();
    SHA1_get_HashValue_in_array();
    SHA1_get_HashValue_in_vec();
    SHA1_fmt_for_to_string();
    SHA1_fmt_for_println();
}

fn SHA1_Quick_Start()
{
    println!("SHA1_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA1;
    let mut hash = SHA1::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "6DCD4CE23D88E2EE9568BA546C007C63D9131C1B");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "0BBCDBD1616A1D2230100F629649DCF5B7A28B7F");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B82A61505779F6B3ACA4F5E0D54DA44C17375B49");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "C6DC54281357FC16D357E1D730BFC313C585DAEC");

    txt = "I am testing SHA1 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "36CD36337097D764797091E5796B6FF45A9FA79F");

    let mut txt = "I am testing SHA-1 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "E408F6B82DCDDB5EE6613A759AC1B13D0FA1CEF1");

    txt = "I am testing SHA1 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "BB2C79F551B95963ECE49D40F8A92349BF66CAE7");
    println!("-------------------------------");
}


fn SHA1_new()
{
    println!("SHA1_new");
    use Cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "67452301EFCDAB8998BADCFE10325476C3D2E1F0");
    println!("-------------------------------");
}

fn SHA1_digest()
{
    println!("SHA1_digest");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA1::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9631162DFDAEAB89821256D4585D66D35CD61FD6");
    println!("-------------------------------");
}

fn SHA1_digest_str()
{
    println!("SHA1_digest_str");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9FDE56BBB5028966CC2E7BDCD0758FE3121407E6");
    println!("-------------------------------");
}

fn SHA1_digest_string()
{
    println!("SHA1_digest_string");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA1::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "FDCDC0EBC9181B881BE1F15FECEBB9D70E4DDAAB");
    println!("-------------------------------");
}

fn SHA1_digest_array()
{
    println!("SHA1_digest_array");
    use Cryptocol::hash::SHA1;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA1::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");
    println!("-------------------------------");
}

fn SHA1_digest_vec()
{
    println!("SHA1_digest_vec");
    use Cryptocol::hash::SHA1;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA1::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");
    println!("-------------------------------");
}

fn SHA1_get_HashValue()
{
    println!("SHA1_get_HashValue");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 20];
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[E9, C6, F4, 3B, 77, AA, 27, A1, 6E, B4, F0, F5, 5B, F3, D8, C7, 3A, EB, 7F, 93]");
    println!("-------------------------------");
}

fn SHA1_get_HashValue_in_string()
{
    println!("SHA1_get_HashValue_in_string");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "899B9673103FCB06B237A5A6A7D04D749EA4BD92");
    println!("-------------------------------");
}

fn SHA1_get_HashValue_in_array()
{
    println!("SHA1_get_HashValue_in_array");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[E9840962, 837B21A9, D9321727, 74980B51, 364DD5A2]");
    println!("-------------------------------");
}

fn SHA1_get_HashValue_in_vec()
{
    println!("SHA1_get_HashValue_in_vec");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[96E00128, E1E04E29, F65ABA7B, AD10C0A2, 1BC438DA]");
    println!("-------------------------------");
}

fn SHA1_fmt_for_to_string()
{
    println!("SHA1_fmt_for_to_string");
    use Cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.to_string());
    assert_eq!(hash.to_string(), "8D0A6284BBFF4DE8D68962A924842C80959B0404");
    println!("-------------------------------");
}

fn SHA1_fmt_for_println()
{
    println!("SHA1_fmt_for_println");
    use Cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "835CEFA297628E4DADBDA011C5FDEA68D88A8EE8");
    println!("-------------------------------");
}
