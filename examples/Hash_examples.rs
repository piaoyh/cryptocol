// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]


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
    // Hash_OS_Rng_main();
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
    MD4_put_HashValue_in_array();
    MD4_tangle();
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
    // Example for MD4
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = myMD4::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    println!("-------------------------------");
}

fn MD4_digest()
{
    println!("MD4_digest");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method digest().";
    let mut hash = MD4::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A18836F660C3C66B8CBEE4BD24FEFFA9");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = myMD4::new();
    let txt = "This is an example of the method digest().";
    let mut my_hash = myMD4::new();
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "B2F465006DCBA147BCE76D7EB8B564E1");
    println!("-------------------------------");
}

fn MD4_digest_str()
{
    println!("MD4_digest_str");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method digest_str().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method digest_str().";
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "719A1EB0F5077837BB408434B7AAD81E");
    println!("-------------------------------");
}

fn MD4_digest_string()
{
    println!("MD4_digest_string");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut hash = MD4::new();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DF23C7808B2B158C5E2D8C9FE1FF2ECC");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut my_hash = myMD4::new();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FD42F7479ED133619D877BB1E6C8A084");
    println!("-------------------------------");
}

fn MD4_digest_array()
{
    println!("MD4_digest_array");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = MD4::new();
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = myMD4::new();
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    println!("-------------------------------");
}

fn MD4_digest_vec()
{
    println!("MD4_digest_vec");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = MD4::new();
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = myMD4::new();
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    println!("-------------------------------");
}

fn MD4_get_HashValue()
{
    println!("MD4_get_HashValue");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 16];
    let mut hash = MD4::new();
    hash.digest_str(txt);
    hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[12, E1, F9, 39, C3, D8, 09, A7, 23, 7D, 94, B4, F1, A0, 1E, FD]");
 
    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 16];
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    my_hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[62, 62, E3, D7, 06, 11, F5, 55, F9, 1D, 64, 78, C3, 32, C2, B8]");
    println!("-------------------------------");
}

fn MD4_get_HashValue_in_string()
{
    println!("MD4_get_HashValue_in_string");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    assert_eq!(hash.get_HashValue_in_string(), "B871AC07999486EB0A6450DA5E09E92D");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_HashValue_in_string());
    assert_eq!(my_hash.get_HashValue_in_string(), "7FC6C3F917CCA507B21D05B67F3E549B");
    println!("-------------------------------");
}

fn MD4_get_HashValue_in_array()
{
    println!("MD4_get_HashValue_in_array");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[9F7E4FD8, 906C5422, 9FAAAFBA, 363BE03A]");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[E68DA94C, 583C881E, A7D2A6F5, 5BC4347F]");
    println!("-------------------------------");
}

fn MD4_get_HashValue_in_vec()
{
    println!("MD4_get_HashValue_in_vec");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_vec());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_vec()), "[6BD261A9, 47EFE9B7, 04397C1B, 628A61D7]");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_vec()), "[093C7233, 97776D39, 5BFDEFCD, 3F679BFF]");
    println!("-------------------------------");
}

fn MD4_put_HashValue_in_array()
{
    println!("MD4_put_HashValue_in_array");
    use Cryptocol::hash::MD4;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = MD4::new();
    let mut hash_code = [0_u32; 4];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[E4046C5C, 06735474, 4152BA95, 9A4261D8]");

    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = myMD4::new();
    let mut hash_code = [0_u32; 4];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[673E3933, 1F45BBC6, F874BCF9, 6C70ED89]");
    println!("-------------------------------");
}

fn MD4_tangle()
{
    println!("MD4_tangle");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let txt = "TANGLING";
    let mut hash = MD4::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[BC65D6E1, F0F37B4E, 2F404331, A8F25E2A]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[CE1E07A3, F3373D70, 95A8F098, 9BC7894E]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[5B9A2D14, 64888002, 15282E23, E5B2F4BD]");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "TANGLING";
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[C4377D49, 05FD9A1F, 3DA4E254, ACF22116]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[8CB0AF83, F75E073C, 77C5BF6C, EDFE1D51]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[A5C900D1, 388193FA, B2C0ED53, 4DE71DDE]");
    println!("-------------------------------");
}

fn MD4_fmt_for_to_string()
{
    println!("MD4_fmt_for_to_string");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "E2244B71E17D5BD7E1CCEB58C8F8C82E");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "Display::fmt() automagically implement to_string().";
    let mut my_hash = myMD4::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "6B0D8F0CE90782E5FF88EE57B5DC5AF1");
    println!("-------------------------------");
}

