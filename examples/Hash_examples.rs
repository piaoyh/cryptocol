// Copyright 2023, 2024 PARK Youngho.
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
    Hash_MD4_main();
    Hash_MD5_main();
    Hash_SHA1_main();
    Hash_SHA2_256_main();
    Hash_SHA2_224_main();
    Hash_SHA2_512_main();
    Hash_SHA2_384_main();
    Hash_SHA2_512_256_main();
    Hash_SHA2_512_t_256_main();
    Hash_SHA2_512_t_224_main();
}

fn Hash_MD4_main()
{
    MD4_Quick_Start();
    MD4_new();
    MD4_digest();
    MD4_digest_str();
    MD4_digest_string();
    MD4_digest_array();
    MD4_digest_vec();
    MD4_get_HashValue();
    MD4_get_HashValue_in_string();
    MD4_get_HashValue_in_array();
    MD4_get_HashValue_in_vec();
    MD4_fmt_for_to_string();
    MD4_fmt_for_println();
}

fn MD4_Quick_Start()
{
    println!("MD4_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "31D6CFE0D16AE931B73C59D7E0C089C0");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "D5EF20EEB3F75679F86CF57F93ED0FFE");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "6407C0E728DA762A04924ADFE630974C");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4F4A24D124B996BEA395344419F9A06B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "9DE35D8FCF68E74867FFB63F28625ABE");

    txt = "I am testing MD4 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "3A9F1487472B3A4315E0C90DC5CB3A2E");

    let mut txt = "I am testing MD4 for the message which is sixty-four bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "6CDB5B2BFF823A4A7B23675180EB7BEF");

    txt = "I am testing MD4 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "56771653687981390B0EB2A7D0A40DBB");
    println!("-------------------------------");
}

fn MD4_new()
{
    println!("MD4_new");
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    println!("-------------------------------");
}

fn MD4_digest()
{
    println!("MD4_digest");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method digest().";
    let mut hash = MD4::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A18836F660C3C66B8CBEE4BD24FEFFA9");
    println!("-------------------------------");
}

fn MD4_digest_str()
{
    println!("MD4_digest_str");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method digest_str().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");
    println!("-------------------------------");
}

fn MD4_digest_string()
{
    println!("MD4_digest_string");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = MD4::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DF23C7808B2B158C5E2D8C9FE1FF2ECC");
    println!("-------------------------------");
}

fn MD4_digest_array()
{
    println!("MD4_digest_array");
    use Cryptocol::hash::MD4;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = MD4::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");
    println!("-------------------------------");
}

fn MD4_digest_vec()
{
    println!("MD4_digest_vec");
    use Cryptocol::hash::MD4;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = MD4::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");
    println!("-------------------------------");
}

fn MD4_get_HashValue()
{
    println!("MD4_get_HashValue");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 16];
    let mut hash = MD4::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[12, E1, F9, 39, C3, D8, 09, A7, 23, 7D, 94, B4, F1, A0, 1E, FD]");
    println!("-------------------------------");
}

fn MD4_get_HashValue_in_string()
{
    println!("MD4_get_HashValue_in_string");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "B871AC07999486EB0A6450DA5E09E92D");
    println!("-------------------------------");
}

fn MD4_get_HashValue_in_array()
{
    println!("MD4_get_HashValue_in_array");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[9F7E4FD8, 906C5422, 9FAAAFBA, 363BE03A]");
    println!("-------------------------------");
}

fn MD4_get_HashValue_in_vec()
{
    println!("MD4_get_HashValue_in_vec");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[6BD261A9, 47EFE9B7, 4397C1B, 628A61D7]");
    println!("-------------------------------");
}

fn MD4_fmt_for_to_string()
{
    println!("MD4_fmt_for_to_string");
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "E2244B71E17D5BD7E1CCEB58C8F8C82E");
}

fn MD4_fmt_for_println()
{
    println!("MD4_fmt_for_println");
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3C607B5548C155DCF4E84C7A6C21D349");
    println!("-------------------------------");
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

    let mut txt = "I am testing MD5 for the message which is sixty-four bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "584D41C6837AC714275196E4FF14B2EF");

    txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
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
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
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
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
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
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
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

fn Hash_SHA2_256_main()
{
    SHA2_256_Quick_Start();
    SHA2_256_new();
    SHA2_256_digest();
    SHA2_256_digest_str();
    SHA2_256_digest_string();
    SHA2_256_digest_array();
    SHA2_256_digest_vec();
    SHA2_256_get_HashValue();
    SHA2_256_get_HashValue_in_string();
    SHA2_256_get_HashValue_in_array();
    SHA2_256_get_HashValue_in_vec();
    SHA2_256_fmt_for_to_string();
    SHA2_256_fmt_for_println();
}

fn SHA2_256_Quick_Start()
{
    println!("SHA2_256_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "559AEAD08264D5795D3909718CDD05ABD49572E84FE55590EEF31A88A08FDFFD");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "3DECCF6826EF78994F099EC321F883527E8218301605E66114268E769D1DF61E");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "9546AE34CBF111CEDC01164DE817512B4DC3B1F9967E208068868BF557E9972A");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "5ADF1754644575EB30E7EBCE1F5EA2AD52E99CDD98713B805B2B2F02CACB3E31");

    txt = "I am testing SHA-2/256 for the data whose length is 62 bytes..";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "12C0E6762B448D5FBFF941D54F932BBFAE308E3EBDEE1795555681D3D9A2C5CF");

    let mut txt = "I am testing SHA-2/256 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "222268D061DF342E7142E79B49EAF57D34B74212D2150C5CA93EF05C767EA5F3");

    txt = "I am testing SHA-2/256 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "D8B1AFE899AA3E02E44EA1603730790791B92BB5A6D14632EB44131BE21334C7");

    txt = "This hash algorithm SHA-2/256 is being tested for the case message whose length is more than one hundred thirty-one bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "8604990CF14A490072D6EC3BD5182079C7B46F9F7E18E684C2C9E645CFA8FEF0");
    println!("-------------------------------");
}

fn SHA2_256_new()
{
    println!("SHA2_256_new");
    use Cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6A09E667BB67AE853C6EF372A54FF53A510E527F9B05688C1F83D9AB5BE0CD19");
    println!("-------------------------------");
}

fn SHA2_256_digest()
{
    println!("SHA2_256_digest");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_256::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F1ECFB4A9F399E3786FD87ABE5D27DB64ADB61F4754BA68EFADCB3792DD15827");
    println!("-------------------------------");
}

fn SHA2_256_digest_str()
{
    println!("SHA2_256_digest_str");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B9396CF025B6ECC98178BE081D045DCC2CD3F18FE1450B1B420451A53C571D32");
    println!("-------------------------------");
}

fn SHA2_256_digest_string()
{
    println!("SHA2_256_digest_string");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_256::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "0DA7CA307C40C3661BD59AAF2828CAC1D3E3C82385CC8EC92A2FAFF1C0A5DF43");
    println!("-------------------------------");
}