fn MD4_fmt_for_println()
{
    println!("MD4_fmt_for_println");
    // Example for MD4
    use Cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3C607B5548C155DCF4E84C7A6C21D349");

    // Example for MD4_Expanded
    use Cryptocol::hash::MD4_Expanded;
    type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = myMD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "745B42127EC2479032923F2EE368FD92");
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
    MD5_put_HashValue_in_array();
    MD5_tangle();
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = myMD5::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let txt = "This is an example of the method digest().";
    let mut my_hash = myMD5::new();
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "51F4248FBEFBE0A00196F9F04DD07FF0");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method digest_str().";
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "21EE03C8185BD65CDB8116D0E2714F09");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut my_hash = myMD5::new();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "02BDBC510B949045A131C0C3302027BA");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = myMD5::new();
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = myMD5::new();
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 16];
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    my_hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[2F, E0, 49, D9, 9C, 33, C0, DC, 6A, 8B, 4F, 3B, C6, 31, 68, 71]");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_HashValue_in_string());
    assert_eq!(my_hash.get_HashValue_in_string(), "1D850D022B0B079C896180B796E7B424");

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
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[A4BE6EEF, C9A5DFBA, 558B5ADF, 3B1035F9]");

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[67A6CAD6, 4B138BB8, 846C082C, 8ABDFE02]");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_vec()), "[E02B8514, CC2B4041, 38CBFA58, 1E6B3F51]");
    println!("-------------------------------");
}

fn MD5_put_HashValue_in_array()
{
    println!("MD5_put_HashValue_in_array");
    use Cryptocol::hash::MD5;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = MD5::new();
    let mut hash_code = [0_u32; 4];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[A2D690F9, CA6253E5, 2CB87DC4, 0ADF1A33]");

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = myMD5::new();
    let mut hash_code = [0_u32; 4];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[39B83B83, C327EE5E, 621A0669, A43A572A]");
    println!("-------------------------------");
}

fn MD5_tangle()
{
    println!("MD5_tangle");
    use Cryptocol::hash::MD5;
    let txt = "TANGLING";
    let mut hash = MD5::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[E60545F6, 6DCF2B02, 8245048B, AE2A98C6]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[E0B5F1C0, 5C62629F, 68D44BC1, D384AB34]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[C75EEA9C, 9D5CF62B, 0ABFA634, CD29C2D4]");

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "TANGLING";
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[9CCE671A, 5366F625, 68056532, D6B0DA5C]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[A12380BC, DE74206D, C145732C, 4CAAD502]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[D9EB87F4, 00C2D299, A492A483, 1C24FCDD]");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "Display::fmt() automagically implement to_string().";
    let mut my_hash = myMD5::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "3FDFF3827C89F3C770A0863F069FE766");
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

    use Cryptocol::hash::MD5_Expanded;
    type myMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = myMD5::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "45BA0E4FEA1FACF829D19544A77105B8");
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
    SHA1_put_HashValue_in_array();
    SHA1_tangle();
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = mySHA1::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888CCCCCCCCFFFFFFFF");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method digest().";
    let mut my_hash = mySHA1::new();
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FAF77A15CDEDFC6A33CE2C4003B119F225CBE414");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method digest_str().";
    let mut my_hash = mySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "A6BE8FEA7E3F61508DC0A8BA85A0AEC77D0C0784");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut my_hash = mySHA1::new();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "F4FE5C5A4D2A4BD414DDDF1FD32B185F3ED8AA32");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA1::new();
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA1::new();
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 20];
    let mut my_hash = mySHA1::new();
    my_hash.digest_str(txt);
    my_hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[2B, C2, AC, 74, 3D, 46, 91, 1C, 89, 40, F7, 54, FD, 25, 4F, 19, CC, 9B, 18, 61]");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut my_hash = mySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_HashValue_in_string());
    assert_eq!(my_hash.get_HashValue_in_string(), "EA946E24D16483679986EEEA53271E2533AE1292");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut my_hash = mySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[D55C8BDE, 1B7102CD, C9827513, 7DCD2E46, E3DE8B12]");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut my_hash = mySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_vec()), "[19D14753, A0912927, 0370172A, EDA62A89, A4764D5C]");
    println!("-------------------------------");
}

fn SHA1_put_HashValue_in_array()
{
    println!("SHA1_put_HashValue_in_array");
    use Cryptocol::hash::SHA1;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA1::new();
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[43A03EAD, E239C73E, 239E1235, 55033CEE, 5603FDF8]");

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA1::new();
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[F1B38744, D331E027, C10A5B1E, 2E8869B9, 42BEF118]");
    println!("-------------------------------");
}