fn SHA2_256_digest_array()
{
    println!("SHA2_256_digest_array");
    use Cryptocol::hash::SHA2_256;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_256::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "411D3F1D2390FF3F482AC8DF4E730780BB081A192F283D2F373138FD101DC8FE");
    println!("-------------------------------");
}

fn SHA2_256_digest_vec()
{
    println!("SHA2_256_digest_vec");
    use Cryptocol::hash::SHA2_256;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_256::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "411D3F1D2390FF3F482AC8DF4E730780BB081A192F283D2F373138FD101DC8FE");
    println!("-------------------------------");
}

fn SHA2_256_get_HashValue()
{
    println!("SHA2_256_get_HashValue");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 32];
    let mut hash = SHA2_256::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[3A, C3, 4A, 2D, 8B, DF, 68, 0D, E1, D0, 11, AB, 62, 73, E1, 25, 75, AC, 28, E8, C8, 8E, F8, 85, 4A, 9C, 32, F3, B8, 61, AA, BD]");
    println!("-------------------------------");
}

fn SHA2_256_get_HashValue_in_string()
{
    println!("SHA2_256_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "E53A2E689388E5A377C37BF694EE88AEF95B3EDA3E72536796E7FD66CF68059A");
    println!("-------------------------------");
}

fn SHA2_256_get_HashValue_in_array()
{
    println!("SHA2_256_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[E74E58B2, A906081, D6250689, F414A121, AC836F06, CF15DA05, 7AC223F3, 32411F8C]");
    println!("-------------------------------");
}

fn SHA2_256_get_HashValue_in_vec()
{
    println!("SHA2_256_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[D83D882D, BFC511D3, B3CBE270, 9F075C02, 2862878C, E69D69D9, 943EE0A4, C8E7F88E]");
    println!("-------------------------------");
}

fn SHA2_256_fmt_for_to_string()
{
    println!("SHA2_256_fmt_for_to_string");
    use Cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "46577469D8A5CBCA1AC874A5350E4FACD318A468046816B066117D51DB9D1640");
    println!("-------------------------------");
}

fn SHA2_256_fmt_for_println()
{
    println!("SHA2_256_fmt_for_println");
    use Cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B8338443431AB13309330A8064AF039E39F90CAC334CF8EA1FF0640646AB121C");
    println!("-------------------------------");
}


fn Hash_SHA2_224_main()
{
    SHA2_224_Quick_Start();
    SHA2_224_new();
    SHA2_224_digest();
    SHA2_224_digest_str();
    SHA2_224_digest_string();
    SHA2_224_digest_array();
    SHA2_224_digest_vec();
    SHA2_224_get_HashValue();
    SHA2_224_get_HashValue_in_string();
    SHA2_224_get_HashValue_in_array();
    SHA2_224_get_HashValue_in_vec();
    SHA2_224_fmt_for_to_string();
    SHA2_224_fmt_for_println();
}

fn SHA2_224_Quick_Start()
{
    println!("SHA2_224_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "D14A028C2A3A2BC9476102BB288234C415A2B01F828EA62AC5B3E42F");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "5CFE2CDDBB9940FB4D8505E25EA77E763A0077693DBB01B1A6AA94F2");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "8B73B7B79FA0E4EC45AF8B4230F88F314554D503FD88F05A48A07DD3");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "0592A67F23DD6B21CA691041B4682831C61D40E0235CEB59AC557358");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "8949B6F7EB831F47B81E3361135D835E93576ED5BAAA32209303C37C");

    txt = "I am testing SHA-2/224 for the data whose length is 62 bytes..";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "84E6CB12422BB17F614D03B95E0DF142F6FD4EABB69E59A3C7C8A1AA");

    let mut txt = "I am testing SHA-2/224 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "FB297817767C2236610810DC9BE34EFB2FDCC0E0C7E2D0BA736C59DB");

    txt = "I am testing SHA-2/224 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DAA4205BB0B38C625AD8A53DAF1FC8A61AFA33D7513B3615826750FD");
    println!("-------------------------------");
}

fn SHA2_224_new()
{
    println!("SHA2_224_new");
    use Cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7");
    println!("-------------------------------");
}

fn SHA2_224_digest()
{
    println!("SHA2_224_digest");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_224::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "279C669E4411237490589A794FC2F0F8E256F8FBC58C520601ABF45B");
    println!("-------------------------------");
}

fn SHA2_224_digest_str()
{
    println!("SHA2_224_digest_str");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5E3731968A757FDFD99F9C9437B1BA2443A66065B362F230AA041C06");
    println!("-------------------------------");
}

fn SHA2_224_digest_string()
{
    println!("SHA2_224_digest_string");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_224::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C56B613F567923DC558D7BD4D68A88875DD206C53BCC7329448508FA");
    println!("-------------------------------");
}

fn SHA2_224_digest_array()
{
    println!("SHA2_224_digest_array");
    use Cryptocol::hash::SHA2_224;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_224::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "80BB5B27988D4C3E8FFA4429A4D01175498EC57BAE6B9E856A37837B");
    println!("-------------------------------");
}

fn SHA2_224_digest_vec()
{
    println!("SHA2_224_digest_vec");
    use Cryptocol::hash::SHA2_224;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_224::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "80BB5B27988D4C3E8FFA4429A4D01175498EC57BAE6B9E856A37837B");
    println!("-------------------------------");
}

fn SHA2_224_get_HashValue()
{
    println!("SHA2_224_get_HashValue");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 28];
    let mut hash = SHA2_224::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[0A, 7F, 91, AB, 7F, FE, 49, F7, 94, 6A, 87, BC, 10, AA, E2, DF, 04, 07, D3, CD, C6, EE, B1, 8F, E5, 96, 20, 29]");
    println!("-------------------------------");
}

fn SHA2_224_get_HashValue_in_string()
{
    println!("SHA2_224_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "BDF28C3554AD62AF7AFAD9EEF51E1480F854A83F64F4EB2FB0F15612");
    println!("-------------------------------");
}

fn SHA2_224_get_HashValue_in_array()
{
    println!("SHA2_224_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[DEBDEDF6, 18DDA1DB, 918876D0, CCFB08F2, 6FC8CC91, 42F16E75, 8950C7C2]");
    println!("-------------------------------");
}

fn SHA2_224_get_HashValue_in_vec()
{
    println!("SHA2_224_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[E2C590F1, 517DBC0B, 231F9798, BB87477C, C34B4596, DEC566B5, 96923BFC]");
    println!("-------------------------------");
}

fn SHA2_224_fmt_for_to_string()
{
    println!("SHA2_224_fmt_for_to_string");
    use Cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "979DB3C5F63C2FBB32A72804A991534EB38884EB5B9131AE0EE3A496");
    println!("-------------------------------");
}

fn SHA2_224_fmt_for_println()
{
    println!("SHA2_224_fmt_for_println");
    use Cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E333EE19A56FCDCCB05957F2B6FB0AD1EA11D7B6258DF28DCE3B526B");
    println!("-------------------------------");
}

fn Hash_SHA2_512_main()
{
    SHA2_512_Quick_Start();
    SHA2_512_new();
    SHA2_512_digest();
    SHA2_512_digest_str();
    SHA2_512_digest_string();
    SHA2_512_digest_array();
    SHA2_512_digest_vec();
    SHA2_512_get_HashValue();
    SHA2_512_get_HashValue_in_string();
    SHA2_512_get_HashValue_in_array();
    SHA2_512_get_HashValue_in_vec();
    SHA2_512_fmt_for_to_string();
    SHA2_512_fmt_for_println();
}

fn SHA2_512_Quick_Start()
{
    println!("SHA2_512_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "CF83E1357EEFB8BDF1542850D66D8007D620E4050B5715DC83F4A921D36CE9CE47D0D13C5D85F2B0FF8318D2877EEC2F63B931BD47417A81A538327AF927DA3E");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "21B4F4BD9E64ED355C3EB676A28EBEDAF6D8F17BDC365995B319097153044080516BD083BFCCE66121A3072646994C8430CC382B8DC543E84880183BF856CFF5");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "1F7B98E0B65D4CD1DE4C2B9EC77CB5F7FC4C20006BDD1380F7E2A9C95BD5F47162B868E724FEC68ECE02B8C3F79BE0C4AB73EEF8AC84B5537063B1A353074735");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "EE74E2DD1FFBFC17A48C80FCBF6A0C55D0A0B4E4F94EDEF7506D28D48EAA5F4DDD7490B3A9CAF212C0CE2101ADABF0C32E4BD91694B8B284C5FAAFE6F54B63D7");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "3893170A81639FB341477704BFB1CDBB8A222F8DAE98B28505EF0552B86DCE1D630ED80DF6CB34D69CD62ADBD88EADD26B550FC9C3CCD7DEFDE4C71594108348");

    txt = "This algorithm SHA-2/512 is being tested with this message the length of which is one hundred twelve bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "134BDEE3708226A589B53F23ED4559203B603D61239B6EBAA4625D4A95ACC5F98182D615194D4035E3CFED16916477C18889E64980A35263B62B8361131B01D4");

    txt = "This algorithm SHA-2/512 is being tested for this message the length of which is one hundred sixty-five long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "38954039BEA7BFF8DD1DA6E6672A68BD8642F5C4DB7C7CFE11DB2D339DC8FA4EBBC4F1BDC11B4FEC71CB9C84B55FBA85CB11EC4BF72937232044BD004CC90CE7");

    txt = "This algorithm SHA-2/512 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BB87AF8EFB6903DD0FA504AA12E223B00224FF1B6ABF568D7E9C53F65639242E3C0C635A44490D20B4C4B27E748A7675F0C973B6F2784B1105BAFEB91293F0BC");
    println!("-------------------------------");
}