fn SHA1_tangle()
{
    println!("SHA1_tangle");
    use Cryptocol::hash::SHA1;
    let txt = "TANGLING";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[5B296514, 79D48A17, 1ADABF55, 09CC69B9, 83477776]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[6D00CD91, 2A9BAD37, 210A8909, B6A83E2F, 5D986325]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[E41C001F, 476FDC14, 1166767C, 3C09AE4D, 447B9B2F]");

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[570C0960, 44388BBA, 0DD84AC9, 2F78A2F8, E514D1FD]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[AE8C42A9, 4CFC9130, FF606528, E4876633, 27FC359F]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[2E33CBCF, 800599AD, 98827D7A, 41AA8BCB, D2D011FD]");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = mySHA1::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "54F0234F7188202D98EDDC643F71D95BEDE77ED7");
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

    use Cryptocol::hash::SHA1_Expanded;
    type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = mySHA1::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "78083F4E573928D6C4E9F869036F8A4D4D549E9F");
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
    SHA2_256_ruminate();
    SHA2_256_ruminate_str();
    SHA2_256_ruminate_string();
    SHA2_256_ruminate_array();
    SHA2_256_ruminate_vec();
    SHA2_256_get_HashValue();
    SHA2_256_get_HashValue_in_string();
    SHA2_256_get_HashValue_in_array();
    SHA2_256_get_HashValue_in_vec();
    SHA2_256_put_HashValue_in_array();
    SHA2_256_tangle();
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = mySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "1111111122222222444444446666666688888888AAAAAAAACCCCCCCCEEEEEEEE");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method digest().";
    let mut hash = mySHA2::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F7301C3222B8AA48ABDC3917F24B2E6E408601AC123C26B733E3FBDA494ACF7D");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method digest_str().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "853979616624A859070DB313AAE6BFED07A58EFA37571E88276D215AE845645B");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut my_hash = mySHA2::new();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "662FE0AAEF2070BE79771F3693F0A1CCA8DF6E9E08A5685535D99C77C258F3AC");
   
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA2::new();
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "C5E18AD8B82014203663BD271D12D2BF5F51045D0E9A1BBDE7D7A7B10A125DA0");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA2::new();
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "C5E18AD8B82014203663BD271D12D2BF5F51045D0E9A1BBDE7D7A7B10A125DA0");
    println!("-------------------------------");
}

fn SHA2_256_ruminate()
{
    println!("SHA2_256_ruminate");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method ruminate().";
    let mut hash = SHA2_256::new();
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5EB550DEA1A606FE03338BBEAEB7200003472FDF02556C6E32273C0405EF1443");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method ruminate().";
    let mut hash = mySHA2::new();
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "78782D14800491F6E66CAA238D955FE11FC9E9750161D51B83429B58AEC3EE0B");
    println!("-------------------------------");
}

fn SHA2_256_ruminate_str()
{
    println!("SHA2_256_ruminate_str");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method ruminate_str().";
    let mut hash = SHA2_256::new();
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "CD24A2825FA0CBAB8D70467D82E92D8BF0CBE86C6B0DAFECB87C254376A9323D");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method ruminate_str().";
    let mut my_hash = mySHA2::new();
    my_hash.ruminate_str(3, txt);
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "BF0DAD806A1D09BA917C08C951CB97F8F51C67D0EE8DFDBFCCEE7E5D6DE288C5");
    println!("-------------------------------");
}

fn SHA2_256_ruminate_string()
{
    println!("SHA2_256_ruminate_string");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method ruminate_string().".to_string();
    let mut hash = SHA2_256::new();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B013EDF34D926ABF2BE04489593A214FFECCA1F5334E31D17BFFEA000E4E85FE");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method ruminate_string().".to_string();
    let mut my_hash = mySHA2::new();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "6C72B5A23ED16E4533FA3437AF27E9DE21ECA6D0C947CDCB1CC684FBABA3E720");
    println!("-------------------------------");
}

fn SHA2_256_ruminate_array()
{
    println!("SHA2_256_ruminate_array");
    use Cryptocol::hash::SHA2_256;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_256::new();
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "CD949334D0E7BBEEC7C0B3FDDED14A2FFFA85EA91DCFB171521C5C74CB989925");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA2::new();
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "5E2FEBBBED0F9EFC2AFA2C892B9AC09B6A38DA45C0D23633D3C9F58A0C547909");
    println!("-------------------------------");
}

fn SHA2_256_ruminate_vec()
{
    println!("SHA2_256_ruminate_vec");
    use Cryptocol::hash::SHA2_256;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut hash = SHA2_256::new();
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "C02DA0025F7228433CB12036E35F60242B2F6C82F55DA1497ACABD491D381EDF");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA2::new();
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "D5E63A6B8D2E4F1912AAD8C6C99C9B063CB001A4FB1AD6B3D08A8F0C1B4CF947");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 32];
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    my_hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[2F, 1A, 5E, 1D, A7, 76, 0D, D5, 30, AF, 10, 15, A5, A8, 87, 05, 5D, 3A, 9C, 84, E6, 51, DC, E6, C1, 04, 03, 28, E6, 53, 94, 04]");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_HashValue_in_string());
    assert_eq!(my_hash.get_HashValue_in_string(), "4434E7FB4469B5D006B0D7B106F08282F35B4539C7CD76F3CAED85B7B27D98DF");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:02X?}", my_hash.get_HashValue_in_array()), "[DD652F45, 981B1D37, FC7B9FBB, 4C2BC1D0, 62BCCF9B, 8397D61A, E1D409F3, 300AB879]");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", my_hash.get_HashValue_in_vec()), "[22B5BF72, 8714E4B6, 98FB6F66, E899E5DE, F8672FBE, 9625486, D20A2EE1, 98CAEA90]");
    println!("-------------------------------");
}