fn SHA2_512_new()
{
    println!("SHA2_512_new");
    use Cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6A09E667F3BCC908BB67AE8584CAA73B3C6EF372FE94F82BA54FF53A5F1D36F1510E527FADE682D19B05688C2B3E6C1F1F83D9ABFB41BD6B5BE0CD19137E2179");
    println!("-------------------------------");
}

fn SHA2_512_digest()
{
    println!("SHA2_512_digest");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_512::new();
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7DE9DD6917A9B3353EA426F9C43894513095E944DBE0678491DDABAC0D87236E007374B7438231AB84DE91271F1BCCD11BA64AEC24E3BDC410DE1115A075E2DC");
    println!("-------------------------------");
}

fn SHA2_512_digest_str()
{
    println!("SHA2_512_digest_str");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5FD3D145014F7886E64034CC082ADF48670E797DA1C2DA54DDEAF5513E028EB3712121FE6305DB6524C12CBBBB93DF3C0A4DA54E8D798E2AC2A29BA81FB3BFD9");
    println!("-------------------------------");
}

fn SHA2_512_digest_string()
{
    println!("SHA2_512_digest_string");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_512::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "768AF82F599E230387C1F4A4BA2F97F59C6C96B76735A61CFFF3300E808EE0D9FF497957456BB61AABD0F88C19790F0675DD586DC0F5722C60DCB5BB27D6853B");
    println!("-------------------------------");
}

fn SHA2_512_digest_array()
{
    println!("SHA2_512_digest_array");
    use Cryptocol::hash::SHA2_512;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4582725B4E904C9FB7C4072B2E4665FB3E4ADC03CB8061C91C0283D582251EA08226CF5A84D9EE441FB30ADB0AB7E564DD66CE8A2BC2BA4B0E32AD36E3BB1253");
    println!("-------------------------------");
}

fn SHA2_512_digest_vec()
{
    println!("SHA2_512_digest_vec");
    use Cryptocol::hash::SHA2_512;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4582725B4E904C9FB7C4072B2E4665FB3E4ADC03CB8061C91C0283D582251EA08226CF5A84D9EE441FB30ADB0AB7E564DD66CE8A2BC2BA4B0E32AD36E3BB1253");
    println!("-------------------------------");
}

fn SHA2_512_get_HashValue()
{
    println!("SHA2_512_get_HashValue");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 64];
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[C9, 9F, F4, 6D, D1, 68, 11, 1D, 0C, 7A, 3E, 16, 5C, BB, 06, 40, D6, DE, 58, 8F, E8, 7D, DB, 29, A1, 3C, 44, CE, FF, B3, E8, 0E, C7, 7C, F0, 09, 8A, 2E, DF, 2F, 53, 32, 6F, 56, 73, AD, 8E, 3F, 87, 0F, 12, 60, CD, B1, 50, DA, 65, B9, 57, 02, C9, CA, 34, 18]");
    println!("-------------------------------");
}

fn SHA2_512_get_HashValue_in_string()
{
    println!("SHA2_512_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "C5B844FE2254A12B6C59A0F532DD91FE3B3C00228D21BFBE3C2005FE0F10677BEEAC811DA09AC12CC5E0B8F5C7DB80BE93672891C7D59D19E6BA03BB645E8A6A");
    println!("-------------------------------");
}

fn SHA2_512_get_HashValue_in_array()
{
    println!("SHA2_512_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[DF2061589C2CD7CB, BDED67828A40F664, 23ABD9849879BB3C, 45F70A5F7BA2E217, 666F4D1842369657, 99727C580358CA62, 1BDE02FE67C79A69, F86777E8508F7358]");
    println!("-------------------------------");
}

fn SHA2_512_get_HashValue_in_vec()
{
    println!("SHA2_512_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[20F75D6A9595036B, 78467C5E0D27C702, A29A92B4B48FAB0, D94AF909AB89A4FC, 4889F1A8498D68CF, 6478B4272EE058DC, 3FD28DF6187A3DB4, F060D07971BFCFF0]");
    println!("-------------------------------");
}

fn SHA2_512_fmt_for_to_string()
{
    println!("SHA2_512_fmt_for_to_string");
    use Cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "4253800692B979FD12F63DD77380BF391AAEC2FB7C599BD447A6E9690F1E7CC06ED615C61CB27514B64F56ACD423A3AC6BE2AEB637885786CE720F1516E38BAD");
    println!("-------------------------------");
}

fn SHA2_512_fmt_for_println()
{
    println!("SHA2_512_fmt_for_println");
    use Cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E960F9A2E39AEBB1C1654B5B7408819AE4507F6983F2D592F232CB64C2CD4DB7265DBF5BCDE9DADA7A1B26618E55B39E54C7A9EC6777B5DA70132160C8E4C837");
    println!("-------------------------------");
}

fn Hash_SHA2_384_main()
{
    SHA2_384_Quick_Start();
    SHA2_384_new();
    SHA2_384_digest();
    SHA2_384_digest_str();
    SHA2_384_digest_string();
    SHA2_384_digest_array();
    SHA2_384_digest_vec();
    SHA2_384_get_HashValue();
    SHA2_384_get_HashValue_in_string();
    SHA2_384_get_HashValue_in_array();
    SHA2_384_get_HashValue_in_vec();
    SHA2_384_fmt_for_to_string();
    SHA2_384_fmt_for_println();
}