fn SHA2_256_put_HashValue_in_array()
{
    println!("SHA2_256_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_256::new();
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[DADAB694, 80913194, 04C1F0B4, ECC59519, BA780B5B]");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[EB6B7D3D, CE7DC017, EAA6AE08, EB576BA2, F7E3B4AB]");
    println!("-------------------------------");
}

fn SHA2_256_tangle()
{
    println!("SHA2_256_tangle");
    use Cryptocol::hash::SHA2_256;
    let txt = "TANGLING";
    let mut hash = SHA2_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[D62ABC88, A57B6A04, B82A9922, C0316859, 3D8DDE86, E0D8783C, 07E34E29, 3F65D373]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[7FC9659F, 17ACCDB7, 43AA0A92, 160137F1, A2A172A6, 1B42868B, 981CA8B2, 98929E8B]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[7D60FABC, 351D79A0, DF04ADC9, A03CE8FB, A7154541, 5DB0A405, CDEE8242, 7D509560]");

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[12FFD34D, 9A5B4843, 1D8DBA65, 3C578886, B85EB6B2, 291D1A45, FD72ECFC, AC8D8577]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[FF0C7268, DA3463BD, 6601EC3B, 5D48D7BF, 10C4460B, F11B209E, CBCB2BCE, 08DE13FC]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[FA6EE74C, F322A5D0, E4EFB28A, 6E30F7FB, 5723E91A, F7B0B0CB, 256610EC, 3E6A6A2B]");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = mySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "6DED905D80768EE8F19D76233902E6CA1417B23A89845C2DA9127FEDD7CCDB5C");
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

    use Cryptocol::hash::SHA2_256_Expanded;
    type mySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = mySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "EF27B2954B124469ACD614F1DE4E99B30C418194B614EE19361674F64F60189C");
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
    SHA2_224_put_HashValue_in_array();
    SHA2_224_tangle();
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = mySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method digest().";
    let mut my_hash = mySHA2::new();
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "98256F32A77281A8CBCBA9105080A73BB55F0B51CCCBCC4A273D744E");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "8A6102A3AB8A7154E78D0FEBE130BA04E508AF7933AC88ED75D34BCD");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method digest_string().".to_string();
    let mut my_hash = mySHA2::new();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "85229E915A413FA4F90F86A51628834A0D0490B054330E032D93430A");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA2::new();
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "24E317307775C27419D4A65A57A647775216FDEB2416D1B283EB2271");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    let mut my_hash = mySHA2::new();
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "24E317307775C27419D4A65A57A647775216FDEB2416D1B283EB2271");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method get_HashValue().";
    let mut hashValue = [0_u8; 28];
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    my_hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    assert_eq!(format!("{:02X?}", hashValue), "[32, 1B, 31, 91, 2A, 11, 93, 05, 85, A4, 72, 98, FE, 6A, D1, 09, E3, 4A, AF, CB, B3, FD, 31, A2, 50, BF, A9, 94]");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method get_HashValue_in_string().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_HashValue_in_string());
    assert_eq!(my_hash.get_HashValue_in_string(), "0AB9C9B780979819224101F0A8DAC6DDD4E1039963F15B6E46D9336C");
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
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[DEBDEDF6, 18DDA1DB, 918876D0, CCFB08F2, 6FC8CC91, 42F16E75, 8950C7C2]");

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method get_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[58F9FE15, 4896019F, A984B63B, 9B37877F, EA8BD4C5, 340F5063, D623E295]");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method get_HashValue_in_vec().";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_HashValue_in_vec());
    assert_eq!(format!("{:02X?}", my_hash.get_HashValue_in_vec()), "[1321E948, CEB5006, F1263FE7, 1E8D6965, 987BE0FC, E2375457, EEBD9AE3]");
    println!("-------------------------------");
}

fn SHA2_224_put_HashValue_in_array()
{
    println!("SHA2_224_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_224;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_224::new();
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[C740100B, 893F2AAE, E414FC25, B3C926FB, 375C8BD7]");

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[5D862523, 506DB895, 57F970E5, 45600976, D4118B86]");
    println!("-------------------------------");
}

fn SHA2_224_tangle()
{
    println!("SHA2_224_tangle");
    use Cryptocol::hash::SHA2_224;
    let txt = "TANGLING";
    let mut hash = SHA2_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[DFBDC998, 897BDD0A, F99B538F, 178A5EE5, 16C96398, 2D544CAF, DC631DE9]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[32CB00E5, 9A09585A, 9051D8FB, F8F6EB0D, FD467652, 46408C7F, F5DD61C8]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[7F5D4897, F323EC3E, D47C95D5, 9D77DF01, 9269E780, 3973310E, 142EB013]");

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[9196ACFD, 94E19450, C9B7D8D3, 220C86A4, 6AC1EE8F, C87D73B4, ECFEE637]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[4D71E0F6, 41D78805, 94358C2C, FAC2356D, AEB666BB, 80880239, B2D1304D]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[851BE861, 5E595131, 072DF35A, 973B5D59, 87DBDC1D, 68BF05A6, 48EAC080]");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = mySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "136C899347821BCC7529F3B42C0A9E3E997E156B1E5E081F57BBB15E");
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

    use Cryptocol::hash::SHA2_224_Expanded;
    type mySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = mySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "849F654BAFF41D3025DE982EC410F8EC6991FFD6E5BF4047F45082F6");
    println!("-------------------------------");
}

fn Hash_SHA2_512_main()
{
    SHA2_512_Quick_Start();
    SHA2_512_new();
    SHA2_512_digest_C();
    SHA2_512_digest();
    SHA2_512_digest_str();
    SHA2_512_digest_string();
    SHA2_512_digest_array();
    SHA2_512_digest_vec();
    SHA2_512_get_HashValue();
    SHA2_512_get_HashValue_in_string();
    SHA2_512_get_HashValue_in_array();
    SHA2_512_get_HashValue_in_vec();
    SHA2_512_put_HashValue_in_array();
    SHA2_512_tangle();
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

    use Cryptocol::hash::SHA2_512_Expanded;
    type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = mySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "11111111111111113333333333333333555555555555555577777777777777779999999999999999BBBBBBBBBBBBBBBBDDDDDDDDDDDDDDDDFFFFFFFFFFFFFFFF");
    println!("-------------------------------");
}

fn SHA2_512_digest_C()
{
    println!("SHA2_512_digest_C");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method digest_C().";
    let mut hash = SHA2_512::new();
    hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DFE8300CADA392421B20D53A709921B4A1937ADFE39C17E2F00235E25F8E0B0983CA2CA5D7EBAF977F132EC1969AFC360BA4A128E535A18BAA48A0D69D2A6603");

    use Cryptocol::hash::SHA2_512_Expanded;
    type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = mySHA2::new();
    my_hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "D5C8CFBA57A0158AE7687C68B9AFA773DBF22688583BA69B389D1776F788A772F413FFFA4B8DB0769DFBF87FBA35DE7ABD92AABDDDE794676114DBC859100C64");
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

    use Cryptocol::hash::SHA2_512_Expanded;
    type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = mySHA2::new();
    my_hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "54D3E18AD977B18F4E3254FBE759C6D8072EFA95A88C671610C16A19D05253A9B3762FE32D054BADBEB877735287A47C1CD7439CA3AE8BE12489D0E8C7F73945");
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

fn SHA2_512_put_HashValue_in_array()
{
    println!("SHA2_512_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_512;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_512::new();
    let mut hash_code = [0_u64; 8];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[A3CC3D33B153F0B9, 34F8FFEBD0AB169C, 44A86EA8AEAD72D8, FA23418B701D96F8, 93061E766F3A07EC, 34F77254BAEB1447, 65E2A891ACA2B972, 15ABD372597E0128]");

    use Cryptocol::hash::SHA2_512_Expanded;
    type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u64; 8];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[842F1672FE5A67DB, 7F1EDB14EFE76CCE, 43235DF2AC2AD679, BBE6DF968F224E0A, D472C47FF927A886, 1BDD155EC5B19274, FDC261D8E469BB4C, 592FD7DEBDE994A1]");
    println!("-------------------------------");
}