fn SHA2_384_Quick_Start()
{
    println!("SHA2_384_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "38B060A751AC96384CD9327EB1B1E36A21FDB71114BE07434C0CC7BF63F6E1DA274EDEBFE76F65FBD51AD2F14898B95B");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "AD14AAF25020BEF2FD4E3EB5EC0C50272CDFD66074B0ED037C9A11254321AAC0729985374BEEAA5B80A504D048BE1864");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "63DD51EE49AEDD57E85F8BF9A9CF53595FF212BF2E334845AC14CAD17F137C2221D065F8143FB39D3EB2612DD4B429CC");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "813660BD8EABF78896F5F33727067071635BDACE0E81C158E32E7EB3626820887C42F83E6D9CE973B71B6A0C50C753C0");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "435138D7CC9CE82E375B13FE3C75301EB670A8BAFDE4A1952D8D33225E464A62D5747F66F68C8289C5E8BB4C45E16A42");

    txt = "This algorithm SHA-2/384 is being tested with this message the length of which is one hundred twelve bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "31D21701C66D9226B1ECEEB713100ECE0C06A1DDCA1EA5B66286316E31B288C7E5147A796195C1A2D93006FFB5372B74");

    txt = "This algorithm SHA-2/384 is being tested for this message the length of which is one hundred sixty-five long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "8B2E5B6105E561A42FC0BE177FEB784321FC67A5024456A48C6A4B481FE483AA3F57A7F57FAE41471362870892465627");

    txt = "This algorithm SHA-2/384 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "6FE78FC9871EBAF5F19777B7C47B49DB400154DC58684808F06C47CAD1428B46C8AFF2F77C438CCFF287DCA6C60C72FC");
    println!("-------------------------------");
}

fn SHA2_384_new()
{
    println!("SHA2_384_new");
    use Cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "CBBB9D5DC1059ED8629A292A367CD5079159015A3070DD17152FECD8F70E593967332667FFC00B318EB44A8768581511");
    println!("-------------------------------");
}

fn SHA2_384_digest()
{
    println!("SHA2_384_digest");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_384::new();
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "1EFF9CDB108E9FC430650DC0A8FB7195654B225B013ECF90F7949077A299D04A921997536D0E1941734ED63FA68AF5E2");
    println!("-------------------------------");
}

fn SHA2_384_digest_str()
{
    println!("SHA2_384_digest_str");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_384::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C1C8355C211B2DF4D562014768ECDF21973D60A25EC0C1038C11510E9996084F4871C15A3578BECDF6EAF2F62A8A56C1");
    println!("-------------------------------");
}

fn SHA2_384_digest_string()
{
    println!("SHA2_384_digest_string");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_384::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "19EA6204374E0C4DB800813E7665350754E7B5E5E3A2FC9B95F3F164D7F1E0493D48F2C4ECC32E2F147EB7789F35B9A4");
    println!("-------------------------------");
}

fn SHA2_384_digest_array()
{
    println!("SHA2_384_digest_array");
    use Cryptocol::hash::SHA2_384;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_384::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9F1D9A9407C018C8D95E4CFBC839121AA45521AC2C6AE0F3140E657A1A55384D7F32ACCBD5FCABC27DD7499DC3DB3F6C");
    println!("-------------------------------");
}

fn SHA2_384_digest_vec()
{
    println!("SHA2_384_digest_vec");
    use Cryptocol::hash::SHA2_384;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_384::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9F1D9A9407C018C8D95E4CFBC839121AA45521AC2C6AE0F3140E657A1A55384D7F32ACCBD5FCABC27DD7499DC3DB3F6C");
    println!("-------------------------------");
}

fn SHA2_384_get_HashValue()
{
    println!("SHA2_384_get_HashValue");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 48];
    let mut hash = SHA2_384::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[BF, C6, 22, 65, 13, A5, 81, E7, 08, 3E, 78, 32, D5, C5, 91, 50, 5B, C8, 4F, 67, 5B, AF, 5D, 39, F1, B9, 06, 0F, 72, 2F, 57, E9, 5C, D1, 18, F3, 5C, 00, CE, BD, 77, 21, 09, 63, 6C, CC, 6C, 7F]");
    println!("-------------------------------");
}

fn SHA2_384_get_HashValue_in_string()
{
    println!("SHA2_384_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_384::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "01A9DB874F3DFC2C68307813BF93C6DA38E78DB69B9ECDB9D52B4A294B40C5A5FB66887179289A11D282AD203A7A5A3E");
    println!("-------------------------------");
}

fn SHA2_384_get_HashValue_in_array()
{
    println!("SHA2_384_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_384::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[18B2F598DD216043, C79769E9313C4A07, 75FF5D2AA370B3AD, 7C08DDEA25D3DC55, 33E87C2233400798, 6EB87DCE52E67FCB]");
    println!("-------------------------------");
}

fn SHA2_384_get_HashValue_in_vec()
{
    println!("SHA2_384_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_384::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[7656E133C42F0A53, 738ED067E15A69D0, E901430889B02383, 909FC0A1EEF00A70, CCB39A7A8482F2B2, 9FF389A078F3A041]");
    println!("-------------------------------");
}

fn SHA2_384_fmt_for_to_string()
{
    println!("SHA2_384_fmt_for_to_string");
    use Cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "20109C9D8199547993C91DCC64C07771605EEBC0AADD939E84B98C54C4CCF419B0CD73D5C1D4178902C9CD115077656C");
    println!("-------------------------------");
}