fn SHA2_512_tangle()
{
    println!("SHA2_512_tangle");
    use Cryptocol::hash::SHA2_512;
    let txt = "TANGLING";
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[070B6A9457F65DD9, A7D2C2326CE14E8A, E870D6939FE02E39, 5CFEEDCA96BF3BA3, 013FFB332B3F51F3, B1D4E16355DBE0A9, E998240787066535, 1D5F597F04F84820]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[4780AEEAD19D5962, C55EAFBA7590FB70, CA6587899B2B276F, 55361EC5C9568667, FFD38C58FF62C288, 5E96A9FFC6B17704, 6D3885C75FE9B667, BFDA80D1514F38E5]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[D7FFE2BEEB81D532, EA420969761C4DAA, 8EE930740ABBBE3E, 0DC90C0705AE5F38, E91531243615F994, 174C4F96168FBFC4, 06373FFDD9C66A16, 910560A5898E3728]");

    use Cryptocol::hash::SHA2_512_Expanded;
    type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[D0EDE5AFDDAB96B5, 78B6CC968AFB83EB, CE2369C35DA4A43F, 4B753CF1D02A1A3F, 29A3861EBD42210C, 952536C0957B0B60, 675FE725336E105E, 6E2ACB9D03A95AD2]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[5C72DB128F57491F, B70402F02D41A779, 1B9B1C9979BD59AF, 90ABF522230D4DB3, 2330B855BB6C253C, 297D4E6FF6B37F70, 929F3A8F3CB9A7FD, 3EDD2459251BB838]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[A2090D429E425CEC, D6FD81EEE61ED3B5, 34D1E87A7B4B06E3, 7415804887A7528D, 89EF9F2F4F6CC538, EED8FE585C02AF99, C20EB506C486C145, 730E9AA7A3B591E6]");
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
    SHA2_384_put_HashValue_in_array();
    SHA2_384_tangle();
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

fn SHA2_384_put_HashValue_in_array()
{
    println!("SHA2_384_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_384;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_384::new();
    let mut hash_code = [0_u64; 6];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[2777A97F91255475, F127D5AC237903EE, 62711EF410BDD62D, 4F880A8577004A7A, 3C982DC71C4E3C21, 55B8486AB8A12ABB]");

    use Cryptocol::hash::SHA2_384_Expanded;
    type mySHA2 = SHA2_384_Expanded<160>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u64; 6];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[40212E268A1D743C, D5FC2CC6747702FD, 571FEBB10FB1B290, 9F7E090DF0195B12, 65DF23A919E58C1B, CE317215CBDF8EC1]");
    println!("-------------------------------");
}

fn SHA2_384_tangle()
{
    println!("SHA2_384_tangle");
    use Cryptocol::hash::SHA2_384;
    let txt = "TANGLING";
    let mut hash = SHA2_384::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[A52945B3E9E6E2E0, 7208374E02CB1DFE, 9481D881D89B7946, C425DF584817FD25, 49001993DD7EB02E, A5BF4D24B77D621E]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[84BE10E10BEB5A66, AF72D1F8D4A763E7, 1B2DFA37B163EDC6, CEABC9EDAC24CB65, 7845447250E564EC, A4FAF9EAEECB878B]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[707481D3670B0FA8, B89726EA56C4170A, DF8C93E221E240BD, AA0DEAEA3D1C891D, 4B8DF37A322EF5FA, E88A2A9E835BAC4D]");

    use Cryptocol::hash::SHA2_384_Expanded;
    type mySHA2 = SHA2_384_Expanded<160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[8A515773672A0C7A, 8CA30FEB93D3A13D, CB81222CFD104F01, DEAA36FB688514FE, 01377A73FCD823E5, 1E44AB0506043A7F]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[D6DD49E21832A216, 3676FE0F8EEB0A8D, 4029F8BD7C7C64CC, D47CA3DAE698F1CE, 6BA349E4F33F2853, E1A939130FE9CD81]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[CD00FDD3A9E9F113, AF71F8BC3F147BBC, CF679991FC2D4957, 2DA56392E6B94D9F, 749AD435F6772132, 50CD667F09190781]");
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
    SHA2_512_256_put_HashValue_in_array();
    SHA2_512_256_tangle();
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

fn SHA2_512_256_put_HashValue_in_array()
{
    println!("SHA2_512_256_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_512_256::new();
    let mut hash_code = [0_u64; 4];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[218BF516E454F8E8, 3275DBD07098A67B, B1D289D9DCC2A854, 4D1672BEFB75B043]");

    use Cryptocol::hash::SHA2_512_256_Expanded;
    type mySHA2 = SHA2_512_256_Expanded<160>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u64; 4];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[DF7A58B7D93D2EA8, 5199F8078F5F6813, D6390AFF49FE37FE, 59E9B161E91C2EB8]");
    println!("-------------------------------");
}

fn SHA2_512_256_tangle()
{
    println!("SHA2_512_256_tangle");
    use Cryptocol::hash::SHA2_512_256;
    let txt = "TANGLING";
    let mut hash = SHA2_512_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[FC36648637962C38, BDFBBAE5DEA75E0E, D72827D56EB79EF9, 4969BAA99DB0E42B]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[96CA6859E014C355, 6BBED0E8DA26FFAD, A4F89477C93C9E8C, 806148BDB037AE26]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[11F5369ABC9E3B5D, D3D869131E697AB2, 1899C8D791BB09FC, 0C6CE82AE3B9D583]");

    use Cryptocol::hash::SHA2_512_256_Expanded;
    type mySHA2 = SHA2_512_256_Expanded<160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[9771ACFA1FFE9B55, BF7CF746370F01E7, D68B291C1C3EEB8C, 5E8D5A2DBC792186]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[B4C1735DDC8677A6, 6AF607FE0979BF92, BFD34066C9E1317F, B51988A069D20E75]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[F12B54C8E3F7F9AB, 3EAD06A674A59791, CF3237564DCBF985, EA8A45DFBFD4B2C9]");
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
    SHA2_512_t_256_new_with_seedText();
    SHA2_512_t_256_digest_C();
    SHA2_512_t_256_digest();
    SHA2_512_t_256_digest_str();
    SHA2_512_t_256_digest_string();
    SHA2_512_t_256_digest_array();
    SHA2_512_t_256_digest_vec();
    SHA2_512_t_256_get_HashValue();
    SHA2_512_t_256_get_HashValue_in_string();
    SHA2_512_t_256_get_HashValue_in_array();
    SHA2_512_t_256_get_HashValue_in_vec();
    SHA2_512_t_256_put_HashValue_in_array();
    SHA2_512_t_256_tangle();
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

fn SHA2_512_t_256_new_with_seedText()
{
    println!("SHA2_512_t_256_new_with_seedText");
    use Cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new_with_seedText("샤-");
    // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6E231779CE7B233F74077E896D4ABCCA8B31054CB94168164E08BD8F31764DCB");
    println!("-------------------------------");
}

fn SHA2_512_t_256_digest_C()
{
    println!("SHA2_512_t_256_digest_C");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method digest_C().";
    let mut hash = SHA2_512_t_256::new();
    hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "91589EAF3AAC1137D8672E02248AFC73B0A831EF6C36A88269566B6A6C0461F0");

    use Cryptocol::hash::SHA2_512_t_256_Expanded;
    type mySHA2 = SHA2_512_t_256_Expanded<0x123456789abcdef0, 160>;
    let mut my_hash = mySHA2::new();
    my_hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FFD5110E22D3BE4CBAA837F95189F369AB199285C1763EDBCE28C4E49FD210F6");
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
    hash.put_HashValue_in_array(&mut h);
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

fn SHA2_512_t_256_put_HashValue_in_array()
{
    println!("SHA2_512_t_256_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_512_t_256::new();
    let mut hash_code = [0_u64; 4];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[218BF516E454F8E8, 3275DBD07098A67B, B1D289D9DCC2A854, 4D1672BEFB75B043]");

    use Cryptocol::hash::SHA2_512_t_256_Expanded;
    type mySHA2 = SHA2_512_t_256_Expanded<160>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u64; 4];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[BC3F9E2D39A7884A, 145476850E31A6A4, 5E9E4735097FA868, 9C54C80B9D591AE8]");
    println!("-------------------------------");
}

fn SHA2_512_t_256_tangle()
{
    println!("SHA2_512_t_256_tangle");
    use Cryptocol::hash::SHA2_512_t_256;
    let txt = "TANGLING";
    let mut hash = SHA2_512_t_256::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array_TM::<u64, 4>());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array_TM::<u64, 4>()), "[FC36648637962C38, BDFBBAE5DEA75E0E, D72827D56EB79EF9, 4969BAA99DB0E42B]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array_TM::<u64, 4>());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array_TM::<u64, 4>()), "[96CA6859E014C355, 6BBED0E8DA26FFAD, A4F89477C93C9E8C, 806148BDB037AE26]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_HashValue_in_array_TM::<u64, 4>());
    assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array_TM::<u64, 4>()), "[11F5369ABC9E3B5D, D3D869131E697AB2, 1899C8D791BB09FC, 0C6CE82AE3B9D583]");

    use Cryptocol::hash::SHA2_512_t_256_Expanded;
    type mySHA2 = SHA2_512_t_256_Expanded<160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array_TM::<u64, 4>());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array_TM::<u64, 4>()), "[C60A42A16859F8B8, 7EAB94538B024642, 654DD7795DDDD39B, 12E1A03748AEFFF3]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array_TM::<u64, 4>());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array_TM::<u64, 4>()), "[05A82162DE47FEE5, 4B7C2320AF525665, 0D9A9FC79B16B8E6, B51D2D5242BADECD]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array_TM::<u64, 4>());
    assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array_TM::<u64, 4>()), "[BC74B5902DD2AB00, 680C9FE85FED5E60, 4FAAF51214292837, B9292AFDBF94B64E]");
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
    // SHA2_512_t_224_new_with_seedText();
    // SHA2_512_t_224_new_with_H();
    // SHA2_512_t_224_digest_C();
    SHA2_512_t_224_digest();
    SHA2_512_t_224_digest_str();
    SHA2_512_t_224_digest_string();
    SHA2_512_t_224_digest_array();
    SHA2_512_t_224_digest_vec();
    SHA2_512_t_224_get_HashValue();
    SHA2_512_t_224_get_HashValue_in_string();
    SHA2_512_t_224_get_HashValue_in_array();
    SHA2_512_t_224_get_HashValue_in_vec();
    SHA2_512_t_224_put_HashValue_in_array();
    SHA2_512_t_224_tangle();
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
    hash.put_HashValue_in_array(&mut h);
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