fn SHA2_384_fmt_for_println()
{
    println!("SHA2_384_fmt_for_println");
    use Cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "222E7F7B1DB2A566D0665D5790B2A4373F006850F06C1E3E83CE6021AF8761BC738BBF10F75A8E45BA09BDB0814DD8E6");
    println!("-------------------------------");
}

fn Hash_SHA2_512_256_main()
{
    SHA2_512_256_Quick_Start();
    SHA2_512_256_new();
    SHA2_512_256_digest();
    SHA2_512_256_digest_str();
    SHA2_512_256_digest_string();
    SHA2_512_256_digest_array();
    SHA2_512_256_digest_vec();
    SHA2_512_256_get_HashValue();
    SHA2_512_256_get_HashValue_in_string();
    SHA2_512_256_get_HashValue_in_array();
    SHA2_512_256_get_HashValue_in_vec();
    SHA2_512_256_fmt_for_to_string();
    SHA2_512_256_fmt_for_println();
}

fn SHA2_512_256_Quick_Start()
{
    println!("SHA2_512_256_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "C672B8D1EF56ED28AB87C3622C5114069BDD3AD7B8F9737498D0C01ECEF0967A");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "65A992AD19967492B5780D76A4733AF553F796F688B79102D01EC7FDE5590CAB");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "E4AF36E824AFDB9E42291983AFA292B894DED2CCAFCCF53346B223FCA846694D");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4E730BDADF49EC9F3E920F72EAD3AC8D09B459900BE4F6E27848652632277205");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "AE0EAB6824897F575FCC051DBC2D1AA7F7BF0DB2C80172F639CE20B3B498C9D5");

    txt = "This algorithm SHA-2/512/256 is being tested with this message, the length of which is one hundred twelve bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "7876C6F1285C4B6EC6A2F4A76BBF81815B470536F3A38B7028AA88A3C5C31651");

    txt = "This algorithm SHA-2/512/256 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "6FCE377EA6116BEAC9C11606C59A5D034C8C6EF5A1920B783A9097E07BE36D31");

    txt = "This algorithm SHA-2/512/256 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "63FD06E11EF67F0F5EF598C3B2F2E221D5557AD1EEA46156D1B657F1EDF08D5D");
    println!("-------------------------------");
}

fn SHA2_512_256_new()
{
    println!("SHA2_512_256_new");
    use Cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");
    println!("-------------------------------");
}

fn SHA2_512_256_digest()
{
    println!("SHA2_512_256_digest");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_512_256::new();
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");
    println!("-------------------------------");
}

fn SHA2_512_256_digest_str()
{
    println!("SHA2_512_256_digest_str");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_512_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D0ED13389E431C8D74FE6E8DB5B6194682874B52E800524136E35D7E9CFA496B");
    println!("-------------------------------");
}

fn SHA2_512_256_digest_string()
{
    println!("SHA2_512_256_digest_string");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_512_256::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "2824B79B5D8A1D02454612B72B9CD9544D0DF8E126E7A01E55AC479B0903297C");
    println!("-------------------------------");
}

fn SHA2_512_256_digest_array()
{
    println!("SHA2_512_256_digest_array");
    use Cryptocol::hash::SHA2_512_256;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512_256::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
    println!("-------------------------------");
}

fn SHA2_512_256_digest_vec()
{
    println!("SHA2_512_256_digest_vec");
    use Cryptocol::hash::SHA2_512_256;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512_256::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
    println!("-------------------------------");
}

fn SHA2_512_256_get_HashValue()
{
    println!("SHA2_512_256_get_HashValue");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 32];
    let mut hash = SHA2_512_256::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[6B, CD, 05, 58, 76, E2, E2, 10, E1, BA, 59, 18, 0A, A8, 04, 8B, 49, 86, CB, 12, E2, 56, 1F, DB, 26, 0A, 0F, 0C, 25, 8F, 88, DD]");
    println!("-------------------------------");
}

fn SHA2_512_256_get_HashValue_in_string()
{
    println!("SHA2_512_256_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_512_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "CFEEC9D8BAB717BBD52B89202B0D671603C6841A0D81FF08C9E1AC60AAD2D038");
    println!("-------------------------------");
}

fn SHA2_512_256_get_HashValue_in_array()
{
    println!("SHA2_512_256_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_512_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[84D92CCECF19A8E1, F10F35786EEE5BC5, B58793B76661CF2, 56EED9A53EDF76F4]");
    println!("-------------------------------");
}

fn SHA2_512_256_get_HashValue_in_vec()
{
    println!("SHA2_512_256_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_512_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[3D61494146786869, A70B85EEF50F080B, AC0C9758D1CD8A32, 776CD6411B2BBAAA]");
    println!("-------------------------------");
}

fn SHA2_512_256_fmt_for_to_string()
{
    println!("SHA2_512_256_fmt_for_to_string");
    use Cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "5ED309022841125DE856B25C56A741166872A1D681DF5C69F84AD8B2F30E6DD8");
    println!("-------------------------------");
}

fn SHA2_512_256_fmt_for_println()
{
    println!("SHA2_512_256_fmt_for_println");
    use Cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "660F8CA5DDC61C43BCEBAB6B8FFD4081F9015CE9A7800BFE29B5100709C3E232");
    println!("-------------------------------");
}