fn SHA2_512_t_224_put_HashValue_in_array()
{
    println!("SHA2_512_t_224_put_HashValue_in_array");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut hash = SHA2_512_t_224::new();
    let mut hash_code = [0_u64; 4];
    hash.digest_str(txt);
    hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[6CF723619865F699, 7DC49BFEAED1C117, 2DF9E01CAEF542F5, FA1BA67BA06F5FF5]");

    use Cryptocol::hash::SHA2_512_t_224_Expanded;
    type mySHA2 = SHA2_512_t_224_Expanded<160>;
    let txt = "This is an example of the method put_HashValue_in_array().";
    let mut my_hash = mySHA2::new();
    let mut hash_code = [0_u64; 4];
    my_hash.digest_str(txt);
    my_hash.put_HashValue_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[08143B9294ECCD69, B89B933148B16625, B9DE1F9C0E182568, C95A5B18FDD97FF5]");
    println!("-------------------------------");
}

fn SHA2_512_t_224_tangle()
{
    println!("SHA2_512_t_224_tangle");
    use Cryptocol::hash::SHA2_512_t_224;
    let txt = "TANGLING";
    let mut hash = SHA2_512_t_224::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array_TM::<u32, 7>());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array_TM::<u32, 7>()), "[72E2E82F, C78389DA, 112F494F, B415B8C4, EF993BFA, EDB5091B, 8C03F067]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array_TM::<u32, 7>());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array_TM::<u32, 7>()), "[A7CED549, 2C050740, 9BC2F6E5, EAC6D908, 26148AE9, 966D5E72, ED5DF840]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_HashValue_in_array_TM::<u32, 7>());
    assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array_TM::<u32, 7>()), "[14C24EAE, B39CD243, 8C484722, CB1A03AA, F1F9F55E, 955A27D8, 70A3ED4F]");

    use Cryptocol::hash::SHA2_512_t_224_Expanded;
    type mySHA2 = SHA2_512_t_224_Expanded<160>;
    let txt = "TANGLING";
    let mut my_hash = mySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array_TM::<u32, 7>());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array_TM::<u32, 7>()), "[6EF90662, CD08A7EA, 93D0EDFC, 390175A6, 53368038, ADC8BCC8, 11351AB8]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array_TM::<u32, 7>());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array_TM::<u32, 7>()), "[F7566CAF, B1039FF1, 722C9B99, 5AA84D67, E6C1182A, 3B4D2DBF, 7F1FA1C8]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array_TM::<u32, 7>());
    assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array_TM::<u32, 7>()), "[5B74C46E, F433ACC6, 6A402398, 39126678, 581E67AD, 14A4C823, 4B387049]");
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

/*
fn Hash_OS_Rng_main()
{
    use std::ops::*;
    use std::fmt::{ Display, Debug };
    use rand::{ rngs, RngCore };

    use Cryptocol::number::SmallUInt;
    use Cryptocol::random::{ PRNG, Random_Generic };

    pub struct OsRng;
    // {
    //     hash_code: [u64; 8]
    // }

    impl PRNG for OsRng
    {
        #[inline]
        fn new() -> Self    { Self }

        #[inline]
        fn new_with<T, const N: usize>(_: &[T; N]) -> Self
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
        { Self::new() }

        #[inline]
        fn sow_array<T, const N: usize>(&mut self, _: &[T; N])
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
        {}

        #[inline]
        fn harvest(&mut self, _: u64) -> [u64; 8]
        {
            [rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
            rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
            rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
            rngs::OsRng.next_u64(), rngs::OsRng.next_u64()]
        }
    }

    pub type Random_OsRng = Random_Generic<OsRng>;
    
    let mut r = Random_OsRng::new();
    println!("Random_OsRng u8 = {}", r.random_u8());
    println!("Random_OsRng u16 = {}", r.random_u16());
    println!("Random_OsRng u32 = {}", r.random_u32());
    println!("Random_OsRng u64 = {}", r.random_u64());
    println!("Random_OsRng u128 = {}", r.random_u128());
    println!("Random_OsRng under 123456789 = {}", r.random_under_uint_(123456789_u64));
    println!("Random_OsRng prime number = {}", r.random_prime_using_Miller_Rabin_uint::<u128>(5));
    println!("Random_OsRng under BigUInt = {}", r.random_BigUInt::<u64, 8>());
    println!("Random_OsRng odd BigUInt = {}", r.random_odd_BigUInt::<u64, 8>());
    println!("Random_OsRng BigUInt prime number = {}", r.random_prime_using_Miller_Rabin_BigUInt::<u64, 8>(5));
}
*/