fn Hash_SHA2_512_t_256_main()
{
    SHA2_512_t_256_Quick_Start();
    SHA2_512_t_256_new();
    SHA2_512_t_256_digest();
    SHA2_512_t_256_digest_str();
    SHA2_512_t_256_digest_string();
    SHA2_512_t_256_digest_array();
    SHA2_512_t_256_digest_vec();
    SHA2_512_t_256_get_HashValue();
    SHA2_512_t_256_get_HashValue_in_string();
    SHA2_512_t_256_get_HashValue_in_array();
    SHA2_512_t_256_get_HashValue_in_vec();
    SHA2_512_t_256_fmt_for_to_string();
    SHA2_512_t_256_fmt_for_println();
}

fn SHA2_512_t_256_Quick_Start()
{
    println!("SHA2_512_t_256_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "C672B8D1EF56ED28AB87C3622C5114069BDD3AD7B8F9737498D0C01ECEF0967A");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "65A992AD19967492B5780D76A4733AF553F796F688B79102D01EC7FDE5590CAB");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "E4AF36E824AFDB9E42291983AFA292B894DED2CCAFCCF53346B223FCA846694D");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4E730BDADF49EC9F3E920F72EAD3AC8D09B459900BE4F6E27848652632277205");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "AE0EAB6824897F575FCC051DBC2D1AA7F7BF0DB2C80172F639CE20B3B498C9D5");

    txt = "This algorithm SHA-2/512/256 is being tested with this message, the length of which is one hundred twelve bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "7876C6F1285C4B6EC6A2F4A76BBF81815B470536F3A38B7028AA88A3C5C31651");

    txt = "This algorithm SHA-2/512/256 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "6FCE377EA6116BEAC9C11606C59A5D034C8C6EF5A1920B783A9097E07BE36D31");

    txt = "This algorithm SHA-2/512/256 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "63FD06E11EF67F0F5EF598C3B2F2E221D5557AD1EEA46156D1B657F1EDF08D5D");
    println!("-------------------------------");
}

fn SHA2_512_t_256_new()
{
    println!("SHA2_512_t_256_new");
    use Cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");
    println!("-------------------------------");
}

fn SHA2_512_t_256_digest()
{
    println!("SHA2_512_t_256_digest");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_512_t_256::new();
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");
    println!("-------------------------------");
}

fn SHA2_512_t_256_digest_str()
{
    println!("SHA2_512_t_256_digest_str");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_512_t_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D0ED13389E431C8D74FE6E8DB5B6194682874B52E800524136E35D7E9CFA496B");
    println!("-------------------------------");
}

fn SHA2_512_t_256_digest_string()
{
    println!("SHA2_512_t_256_digest_string");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_512_t_256::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "2824B79B5D8A1D02454612B72B9CD9544D0DF8E126E7A01E55AC479B0903297C");
    println!("-------------------------------");
}

fn SHA2_512_t_256_digest_array()
{
    println!("SHA2_512_t_256_digest_array");
    use Cryptocol::hash::SHA2_512_t_256;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512_t_256::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
    println!("-------------------------------");
}

fn SHA2_512_t_256_digest_vec()
{
    println!("SHA2_512_t_256_digest_vec");
    use Cryptocol::hash::SHA2_512_t_256;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512_t_256::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
    println!("-------------------------------");
}

fn SHA2_512_t_256_get_HashValue()
{
    println!("SHA2_512_t_256_get_HashValue");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 32];
    let mut hash = SHA2_512_t_256::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[6B, CD, 05, 58, 76, E2, E2, 10, E1, BA, 59, 18, 0A, A8, 04, 8B, 49, 86, CB, 12, E2, 56, 1F, DB, 26, 0A, 0F, 0C, 25, 8F, 88, DD]");
    println!("-------------------------------");
}

fn SHA2_512_t_256_get_HashValue_in_string()
{
    println!("SHA2_512_t_256_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_512_t_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "CFEEC9D8BAB717BBD52B89202B0D671603C6841A0D81FF08C9E1AC60AAD2D038");
    println!("-------------------------------");
}

fn SHA2_512_t_256_get_HashValue_in_array()
{
    println!("SHA2_512_t_256_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_512_t_256::new();
    hash.digest_str(txt);
    let mut h = [0_u64; 4];
    hash.get_HashValue_in_array(&mut h);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, h);
    assert_eq!(format!("{:02X?}", h), "[84D92CCECF19A8E1, F10F35786EEE5BC5, B58793B76661CF2, 56EED9A53EDF76F4]");
    println!("-------------------------------");
}

fn SHA2_512_t_256_get_HashValue_in_vec()
{
    println!("SHA2_512_t_256_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_512_t_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[3D61494146786869, A70B85EEF50F080B, AC0C9758D1CD8A32, 776CD6411B2BBAAA]");
    println!("-------------------------------");
}

fn SHA2_512_t_256_fmt_for_to_string()
{
    println!("SHA2_512_t_256_fmt_for_to_string");
    use Cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "5ED309022841125DE856B25C56A741166872A1D681DF5C69F84AD8B2F30E6DD8");
    println!("-------------------------------");
}

fn SHA2_512_t_256_fmt_for_println()
{
    println!("SHA2_512_t_256_fmt_for_println");
    use Cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "660F8CA5DDC61C43BCEBAB6B8FFD4081F9015CE9A7800BFE29B5100709C3E232");
    println!("-------------------------------");
}

fn Hash_SHA2_512_t_224_main()
{
    SHA2_512_t_224_Quick_Start();
    SHA2_512_t_224_new();
    SHA2_512_t_224_digest();
    SHA2_512_t_224_digest_str();
    SHA2_512_t_224_digest_string();
    SHA2_512_t_224_digest_array();
    SHA2_512_t_224_digest_vec();
    SHA2_512_t_224_get_HashValue();
    SHA2_512_t_224_get_HashValue_in_string();
    SHA2_512_t_224_get_HashValue_in_array();
    SHA2_512_t_224_get_HashValue_in_vec();
    SHA2_512_t_224_fmt_for_to_string();
    SHA2_512_t_224_fmt_for_println();
}

fn SHA2_512_t_224_Quick_Start()
{
    println!("SHA2_512_t_224_Quick_Start");
    use std::string::*;
    use Cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "6ED0DD02806FA89E25DE060C19D3AC86CABB87D6A0DDD05C333B84F4");

    let txtStirng = String::from("A");
    hash.digest_string(&txtStirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
    assert_eq!(hash.to_string(), "1DEF1E6A5344538A07A3C93A3A765FA1D2859A576947791A9047C3E6");

    let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txtArray);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
    assert_eq!(hash.get_HashValue_in_string(), "021B7E0CFE3FBD598CF0366464AEB4C93A900BBA1DF8CADB5F611345");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "1E6EEBF17E8B2B1D2A41B14D9813561E44814E35F01119ED7BA3E19F");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "5251D628FE99DA19238D277DF9AC03382249FF3830AD764EF0A68CDA");

    txt = "This algorithm SHA-2/512/224 is being tested with this message, the length of which is one hundred twelve bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "225B3D39D9B91705E7C08DBBF66E5F34E88554685C78AF2535FD3CE2");

    txt = "This algorithm SHA-2/512/224 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_HashValue_in_string(), "3DD5D6503AFE8247B37AFD72DFD56718E6CA70D0B425739928885D0F");

    txt = "This algorithm SHA-2/512/224 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D709EC6C2CAA1DAC61B0121675C3B131C23209F9E9ABC60392D99F52");
    println!("-------------------------------");
}

fn SHA2_512_t_224_new()
{
    println!("SHA2_512_t_224_new");
    use Cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "8C3D37C819544DA273E1996689DCD4D61DFAB7AE32FF9C82679DD514");
    println!("-------------------------------");
}

fn SHA2_512_t_224_digest()
{
    println!("SHA2_512_t_224_digest");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_512_t_224::new();
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "2269C5A3791E72D00337D9EDDE9BA9568539F4E131B7DB7555545633");
    println!("-------------------------------");
}

fn SHA2_512_t_224_digest_str()
{
    println!("SHA2_512_t_224_digest_str");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method digest_str().";
    let mut hash = SHA2_512_t_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "17E80E466E706474DB2C9E39691150805AC536319125AFB1E436BE8F");
    println!("-------------------------------");
}

fn SHA2_512_t_224_digest_string()
{
    println!("SHA2_512_t_224_digest_string");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = SHA2_512_t_224::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E1423096CED4DC8D9522C75C8BBB12B59A4510093CFA4FD480D270FD");
    println!("-------------------------------");
}

fn SHA2_512_t_224_digest_array()
{
    println!("SHA2_512_t_224_digest_array");
    use Cryptocol::hash::SHA2_512_t_224;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512_t_224::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "3F600A922240910231ACA350DEDD49BD875936BE5AAB8A034D09334B");
    println!("-------------------------------");
}

fn SHA2_512_t_224_digest_vec()
{
    println!("SHA2_512_t_224_digest_vec");
    use Cryptocol::hash::SHA2_512_t_224;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_512_t_224::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "3F600A922240910231ACA350DEDD49BD875936BE5AAB8A034D09334B");
    println!("-------------------------------");
}

fn SHA2_512_t_224_get_HashValue()
{
    println!("SHA2_512_t_224_get_HashValue");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 28];
    let mut hash = SHA2_512_t_224::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[65, 12, 55, 2D, 41, F9, 6A, 38, 89, 91, FB, E2, 68, 1F, 0E, F5, 86, E0, 08, 8D, 81, 36, DB, 27, 7C, 7D, 96, 42]");
    println!("-------------------------------");
}

fn SHA2_512_t_224_get_HashValue_in_string()
{
    println!("SHA2_512_t_224_get_HashValue_in_string");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = SHA2_512_t_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "B030A1B50A7C3886A0DCFB53C2D65404C6DD3B59F19CE6850D413181");
    println!("-------------------------------");
}

fn SHA2_512_t_224_get_HashValue_in_array()
{
    println!("SHA2_512_t_224_get_HashValue_in_array");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = SHA2_512_t_224::new();
    hash.digest_str(txt);
    let mut h = [0_u32; 7];
    hash.get_HashValue_in_array(&mut h);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, h);
    assert_eq!(format!("{:02X?}", h), "[D79CD72B, 7CE44470, 5035DFA8, B50B1D54, AF732B81, 369AFB07, 5010FD03]");
    println!("-------------------------------");
}

fn SHA2_512_t_224_get_HashValue_in_vec()
{
    println!("SHA2_512_t_224_get_HashValue_in_vec");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = SHA2_512_t_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[208DF702B905BB5D, 49169B1240A9883, 72C4A5A7D4FD5141, D8081D6200000000]");
    println!("-------------------------------");
}

fn SHA2_512_t_224_fmt_for_to_string()
{
    println!("SHA2_512_t_224_fmt_for_to_string");
    use Cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "0FFD651E288004466FF247808E1FF5B482AFF547E94C66FF507BF021");
    println!("-------------------------------");
}

fn SHA2_512_t_224_fmt_for_println()
{
    println!("SHA2_512_t_224_fmt_for_println");
    use Cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7988DFC3FB4BB8DB449B189C5D906901921C1AC0D60D94376B498795");
    println!("-------------------------------");
}
