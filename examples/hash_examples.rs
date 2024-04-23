// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]


pub fn main()
{
    hash_md4_main();
    hash_md5_main();
    hash_sha1_main();
    hash_sha2_256_main();
    hash_sha2_224_main();
    hash_sha2_512_main();
    hash_sha2_384_main();
    hash_sha2_512_256_main();
    hash_sha2_512_t_256_main();
    hash_sha2_512_t_224_main();
}

fn hash_md4_main()
{
    md4_quick_start1();
    md4_quick_start2();
    md4_new();
    md4_digest();
    md4_digest_str();
    md4_digest_string();
    md4_digest_array();
    md4_digest_vec();
    md4_ruminate();
    md4_ruminate_str();
    md4_ruminate_string();
    md4_ruminate_array();
    md4_ruminate_vec();
    md4_get_hash_value();
    md4_get_hash_value_in_string();
    md4_get_hash_value_in_array();
    md4_get_hash_value_in_vec();
    md4_put_hash_value_in_array();
    md4_tangle();
    md4_fmt_for_to_string();
    md4_fmt_for_println();
}

fn md4_quick_start1()
{
    println!("md4_quick_start1");
    use std::string::*;
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "31D6CFE0D16AE931B73C59D7E0C089C0");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "D5EF20EEB3F75679F86CF57F93ED0FFE");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "6407C0E728DA762A04924ADFE630974C");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4F4A24D124B996BEA395344419F9A06B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "9DE35D8FCF68E74867FFB63F28625ABE");

    txt = "I am testing MD4 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "3A9F1487472B3A4315E0C90DC5CB3A2E");

    txt = "I am testing MD4 for the message which is sixty-four bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "6CDB5B2BFF823A4A7B23675180EB7BEF");

    txt = "I am testing MD4 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "56771653687981390B0EB2A7D0A40DBB");
    println!("-------------------------------");
}

fn md4_quick_start2()
{
    println!("md4_quick_start2");
    use std::string::*;
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut hash = MyMD4::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "13892AE087B903E5EC030A51E1BC720A");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "0C0BE1B8893E47C005D95C69234141E9");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "17545CEB681C5B848234A557C5957AA7");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "70F3EA1DCDE46C65868DC0937E374433");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "640B4635ED76F6574FC30AB233B74712");
    
    txt = "I am testing MD4_Expanded for the data of the length 62 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B0D18D969B99F4BF48365449AF82EAFB");
    
    txt = "I am testing MD4_Expanded for the message which is 64 byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "2D4ADABC3504B4A1B98FCCBFC48145AE");

    txt = "I am testing MD4_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "26E5336E4D863BBAD6347918CE6DBAF5");
    println!("-------------------------------");
}

fn md4_new()
{
    println!("md4_new");
    // Example for MD4
    use cryptocol::hash::MD4;
    let hash = MD4::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let my_hash = MyMD4::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    println!("-------------------------------");
}

fn md4_digest()
{
    println!("md4_digest");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A18836F660C3C66B8CBEE4BD24FEFFA9");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "B2F465006DCBA147BCE76D7EB8B564E1");
    println!("-------------------------------");
}

fn md4_digest_str()
{
    println!("md4_digest_str");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "719A1EB0F5077837BB408434B7AAD81E");
    println!("-------------------------------");
}

fn md4_digest_string()
{
    println!("md4_digest_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DF23C7808B2B158C5E2D8C9FE1FF2ECC");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FD42F7479ED133619D877BB1E6C8A084");
    println!("-------------------------------");
}

fn md4_digest_array()
{
    println!("md4_digest_array");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    println!("-------------------------------");
}

fn md4_digest_vec()
{
    println!("md4_digest_vec");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    println!("-------------------------------");
}

fn md4_ruminate()
{
    println!("md4_ruminate");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "23EAC3CEE64E4266EEDFE2D6AB255B9F");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut hash = MyMD4::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A1608F7E4052E267B3233862FD5C1C41");
    println!("-------------------------------");
}

fn md4_ruminate_str()
{
    println!("md4_ruminate_str");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B19769E514631D59FD257C4AD667BD9D");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "534F1EC44D4B2CEF12B7A9A81941D9A8");
    println!("-------------------------------");
}

fn md4_ruminate_string()
{
    println!("md4_ruminate_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "71D3AB5636348DB24A7AE302E7E6C05A");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "EFB3B63FC1DBF3852F469D4EA0E8D517");
    println!("-------------------------------");
}

fn md4_ruminate_array()
{
    println!("md4_ruminate_array");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "810F75A7BD28179BA2D4604A3092FBC8");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "27F598D17E6DFBA0A0713F3262D34FFC");
    println!("-------------------------------");
}

fn md4_ruminate_vec()
{
    println!("md4_ruminate_vec");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "B3E296760B88B44613DB03D72CE59917");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "AFC96A14952E9FB9D6D7C7A1FD3D4C2E");
    println!("-------------------------------");
}

fn md4_get_hash_value()
{
    println!("md4_get_hash_value");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[A7, AD, DF, 36, A2, 43, 97, D1, 6D, 3C, 99, 78, A6, D5, 6E, 74]");
 
    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[02, 43, 79, C6, 08, F1, CA, 30, C0, 75, 5C, 6C, 07, AD, 76, 72]");
    println!("-------------------------------");
}

fn md4_get_hash_value_in_string()
{
    println!("md4_get_hash_value_in_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "FA48527AD8257A371E70AA9473D425D6");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "626192ACD80D62D8966ACE89AE439E76");
    println!("-------------------------------");
}

fn md4_get_hash_value_in_array()
{
    println!("md4_get_hash_value_in_array");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[832C724B, 4A73A717, 5EA679B8, E991D13B]");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[2CFB0798, 77AA2A27, 602B457E, AD3B964C]");
    println!("-------------------------------");
}

fn md4_get_hash_value_in_vec()
{
    println!("md4_get_hash_value_in_vec");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[EE74475E, ECA09C8F, 038A89A3, 9B2A6C4F]");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[440664DA, 49687C74, C0536C83, 192830D8]");
    println!("-------------------------------");
}

fn md4_put_hash_value_in_array()
{
    println!("md4_put_hash_value_in_array");
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[147DD795, C34F9C9D, 80B94C86, FB922262]");

    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[1411D15D, 37BBE0DF, 1EAF8DA5, AC822C42]");
    println!("-------------------------------");
}

fn md4_tangle()
{
    println!("md4_tangle");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[BC65D6E1, F0F37B4E, 2F404331, A8F25E2A]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[CE1E07A3, F3373D70, 95A8F098, 9BC7894E]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[5B9A2D14, 64888002, 15282E23, E5B2F4BD]");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[C4377D49, 05FD9A1F, 3DA4E254, ACF22116]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[8CB0AF83, F75E073C, 77C5BF6C, EDFE1D51]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[A5C900D1, 388193FA, B2C0ED53, 4DE71DDE]");
    println!("-------------------------------");
}

fn md4_fmt_for_to_string()
{
    println!("md4_fmt_for_to_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "E2244B71E17D5BD7E1CCEB58C8F8C82E");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "6B0D8F0CE90782E5FF88EE57B5DC5AF1");
    println!("-------------------------------");
}

fn md4_fmt_for_println()
{
    println!("md4_fmt_for_println");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3C607B5548C155DCF4E84C7A6C21D349");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "745B42127EC2479032923F2EE368FD92");
    println!("-------------------------------");
}


fn hash_md5_main()
{
    md5_quick_start1();
    md5_quick_start2();
    md5_new();
    md5_digest();
    md5_digest_str();
    md5_digest_string();
    md5_digest_array();
    md5_digest_vec();
    md5_ruminate();
    md5_ruminate_str();
    md5_ruminate_string();
    md5_ruminate_array();
    md5_ruminate_vec();
    md5_get_hash_value();
    md5_get_hash_value_in_string();
    md5_get_hash_value_in_array();
    md5_get_hash_value_in_vec();
    md5_put_hash_value_in_array();
    md5_tangle();
    md5_fmt_for_to_string();
    md5_fmt_for_println();
}

fn md5_quick_start1()
{
    println!("md5_quick_start1");
    use std::string::*;
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "D41D8CD98F00B204E9800998ECF8427E");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "7FC56270E7A70FA81A5935B72EACBE29");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "49DC5E45FBEC1433E2C612E5AA809C10");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "17ED1DB5CD96184041659D84BB36D76B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C3EB6D4A1071E1A9C5E08FEF6E8F3FBF");

    txt = "I am testing MD5 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "6C33614E6317DC4641573E0EBC287F98");

    let mut txt = "I am testing MD5 for the message which is sixty-four bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "584D41C6837AC714275196E4FF14B2EF");

    txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9831162AB272AE1D85245B75726D215E");
    println!("-------------------------------");
}

fn md5_quick_start2()
{
    println!("md5_quick_start2");
    use std::string::*;
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut hash = MyMD5::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "2793C0925118EEA53C288640AA7D9C81");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "78DBB9B3C63B704745BFF37E3254B350");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "298B65824C2F415446A6210AB0191B8B");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "80C1F5A7858DEB5A136CE57DC60FCFBE");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "F3B540538E750E62A3F9417C308DD018");

    txt = "I am testing MD5_Expanded for the data of the length 62 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "A136326C1DA54B63CC2743647219BA60");

    txt = "I am testing MD5_Expanded for the message which is 64 byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C6AC6907DDE0E8429CF44BC21941F64A");

    txt = "I am testing MD5_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BD4E0962262EBBAE4C4A89FBB2F7A2D4");
    println!("-------------------------------");
}

fn md5_new()
{
    println!("md5_new");
    // Example for MD5
    use cryptocol::hash::MD5;
    let hash = MD5::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let my_hash = MyMD5::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    println!("-------------------------------");
}

fn md5_digest()
{
    println!("md5_digest");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "336EA91DD3216BD0FC841E86F9E722D8");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "51F4248FBEFBE0A00196F9F04DD07FF0");
    println!("-------------------------------");
}

fn md5_digest_str()
{
    println!("md5_digest_str");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F2E455CEB5FB993A980E67D3FA8A3961");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "21EE03C8185BD65CDB8116D0E2714F09");
    println!("-------------------------------");
}

fn md5_digest_string()
{
    println!("md5_digest_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "40929E789D2F5880B85456E289F704C0");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "02BDBC510B949045A131C0C3302027BA");
    println!("-------------------------------");
}

fn md5_digest_array()
{
    println!("md5_digest_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
    println!("-------------------------------");
}

fn md5_digest_vec()
{
    println!("md5_digest_vec");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
    println!("-------------------------------");
}

fn md5_ruminate()
{
    println!("md5_ruminate");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "71F09FB7840FA1EB78A88ED071627C0D");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut hash = MyMD5::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "CC179809A9DC1475EEF5E4810C272882");
    println!("-------------------------------");
}

fn md5_ruminate_str()
{
    println!("md5_ruminate_str");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "68B3B09AE0EED0D15E744671E29824D4");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "7A460BDA766C6A7D4F9A23DCBDB71A4C");
    println!("-------------------------------");
}

fn md5_ruminate_string()
{
    println!("md5_ruminate_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E06B1A664322C1296D1FCD3F28428493");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "5018AF48C7606F748073FC5255448BAB");
    println!("-------------------------------");
}

fn md5_ruminate_array()
{
    println!("md5_ruminate_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4914D161AE665750248DF91B6E57C7BE");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "1FBF755293909670FE66B8CA482BCF66");
    println!("-------------------------------");
}

fn md5_ruminate_vec()
{
    println!("md5_ruminate_vec");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BDEE5A3C5B2DB7B6F18B170C2E865FE0");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "DBFD74659889B373D90477B59A193CBD");
    println!("-------------------------------");
}

fn md5_get_hash_value()
{
    println!("md5_get_hash_value");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[91, 57, 43, 58, C7, F9, 04, 83, 60, 63, 15, CD, 1B, 77, 2E, DD]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[A4, 5C, 46, 58, 29, BB, 83, 06, 32, 4D, 20, 20, 23, 9D, 41, AE]");
    println!("-------------------------------");
}

fn md5_get_hash_value_in_string()
{
    println!("md5_get_hash_value_in_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "5E9D7F0006214CB49D09FC846FBE2927");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "A8BA6619878AE3A8135B7FD2A6ECAE6D");
    println!("-------------------------------");
}

fn md5_get_hash_value_in_array()
{
    println!("md5_get_hash_value_in_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[1FC84032, 1DFA906E, 911B468C, 66EDE0CE]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[06813E1E, DA7BA0BF, 4B48D110, 6B111859]");
    println!("-------------------------------");
}

fn md5_get_hash_value_in_vec()
{
    println!("md5_get_hash_value_in_vec");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[D9A44F09, 27F51F07, 4517E390, 4CF17D73]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[5D9AB684, 090F7AEB, 31FD214E, F03D3032]");
    println!("-------------------------------");
}

fn md5_put_hash_value_in_array()
{
    println!("md5_put_hash_value_in_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[512E75DE, 4525528D, 41E8D192, 5606EE3B]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[F8634B80, 96E02659, E26EA89D, EDA8E0C4]");
    println!("-------------------------------");
}

fn md5_tangle()
{
    println!("md5_tangle");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E60545F6, 6DCF2B02, 8245048B, AE2A98C6]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E0B5F1C0, 5C62629F, 68D44BC1, D384AB34]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[C75EEA9C, 9D5CF62B, 0ABFA634, CD29C2D4]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[9CCE671A, 5366F625, 68056532, D6B0DA5C]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[A12380BC, DE74206D, C145732C, 4CAAD502]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[D9EB87F4, 00C2D299, A492A483, 1C24FCDD]");
    println!("-------------------------------");
}

fn md5_fmt_for_to_string()
{
    println!("md5_fmt_for_to_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "ED085603C2CDE77DD0C6FED3EC1A8ADB");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "3FDFF3827C89F3C770A0863F069FE766");
}

fn md5_fmt_for_println()
{
    println!("md5_fmt_for_println");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "6C9494A4A5C313001695262D72571F74");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "45BA0E4FEA1FACF829D19544A77105B8");
    println!("-------------------------------");
}

fn hash_sha1_main()
{
    sha1_quick_start1();
    sha1_quick_start2();
    sha1_quick_start3();
    sha1_quick_start4();
    sha1_new();
    sha1_digest();
    sha1_digest_str();
    sha1_digest_string();
    sha1_digest_array();
    sha1_digest_vec();
    sha1_ruminate();
    sha1_ruminate_str();
    sha1_ruminate_string();
    sha1_ruminate_array();
    sha1_ruminate_vec();
    sha1_get_hash_value();
    sha1_get_hash_value_in_string();
    sha1_get_hash_value_in_array();
    sha1_get_hash_value_in_vec();
    sha1_put_hash_value_in_array();
    sha1_tangle();
    sha1_fmt_for_to_string();
    sha1_fmt_for_println();
}

fn sha1_quick_start1()
{
    println!("sha1_quick_start1");
    use std::string::*;
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "6DCD4CE23D88E2EE9568BA546C007C63D9131C1B");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "0BBCDBD1616A1D2230100F629649DCF5B7A28B7F");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B82A61505779F6B3ACA4F5E0D54DA44C17375B49");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C6DC54281357FC16D357E1D730BFC313C585DAEC");

    txt = "I am testing SHA1 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "36CD36337097D764797091E5796B6FF45A9FA79F");

    txt = "I am testing SHA-1 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "E408F6B82DCDDB5EE6613A759AC1B13D0FA1CEF1");

    txt = "I am testing SHA1 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BB2C79F551B95963ECE49D40F8A92349BF66CAE7");
    println!("-------------------------------");
}

fn sha1_quick_start2()
{
    println!("sha1_quick_start2");
    use std::string::*;
    use cryptocol::hash::SHA0;
    let mut hash = SHA0::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "F96CEA198AD1DD5617AC084A3D92C6107708C0EF");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "E4DA6A8FBD813C90E6FA040D5F15398ECA200339");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "72CFDDBCDCCCC0847DA8AA7FDBA901A2FC431068");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B56263EB76AE1ABA8E7E4A4CA104BC78F1BC8D7A");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "613FEB0029DF4FE0D16CBA8AAFA596D9BC309D18");

    txt = "I am testing SHA0 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "E0351ED0E4FDD2F5731A2E7472B08038B10AFB0D");

    txt = "I am testing SHA-0 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "270CCCFD32361F7C01427D9F64B2248C6C88D080");

    txt = "I am testing SHA0 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "0E71D76AC85D342DB566EDCFC971B6E06C5D7CBC");
    println!("-------------------------------");
}

fn sha1_quick_start3()
{
    println!("sha1_quick_start3");
    use std::string::*;
    use cryptocol::hash::SHA1_Expanded;

    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut hash = MySHA1::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "6C64DE0C4E997B2EE6DD562DBC43D2A1CB53F186");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "DA0918B8FDF524572C293C4971CCE4071E14CE30");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C6FC72A4D3A72DC40FF8B601DA4F1A626210EBC4");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "58CEBE40E0391A38724EA06F327946C70C5585B2");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "756049AC909ACA9BC1A9213E0148402156B0DC7F");
    
    txt = "I am testing SHA1_Expanded for the data of sixty-two byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "18AD939F2027D8FEF249E533ECEC4BC551558317");
    
    txt = "I am testing SHA1_Expanded for the data whose length is 64 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "89DE08E440FEDA32C10C704B0741E7EBBA8F74F4");

    txt = "I am testing SHA1_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E131AE2668538116730817D639801D639AD9B883");
    println!("-------------------------------");
}

fn sha1_quick_start4()
{
    println!("sha1_quick_start4");
    use std::string::*;
    use cryptocol::hash::SHA0_Expanded;

    type MySHA0 = SHA0_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut hash = MySHA0::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "DC1EF9093D309849ABA5EDF152DB32C695421438");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "244743C28AE0223A6A0661813221864CC2EDBA35");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "2BBE776DC9E577B444CDA082EE31A87DB3C4EF57");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "170E3BCFC4DDE939C7BF62C2F28A3DED07041407");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "E3EDDB4717F72DC7E703139D60964860A6AB316B");
    
    txt = "I am testing SHA0_Expanded for the data of sixty-two byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "ADF444B96A666E409E10452F634E0830B704C4EA");
    
    txt = "I am testing SHA0_Expanded for the data whose length is 64 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "F0148F75FEF02934F69B18870FEC85DFE215B2AF");

    txt = "I am testing SHA0_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C9A0A2597CAB6E3A6C0009FA6552F46814DBCB54");
    println!("-------------------------------");
}

fn sha1_new()
{
    println!("sha1_new");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let hash = SHA1::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "67452301EFCDAB8998BADCFE10325476C3D2E1F0");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let my_hash = MySHA1::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888CCCCCCCCFFFFFFFF");
    println!("-------------------------------");
}

fn sha1_digest()
{
    println!("sha1_digest");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9631162DFDAEAB89821256D4585D66D35CD61FD6");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FAF77A15CDEDFC6A33CE2C4003B119F225CBE414");
    println!("-------------------------------");
}

fn sha1_digest_str()
{
    println!("sha1_digest_str");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9FDE56BBB5028966CC2E7BDCD0758FE3121407E6");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "A6BE8FEA7E3F61508DC0A8BA85A0AEC77D0C0784");
    println!("-------------------------------");
}

fn sha1_digest_string()
{
    println!("sha1_digest_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "FDCDC0EBC9181B881BE1F15FECEBB9D70E4DDAAB");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "F4FE5C5A4D2A4BD414DDDF1FD32B185F3ED8AA32");
    println!("-------------------------------");
}

fn sha1_digest_array()
{
    println!("sha1_digest_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
    println!("-------------------------------");
}

fn sha1_digest_vec()
{
    println!("sha1_digest_vec");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
    println!("-------------------------------");
}

fn sha1_ruminate()
{
    println!("sha1_ruminate");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "1E91427CF3BBB256A2BD44DA9F89D7406ED5D5FE");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut hash = MySHA1::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "509038D0447A5D05F4AD62C25AD6F9E130E694F4");
    println!("-------------------------------");
}

fn sha1_ruminate_str()
{
    println!("sha1_ruminate_str");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "778B3FF529024A46A3CC06F01CBE9078F6447BC0");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "0CFDD49B87B844C4C329C997C1FB650EBEEA4909");
    println!("-------------------------------");
}

fn sha1_ruminate_string()
{
    println!("sha1_ruminate_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F4CE0B5A8D93BEB1C0A99F6290B26661C212A8B3");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C4B55C59A15FCDFF6FFD39D3867665F67E89C8FC");
    println!("-------------------------------");
}

fn sha1_ruminate_array()
{
    println!("sha1_ruminate_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "35BC04C66EBA9751C482FD98BCD1CBDC2C5E56AF");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "898835EC92B5F7818A25C6645673DED30DA5F78D");
    println!("-------------------------------");
}

fn sha1_ruminate_vec()
{
    println!("sha1_ruminate_vec");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "042811212E91F341473A43BF71BD8DA035D23032");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "99B8709ACB93051C4CB238CE9CD9031BD40F2A2B");
    println!("-------------------------------");
}

fn sha1_get_hash_value()
{
    println!("sha1_get_hash_value");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 20];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[82, 62, 1B, E6, A6, 74, 88, 18, 12, 60, 5F, 27, C7, EF, 19, 38, 65, 39, 00, 8A]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 20];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[F5, DD, 99, 0C, 9B, 5A, 4C, A3, 84, DF, B1, 3D, 73, 5A, CE, CF, 19, BB, 52, B4]");
    println!("-------------------------------");
}

fn sha1_get_hash_value_in_string()
{
    println!("sha1_get_hash_value_in_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "826621B45597FA1B58C855DFCDE111E7500BCC96");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "72CEC05D49E2FA7206E2BF5A6C9D38F0404E7956");
    println!("-------------------------------");
}

fn sha1_get_hash_value_in_array()
{
    println!("sha1_get_hash_value_in_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:02X?}", hash.get_hash_value_in_array()), "[7647F56F, 1508A320, 2303B1A8, D3BB7325, FC4497F8]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[C7DD61D1, 4E88AC6C, FFFC2A7E, C8E2DA66, 01BD283D]");
    println!("-------------------------------");
}

fn sha1_get_hash_value_in_vec()
{
    println!("sha1_get_hash_value_in_vec");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_hash_value_in_vec()), "[58271E8F, 7E54E508, CF099E8F, 4D3B597B, D3BE3F42]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[DA959A5F, A8B581AD, FC006FB0, 9CCB3BCF, 7F4732F3]");
    println!("-------------------------------");
}

fn sha1_put_hash_value_in_array()
{
    println!("sha1_put_hash_value_in_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[BC02B27F, 99A5A1FB, A820CEC4, 19516BC8, E4D2A0D6]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[91EF4936, CFCF8F2D, C581EF30, 450E4E05, 0FBD39A7]");
    println!("-------------------------------");
}

fn sha1_tangle()
{
    println!("sha1_tangle");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let txt = "TANGLING";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[5B296514, 79D48A17, 1ADABF55, 09CC69B9, 83477776]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[6D00CD91, 2A9BAD37, 210A8909, B6A83E2F, 5D986325]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E41C001F, 476FDC14, 1166767C, 3C09AE4D, 447B9B2F]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "TANGLING";
    let mut my_hash = MySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[570C0960, 44388BBA, 0DD84AC9, 2F78A2F8, E514D1FD]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[AE8C42A9, 4CFC9130, FF606528, E4876633, 27FC359F]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[2E33CBCF, 800599AD, 98827D7A, 41AA8BCB, D2D011FD]");
    println!("-------------------------------");
}

fn sha1_fmt_for_to_string()
{
    println!("sha1_fmt_for_to_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "8D0A6284BBFF4DE8D68962A924842C80959B0404");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "54F0234F7188202D98EDDC643F71D95BEDE77ED7");
    println!("-------------------------------");
}

fn sha1_fmt_for_println()
{
    println!("sha1_fmt_for_println");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "835CEFA297628E4DADBDA011C5FDEA68D88A8EE8");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "78083F4E573928D6C4E9F869036F8A4D4D549E9F");
    println!("-------------------------------");
}

fn hash_sha2_256_main()
{
    sha2_256_quick_start();
    sha2_256_new();
    sha2_256_digest();
    sha2_256_digest_str();
    sha2_256_digest_string();
    sha2_256_digest_array();
    sha2_256_digest_vec();
    sha2_256_ruminate();
    sha2_256_ruminate_str();
    sha2_256_ruminate_string();
    sha2_256_ruminate_array();
    sha2_256_ruminate_vec();
    sha2_256_get_hash_value();
    sha2_256_get_hash_value_in_string();
    sha2_256_get_hash_value_in_array();
    sha2_256_get_hash_value_in_vec();
    sha2_256_put_hash_value_in_array();
    sha2_256_tangle();
    sha2_256_fmt_for_to_string();
    sha2_256_fmt_for_println();
}

fn sha2_256_quick_start()
{
    println!("sha2_256_quick_start");
    use std::string::*;
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "559AEAD08264D5795D3909718CDD05ABD49572E84FE55590EEF31A88A08FDFFD");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "3DECCF6826EF78994F099EC321F883527E8218301605E66114268E769D1DF61E");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "9546AE34CBF111CEDC01164DE817512B4DC3B1F9967E208068868BF557E9972A");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "5ADF1754644575EB30E7EBCE1F5EA2AD52E99CDD98713B805B2B2F02CACB3E31");

    txt = "I am testing SHA-2/256 for the data whose length is 62 bytes..";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "12C0E6762B448D5FBFF941D54F932BBFAE308E3EBDEE1795555681D3D9A2C5CF");

    let mut txt = "I am testing SHA-2/256 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "222268D061DF342E7142E79B49EAF57D34B74212D2150C5CA93EF05C767EA5F3");

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

fn sha2_256_new()
{
    println!("sha2_256_new");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let hash = SHA2_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6A09E667BB67AE853C6EF372A54FF53A510E527F9B05688C1F83D9AB5BE0CD19");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "1111111122222222444444446666666688888888AAAAAAAACCCCCCCCEEEEEEEE");
    println!("-------------------------------");
}

fn sha2_256_digest()
{
    println!("sha2_256_digest");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let txt = "This is an example of the method digest().";
    let mut hash = SHA2_256::new();
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F1ECFB4A9F399E3786FD87ABE5D27DB64ADB61F4754BA68EFADCB3792DD15827");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut hash = MySHA2::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F7301C3222B8AA48ABDC3917F24B2E6E408601AC123C26B733E3FBDA494ACF7D");
    println!("-------------------------------");
}

fn sha2_256_digest_str()
{
    println!("sha2_256_digest_str");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B9396CF025B6ECC98178BE081D045DCC2CD3F18FE1450B1B420451A53C571D32");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "853979616624A859070DB313AAE6BFED07A58EFA37571E88276D215AE845645B");
    println!("-------------------------------");
}

fn sha2_256_digest_string()
{
    println!("sha2_256_digest_string");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "0DA7CA307C40C3661BD59AAF2828CAC1D3E3C82385CC8EC92A2FAFF1C0A5DF43");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "662FE0AAEF2070BE79771F3693F0A1CCA8DF6E9E08A5685535D99C77C258F3AC");
   
    println!("-------------------------------");
}

fn sha2_256_digest_array()
{
    println!("sha2_256_digest_array");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "411D3F1D2390FF3F482AC8DF4E730780BB081A192F283D2F373138FD101DC8FE");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "C5E18AD8B82014203663BD271D12D2BF5F51045D0E9A1BBDE7D7A7B10A125DA0");
    println!("-------------------------------");
}

fn sha2_256_digest_vec()
{
    println!("sha2_256_digest_vec");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "411D3F1D2390FF3F482AC8DF4E730780BB081A192F283D2F373138FD101DC8FE");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "C5E18AD8B82014203663BD271D12D2BF5F51045D0E9A1BBDE7D7A7B10A125DA0");
    println!("-------------------------------");
}

fn sha2_256_ruminate()
{
    println!("sha2_256_ruminate");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5EB550DEA1A606FE03338BBEAEB7200003472FDF02556C6E32273C0405EF1443");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut hash = MySHA2::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "78782D14800491F6E66CAA238D955FE11FC9E9750161D51B83429B58AEC3EE0B");
    println!("-------------------------------");
}

fn sha2_256_ruminate_str()
{
    println!("sha2_256_ruminate_str");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "CD24A2825FA0CBAB8D70467D82E92D8BF0CBE86C6B0DAFECB87C254376A9323D");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "BF0DAD806A1D09BA917C08C951CB97F8F51C67D0EE8DFDBFCCEE7E5D6DE288C5");
    println!("-------------------------------");
}

fn sha2_256_ruminate_string()
{
    println!("sha2_256_ruminate_string");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B013EDF34D926ABF2BE04489593A214FFECCA1F5334E31D17BFFEA000E4E85FE");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "6C72B5A23ED16E4533FA3437AF27E9DE21ECA6D0C947CDCB1CC684FBABA3E720");
    println!("-------------------------------");
}

fn sha2_256_ruminate_array()
{
    println!("sha2_256_ruminate_array");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "CD949334D0E7BBEEC7C0B3FDDED14A2FFFA85EA91DCFB171521C5C74CB989925");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "5E2FEBBBED0F9EFC2AFA2C892B9AC09B6A38DA45C0D23633D3C9F58A0C547909");
    println!("-------------------------------");
}

fn sha2_256_ruminate_vec()
{
    println!("sha2_256_ruminate_vec");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "C02DA0025F7228433CB12036E35F60242B2F6C82F55DA1497ACABD491D381EDF");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "D5E63A6B8D2E4F1912AAD8C6C99C9B063CB001A4FB1AD6B3D08A8F0C1B4CF947");
    println!("-------------------------------");
}

fn sha2_256_get_hash_value()
{
    println!("sha2_256_get_hash_value");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 32];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[80, 57, B7, FD, 85, 6A, B5, AD, A5, A8, 25, 83, B2, 29, 91, 7A, 9D, 55, E6, 6C, 06, 4F, 9A, F3, 2A, 0F, BF, BF, D7, E5, CA, 20]");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 32];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[F7, 71, 58, A3, 6F, 47, 5D, BB, 1C, CB, 40, 5B, C7, DB, 13, 17, 37, 13, CA, 6D, 88, E7, 76, 8D, 71, 4B, CD, 1C, CE, 53, 36, DD]");
    println!("-------------------------------");
}

fn sha2_256_get_hash_value_in_string()
{
    println!("sha2_256_get_hash_value_in_string");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "F35FDE269F52ED6D31D38217988B08CB58DC4A5F600ED51AB705DBBD6FBDC4B0");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "34ECF36DCBB56DC00FBC4E0AFA5A3D6AB3858AEE810017784D61717F6A4E2FC1");
    println!("-------------------------------");
}

fn sha2_256_get_hash_value_in_array()
{
    println!("sha2_256_get_hash_value_in_array");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:02X?}", hash.get_hash_value_in_array()), "[3CC694F3, 6ACEFBBA, 6DFC351C, 22F6CCDF, CF8261F8, 52616CBF, B3E7378A, 10A3CFCC]");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:02X?}", my_hash.get_hash_value_in_array()), "[5E76B301, 821107BB, 9B4BEEBB, AAE226B7, 7D038946, BE9025FD, FB26F6AF, 6EB1C43E]");
    println!("-------------------------------");
}

fn sha2_256_get_hash_value_in_vec()
{
    println!("sha2_256_get_hash_value_in_vec");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_hash_value_in_vec()), "[354B3EE4, 6B5A94AC, 197E0B7B, 38E0489C, 66CC8794, 227B95B, A045CEF0, 8565D27C]");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:02X?}", my_hash.get_hash_value_in_vec()), "[992EB41E, 6A60E9AD, B4727373, F724402F, C8C9AC0F, 8C8BF905, 56EAF929, 1817F41C]");
    println!("-------------------------------");
}

fn sha2_256_put_hash_value_in_array()
{
    println!("sha2_256_put_hash_value_in_array");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[1F057A27, D3162A35, 63E11F9D, 7362549D, CB42D322]");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[98223576, 8B1EC66B, EF9BCF28, 6A9A89C8, BB42953F]");
    println!("-------------------------------");
}

fn sha2_256_tangle()
{
    println!("sha2_256_tangle");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[D62ABC88, A57B6A04, B82A9922, C0316859, 3D8DDE86, E0D8783C, 07E34E29, 3F65D373]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[7FC9659F, 17ACCDB7, 43AA0A92, 160137F1, A2A172A6, 1B42868B, 981CA8B2, 98929E8B]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[7D60FABC, 351D79A0, DF04ADC9, A03CE8FB, A7154541, 5DB0A405, CDEE8242, 7D509560]");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[12FFD34D, 9A5B4843, 1D8DBA65, 3C578886, B85EB6B2, 291D1A45, FD72ECFC, AC8D8577]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[FF0C7268, DA3463BD, 6601EC3B, 5D48D7BF, 10C4460B, F11B209E, CBCB2BCE, 08DE13FC]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[FA6EE74C, F322A5D0, E4EFB28A, 6E30F7FB, 5723E91A, F7B0B0CB, 256610EC, 3E6A6A2B]");
    println!("-------------------------------");
}

fn sha2_256_fmt_for_to_string()
{
    println!("sha2_256_fmt_for_to_string");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "46577469D8A5CBCA1AC874A5350E4FACD318A468046816B066117D51DB9D1640");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "6DED905D80768EE8F19D76233902E6CA1417B23A89845C2DA9127FEDD7CCDB5C");
    println!("-------------------------------");
}

fn sha2_256_fmt_for_println()
{
    println!("sha2_256_fmt_for_println");
    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut hash = SHA2_256::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B8338443431AB13309330A8064AF039E39F90CAC334CF8EA1FF0640646AB121C");

    // Example for SHA2_256_Expanded
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "EF27B2954B124469ACD614F1DE4E99B30C418194B614EE19361674F64F60189C");
    println!("-------------------------------");
}

fn hash_sha2_224_main()
{
    sha2_224_quick_start();
    sha2_224_new();
    sha2_224_digest();
    sha2_224_digest_str();
    sha2_224_digest_string();
    sha2_224_digest_array();
    sha2_224_digest_vec();
    sha2_224_ruminate();
    sha2_224_ruminate_str();
    sha2_224_ruminate_string();
    sha2_224_ruminate_array();
    sha2_224_ruminate_vec();
    sha2_224_get_hash_value();
    sha2_224_get_hash_value_in_string();
    sha2_224_get_hash_value_in_array();
    sha2_224_get_hash_value_in_vec();
    sha2_224_put_hash_value_in_array();
    sha2_224_tangle();
    sha2_224_fmt_for_to_string();
    sha2_224_fmt_for_println();
}

fn sha2_224_quick_start()
{
    println!("sha2_224_quick_start");
    use std::string::*;
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "D14A028C2A3A2BC9476102BB288234C415A2B01F828EA62AC5B3E42F");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "5CFE2CDDBB9940FB4D8505E25EA77E763A0077693DBB01B1A6AA94F2");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "8B73B7B79FA0E4EC45AF8B4230F88F314554D503FD88F05A48A07DD3");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "0592A67F23DD6B21CA691041B4682831C61D40E0235CEB59AC557358");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "8949B6F7EB831F47B81E3361135D835E93576ED5BAAA32209303C37C");

    txt = "I am testing SHA-2/224 for the data whose length is 62 bytes..";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "84E6CB12422BB17F614D03B95E0DF142F6FD4EABB69E59A3C7C8A1AA");

    let mut txt = "I am testing SHA-2/224 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "FB297817767C2236610810DC9BE34EFB2FDCC0E0C7E2D0BA736C59DB");

    txt = "I am testing SHA-2/224 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DAA4205BB0B38C625AD8A53DAF1FC8A61AFA33D7513B3615826750FD");
    println!("-------------------------------");
}

fn sha2_224_new()
{
    println!("sha2_224_new");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let hash = SHA2_224::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7");
    println!("-------------------------------");
}

fn sha2_224_digest()
{
    println!("sha2_224_digest");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "279C669E4411237490589A794FC2F0F8E256F8FBC58C520601ABF45B");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "98256F32A77281A8CBCBA9105080A73BB55F0B51CCCBCC4A273D744E");
    println!("-------------------------------");
}

fn sha2_224_digest_str()
{
    println!("sha2_224_digest_str");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5E3731968A757FDFD99F9C9437B1BA2443A66065B362F230AA041C06");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "8A6102A3AB8A7154E78D0FEBE130BA04E508AF7933AC88ED75D34BCD");
    println!("-------------------------------");
}

fn sha2_224_digest_string()
{
    println!("sha2_224_digest_string");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C56B613F567923DC558D7BD4D68A88875DD206C53BCC7329448508FA");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "85229E915A413FA4F90F86A51628834A0D0490B054330E032D93430A");
    println!("-------------------------------");
}

fn sha2_224_digest_array()
{
    println!("sha2_224_digest_array");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "80BB5B27988D4C3E8FFA4429A4D01175498EC57BAE6B9E856A37837B");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "24E317307775C27419D4A65A57A647775216FDEB2416D1B283EB2271");
    println!("-------------------------------");
}

fn sha2_224_digest_vec()
{
    println!("sha2_224_digest_vec");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "80BB5B27988D4C3E8FFA4429A4D01175498EC57BAE6B9E856A37837B");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "24E317307775C27419D4A65A57A647775216FDEB2416D1B283EB2271");
    println!("-------------------------------");
}


fn sha2_224_ruminate()
{
    println!("sha2_224_ruminate");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "30BC4B7A0C1EE66CBE7F708AB7A8510CA94CE5F76D6442D4A8BD8051");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut hash = MySHA2::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5B377DE6E64005E60ACEB32811BA594006955623A34AF2D71D9A8A84");
    println!("-------------------------------");
}

fn sha2_224_ruminate_str()
{
    println!("sha2_224_ruminate_str");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "4410FF448733776380ABF2A899BE858AB8767C3E7DB55F435691F22D");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "7C4BADDC365A4A0E59B7A1195B4FAEB7222568EF43CA49395F3DFE24");
    println!("-------------------------------");
}

fn sha2_224_ruminate_string()
{
    println!("sha2_224_ruminate_string");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "267EE9D909B721671130C62C3CF161EE2AEEED638F7FDC85A96EADDA");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "B05B7CB13228323D392B5DE4D54B40AD9E7D68A4683A7DC689B05489");
    println!("-------------------------------");
}

fn sha2_224_ruminate_array()
{
    println!("sha2_224_ruminate_array");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "2ECBE562B252532846838B27F0ABF9EDC82C7112F9705F0FA77E9EF1");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "0C21E7F3B3E08B5A751E16C701AC2438871084CFF37E5177E15EE168");
    println!("-------------------------------");
}

fn sha2_224_ruminate_vec()
{
    println!("sha2_224_ruminate_vec");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "F04D0993A12818003DF69DC5FF2901F766A3CED0D48766E984B4745B");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "D670D49F1DBBE320A11806DAC55BA4E4B0AB2CB3EB4821E711D02E9A");
    println!("-------------------------------");
}

fn sha2_224_get_hash_value()
{
    println!("sha2_224_get_hash_value");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 28];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[69, DC, CE, 52, 29, 1E, 97, 06, 1A, 6D, 57, 62, 3E, FE, FA, 7C, CF, 74, 35, 30, 40, 04, 5C, 33, 8E, 09, 28, 8C]");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 28];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[09, 89, 84, 69, 9F, C8, DE, 3B, 56, 21, 69, A4, A3, 06, AD, B4, 0F, 7B, A0, 36, 95, 3E, 98, C4, 7A, 5D, 30, 37]");
    println!("-------------------------------");
}

fn sha2_224_get_hash_value_in_string()
{
    println!("sha2_224_get_hash_value_in_string");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "A35C7F70866C2D4224B4F66CDF11932955BA312CE04322EF83A680A1");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "7E9B316E6510BA22C8290C8C14C74C90C540097B1DA9A8D840D2EDCC");
    println!("-------------------------------");
}

fn sha2_224_get_hash_value_in_array()
{
    println!("sha2_224_get_hash_value_in_array");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[95B6C324, ADFEA823, F398E670, 7B44C57B, B7646166, 46A4BD98, 73B076E2]");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[FBC6DEA8, 0D40B971, D842AF35, 5CD5ABC9, 309E88E5, 7592A190, 19968AA6]");
    println!("-------------------------------");
}

fn sha2_224_get_hash_value_in_vec()
{
    println!("sha2_224_get_hash_value_in_vec");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[2D3D3B76, 53CD9A03, 439CFF14, A565148A, 10479BDB, 09CAADF9, 1DD5ABAA]");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[C9CEC151, 8A70C4C8, AB43C3BC, 3BF60C45, 00D4D6A0, 2AE25C0D, 4C212514]");
    println!("-------------------------------");
}

fn sha2_224_put_hash_value_in_array()
{
    println!("sha2_224_put_hash_value_in_array");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[9D43618A, DE2A6358, 53FCAD2D, 6757E0C7, 684E65FB]");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[FF72EB17, FBED9CD1, 0BD59F6E, DABDBA49, EA3BDF96]");
    println!("-------------------------------");
}

fn sha2_224_tangle()
{
    println!("sha2_224_tangle");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[DFBDC998, 897BDD0A, F99B538F, 178A5EE5, 16C96398, 2D544CAF, DC631DE9]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[32CB00E5, 9A09585A, 9051D8FB, F8F6EB0D, FD467652, 46408C7F, F5DD61C8]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[7F5D4897, F323EC3E, D47C95D5, 9D77DF01, 9269E780, 3973310E, 142EB013]");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[9196ACFD, 94E19450, C9B7D8D3, 220C86A4, 6AC1EE8F, C87D73B4, ECFEE637]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[4D71E0F6, 41D78805, 94358C2C, FAC2356D, AEB666BB, 80880239, B2D1304D]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[851BE861, 5E595131, 072DF35A, 973B5D59, 87DBDC1D, 68BF05A6, 48EAC080]");
    println!("-------------------------------");
}

fn sha2_224_fmt_for_to_string()
{
    println!("sha2_224_fmt_for_to_string");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "979DB3C5F63C2FBB32A72804A991534EB38884EB5B9131AE0EE3A496");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "136C899347821BCC7529F3B42C0A9E3E997E156B1E5E081F57BBB15E");
    println!("-------------------------------");
}

fn sha2_224_fmt_for_println()
{
    println!("sha2_224_fmt_for_println");
    // Example for SHA2_224
    use cryptocol::hash::SHA2_224;
    let mut hash = SHA2_224::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E333EE19A56FCDCCB05957F2B6FB0AD1EA11D7B6258DF28DCE3B526B");

    // Example for SHA2_224_Expanded
    use cryptocol::hash::SHA2_224_Expanded;
    type MySHA2 = SHA2_224_Expanded<128>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "849F654BAFF41D3025DE982EC410F8EC6991FFD6E5BF4047F45082F6");
    println!("-------------------------------");
}

fn hash_sha2_512_main()
{
    sha2_512_quick_start();
    sha2_512_new();
    sha2_512_digest_c();
    sha2_512_digest();
    sha2_512_digest_str();
    sha2_512_digest_string();
    sha2_512_digest_array();
    sha2_512_digest_vec();
    sha2_512_ruminate_c();
    sha2_512_ruminate();
    sha2_512_ruminate_str();
    sha2_512_ruminate_string();
    sha2_512_ruminate_array();
    sha2_512_ruminate_vec();
    sha2_512_get_hash_value();
    sha2_512_get_hash_value_in_string();
    sha2_512_get_hash_value_in_array();
    sha2_512_get_hash_value_in_vec();
    sha2_512_put_hash_value_in_array();
    sha2_512_tangle();
    sha2_512_fmt_for_to_string();
    sha2_512_fmt_for_println();
}

fn sha2_512_quick_start()
{
    println!("sha2_512_quick_start");
    // # Example for SHA2_512
    use std::string::*;
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "CF83E1357EEFB8BDF1542850D66D8007D620E4050B5715DC83F4A921D36CE9CE47D0D13C5D85F2B0FF8318D2877EEC2F63B931BD47417A81A538327AF927DA3E");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "21B4F4BD9E64ED355C3EB676A28EBEDAF6D8F17BDC365995B319097153044080516BD083BFCCE66121A3072646994C8430CC382B8DC543E84880183BF856CFF5");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "1F7B98E0B65D4CD1DE4C2B9EC77CB5F7FC4C20006BDD1380F7E2A9C95BD5F47162B868E724FEC68ECE02B8C3F79BE0C4AB73EEF8AC84B5537063B1A353074735");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "EE74E2DD1FFBFC17A48C80FCBF6A0C55D0A0B4E4F94EDEF7506D28D48EAA5F4DDD7490B3A9CAF212C0CE2101ADABF0C32E4BD91694B8B284C5FAAFE6F54B63D7");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "3893170A81639FB341477704BFB1CDBB8A222F8DAE98B28505EF0552B86DCE1D630ED80DF6CB34D69CD62ADBD88EADD26B550FC9C3CCD7DEFDE4C71594108348");

    txt = "This algorithm SHA-2/512 is being tested with this message the length of which is one hundred twelve bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "134BDEE3708226A589B53F23ED4559203B603D61239B6EBAA4625D4A95ACC5F98182D615194D4035E3CFED16916477C18889E64980A35263B62B8361131B01D4");

    txt = "This algorithm SHA-2/512 is being tested for this message the length of which is one hundred sixty-five long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "38954039BEA7BFF8DD1DA6E6672A68BD8642F5C4DB7C7CFE11DB2D339DC8FA4EBBC4F1BDC11B4FEC71CB9C84B55FBA85CB11EC4BF72937232044BD004CC90CE7");

    txt = "This algorithm SHA-2/512 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BB87AF8EFB6903DD0FA504AA12E223B00224FF1B6ABF568D7E9C53F65639242E3C0C635A44490D20B4C4B27E748A7675F0C973B6F2784B1105BAFEB91293F0BC");
    println!("-------------------------------");
}

fn sha2_512_new()
{
    println!("sha2_512_new");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let hash = SHA2_512::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6A09E667F3BCC908BB67AE8584CAA73B3C6EF372FE94F82BA54FF53A5F1D36F1510E527FADE682D19B05688C2B3E6C1F1F83D9ABFB41BD6B5BE0CD19137E2179");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "11111111111111113333333333333333555555555555555577777777777777779999999999999999BBBBBBBBBBBBBBBBDDDDDDDDDDDDDDDDFFFFFFFFFFFFFFFF");
    println!("-------------------------------");
}

fn sha2_512_digest_c()
{
    println!("sha2_512_digest_c");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method digest_c().";
    hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "345156BD2AC721CE25C08D57290D1C0DEB60E44B9B7BFBC16600F26A2061AE9AF6CC990F30E3B8C3BF0EA43DAFB17DD3C37C1CA2222ACE2E828A676443EF4F1C");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_c().";
    my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "24354EDB98432813AF297B6BEEDB47F3563BBDDB29FF7F9F25996799BD46BB6364181CA4A8073978D14A345F4A43CB518BBC5D5F8BD6524904C840055B525D2C");
    println!("-------------------------------");
}

fn sha2_512_digest()
{
    println!("sha2_512_digest");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7DE9DD6917A9B3353EA426F9C43894513095E944DBE0678491DDABAC0D87236E007374B7438231AB84DE91271F1BCCD11BA64AEC24E3BDC410DE1115A075E2DC");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "54D3E18AD977B18F4E3254FBE759C6D8072EFA95A88C671610C16A19D05253A9B3762FE32D054BADBEB877735287A47C1CD7439CA3AE8BE12489D0E8C7F73945");
    println!("-------------------------------");
}

fn sha2_512_digest_str()
{
    println!("sha2_512_digest_str");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "5FD3D145014F7886E64034CC082ADF48670E797DA1C2DA54DDEAF5513E028EB3712121FE6305DB6524C12CBBBB93DF3C0A4DA54E8D798E2AC2A29BA81FB3BFD9");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C11712E6B0959FE9C3C9ECB85312166C7667E768FF6F7BBA80F1668BD898A70E65B34B08CC8BC73F85049971EC0469B2FEA4BB1DB7F8DAC9D5236949F2CAC472");
    println!("-------------------------------");
}

fn sha2_512_digest_string()
{
    println!("sha2_512_digest_string");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "768AF82F599E230387C1F4A4BA2F97F59C6C96B76735A61CFFF3300E808EE0D9FF497957456BB61AABD0F88C19790F0675DD586DC0F5722C60DCB5BB27D6853B");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "8ED6D614A610C778BDE4A4AFF79C4A824821678FEBEC9C4AD34B59FC113A357598001B2209CC8F06A1E0A0F04A59D84F507BA4F288FB7AF5E8FDCEFC843ED371");
    println!("-------------------------------");
}

fn sha2_512_digest_array()
{
    println!("sha2_512_digest_array");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4582725B4E904C9FB7C4072B2E4665FB3E4ADC03CB8061C91C0283D582251EA08226CF5A84D9EE441FB30ADB0AB7E564DD66CE8A2BC2BA4B0E32AD36E3BB1253");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A4880984CEB88A123E79A561B1C4C415267C51A155C915CF99A788A83609A3CA651AB46C0AF33484F68AC73C76E88E00039BC3EAE0649D97F1379009C633D506");
    println!("-------------------------------");
}

fn sha2_512_digest_vec()
{
    println!("sha2_512_digest_vec");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4582725B4E904C9FB7C4072B2E4665FB3E4ADC03CB8061C91C0283D582251EA08226CF5A84D9EE441FB30ADB0AB7E564DD66CE8A2BC2BA4B0E32AD36E3BB1253");

    // # Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A4880984CEB88A123E79A561B1C4C415267C51A155C915CF99A788A83609A3CA651AB46C0AF33484F68AC73C76E88E00039BC3EAE0649D97F1379009C633D506");
    println!("-------------------------------");
}

fn sha2_512_ruminate_c()
{
    println!("sha2_512_ruminate_c");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method ruminate_c().";
    hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "52787131CFC0C4C058147A8620A4592F9077E05796853761B9846E5A141F79D5C833F4C31754A0C39C6A111A5C9884703838F61544F31FDF39B74D07F3F04CCD");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut hash = MySHA2_512::new();
    let txt = "This is an example of the method ruminate_c().";
    hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3A1097A01FE71CBD6F8EAC1D08D7FEAD5754F077DE1E997B0411A60313EAC182C82BAAFD550AC367D029CCF3B7DE823475EE0AFCAEAF9388BCA09262C28F730C");
    println!("-------------------------------");
}

fn sha2_512_ruminate()
{
    println!("sha2_512_ruminate");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C19B01562E1A198DBDBB2C6CB093277109B9130E19B0A0EDBFF67D9690BAE5A5D12551257CA7DEC7AB02C27ADB8AD98D3281D376EBC0F944CBC04D7D87A80D11");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut hash = MySHA2_512::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9DD3646C38AECA875339D816DAA59A69A890F7098BF8BE7EF2714834EDB34C572555D609BA8D009BA65BE7E98E4A64CE80FCA8D2C6308085B0471F3758C17081");
    println!("-------------------------------");
}

fn sha2_512_ruminate_str()
{
    println!("sha2_512_ruminate_str");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "4F596A3417C5679D5CB7E02F4980FD438272D72C33BFD582E98F7A4AFDB2474332735D13E3D70FFB861BD12D688A4883F95611DDE2D049C082DAEEE78E5A3CAD");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "25B3F403FD3A849877E8101786E48FB0EC5137A6874B5523B8D86A47ED3834FA7926CF6FE648A5BE479508D6353925288D51732E4BEF768F5C759A398E4422EC");
    println!("-------------------------------");
}

fn sha2_512_ruminate_string()
{
    println!("sha2_512_ruminate_string");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "1F603531B915614903A42531F646696333DA0CCD35EC0F051A3EC682D4597D5137839D9CA8C8596145000893A209496E68620AE80113E12FB7C0DC8D379A1708");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "8720DD2A837C0D2E6B16597C5217F752AE9F9686ACA77AEDDB999912689EF6589ABC435CEFB5423DD81B79639E3799F900BDD90B9FA121776A176A7518C1C5AC");
    println!("-------------------------------");
}

fn sha2_512_ruminate_array()
{
    println!("sha2_512_ruminate_array");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4471964A862473FB47151E54C87AF4E2AA6B3AB3E61E9A97C9823166CD7F5BA88B9305008579F88A83DE45CC8554F3844F8DE03259393B6DAB69B822491ACE2C");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "827C229D2BADA0470D6BE41F22AD045D9AC6D4438F9CCD0697BDF3DDC23F2A2831C744B07D696F66BB56DBF50947A05F60F904D3D984BCCE6FC3A88DC05B27EE");
    println!("-------------------------------");
}

fn sha2_512_ruminate_vec()
{
    println!("sha2_512_ruminate_vec");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "47054627D6E9D65A9E5C9A4419D8F3BC2E446082C65AFB80691800F73237E886E6CD593D59EA30DCD5629A0B13B84C1D3E2F046765ACCD999DBE755E77F2E64B");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "1612E8ED7FDB80AEAF3C2B12853050086F6DE6694D50EFC4327C1F8954D870CFBFC49898DFAAF458DE671C6FCA101894E33C61DC0300D4E65584F3BB0B5E720C");
    println!("-------------------------------");
}

fn sha2_512_get_hash_value()
{
    println!("sha2_512_get_hash_value");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 64];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[82, 83, F3, 64, BD, C8, 87, CD, DA, B4, 28, 91, DD, C8, 8C, 67, 3F, F1, 72, 32, A4, B9, F7, 56, CB, C4, E5, 3B, 1D, A0, C8, FA, 74, 54, 8B, 73, E9, B8, F8, 4A, 55, 5F, E0, 4C, 61, 34, C2, 68, 95, 63, 82, 61, A9, 9E, B7, E2, 8C, 85, 88, A5, DC, 1B, 57, E6]");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 64];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[FA, A6, 41, 73, 35, 6F, CB, 88, 1B, 22, 73, 65, DA, A5, 0E, 87, 2A, 63, 21, 1B, F6, 1B, 53, DC, 4A, 82, 6C, A4, 23, F1, 3F, AD, 45, AB, 30, 0A, B7, F7, 5F, 3B, C7, 8C, 2B, 7F, 87, A1, 38, DC, 46, 00, 53, B8, F7, 3C, 8D, 83, FF, 8F, C8, 1D, A6, AC, 97, 2E]");
    println!("-------------------------------");
}

fn sha2_512_get_hash_value_in_string()
{
    println!("sha2_512_get_hash_value_in_string");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "8AECA96A2928FC8E01F28B998ADA3AE3A077A881F81DD30F2BCE777FD86909F3C9D9324740FB6A1DA384F9ECBFD703F905946E71F4EDBD002C495D38F1241570");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "DA49529344BD8D46A494A3F89EC15CC28719415451041B88369E0593AC3A3B284D0FF10FA4798C3CDC4336AD4DB18F50040194D4E45C6ACE6E948E47822298C5");
    println!("-------------------------------");
}

fn sha2_512_get_hash_value_in_array()
{
    println!("sha2_512_get_hash_value_in_array");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[269EC4F05FAC68BA, 353F93F8BDDB9C79, F9F7D199992E1A57, EEEFEB72D428F890, 5908F0C5E3A56D27, 7898E80A2E56602E, 32829E8112CBE584, 1CF7DB82D5FB5C7D]");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[394BC3D25DE16154, E2615B88BA96BADF, 1AB83CF6DBCF191B, CFBFA0DCD1DEE1EE, B056B80296B6D337, 8AC4D3CF0442A805, 2966C7740FDAE6D0, A51D928A5E113A21]");
    println!("-------------------------------");
}

fn sha2_512_get_hash_value_in_vec()
{
    println!("sha2_512_get_hash_value_in_vec");
    // # Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[FBCC0C9263024A5E, 00F731E672F30E0E, BFDFF3CE32DDD0D0, 585F2A2FA043FB41, 4EB2BFCD5492BBFF, 6B353AB79E099410, CE242F09012B55BD, BEB308A492F01A5A]");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[63716011B33F95E0, 645286436AB1314A, 10D1181DA43D48C0, BB48FCA82574C99F, 1696A11789092457, 3447093E613453F3, 28D9CC66C338681B, 5C59FFBF76683176]");
    println!("-------------------------------");
}

fn sha2_512_put_hash_value_in_array()
{
    println!("sha2_512_put_hash_value_in_array");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 8];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[07D97419B0FEF635, 22315DF49563FE48, FF291B21CDF1AEE8, 012FEF9A0DE3AF09, D9BA34115A180A53, 2282C4D2365E7B4E, 453ACE857528771B, 7065933387341E8B]");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut my_hash = MySHA2::new();
    let mut hash_code = [0_u64; 8];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[A547D955E06569F4, 1D04F93ED5509CC0, 15E5E8E418642ABC, D6C5B621DE575B15, 0C76DFB01A20113D, F2F28AFD20895868, 50B473890ABD75FA, 9682EDEA72F26C67]");
    println!("-------------------------------");
}

fn sha2_512_tangle()
{
    println!("sha2_512_tangle");
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[070B6A9457F65DD9, A7D2C2326CE14E8A, E870D6939FE02E39, 5CFEEDCA96BF3BA3, 013FFB332B3F51F3, B1D4E16355DBE0A9, E998240787066535, 1D5F597F04F84820]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[4780AEEAD19D5962, C55EAFBA7590FB70, CA6587899B2B276F, 55361EC5C9568667, FFD38C58FF62C288, 5E96A9FFC6B17704, 6D3885C75FE9B667, BFDA80D1514F38E5]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[D7FFE2BEEB81D532, EA420969761C4DAA, 8EE930740ABBBE3E, 0DC90C0705AE5F38, E91531243615F994, 174C4F96168FBFC4, 06373FFDD9C66A16, 910560A5898E3728]");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let txt = "TANGLING";
    let mut my_hash = MySHA2::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[D0EDE5AFDDAB96B5, 78B6CC968AFB83EB, CE2369C35DA4A43F, 4B753CF1D02A1A3F, 29A3861EBD42210C, 952536C0957B0B60, 675FE725336E105E, 6E2ACB9D03A95AD2]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[5C72DB128F57491F, B70402F02D41A779, 1B9B1C9979BD59AF, 90ABF522230D4DB3, 2330B855BB6C253C, 297D4E6FF6B37F70, 929F3A8F3CB9A7FD, 3EDD2459251BB838]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[A2090D429E425CEC, D6FD81EEE61ED3B5, 34D1E87A7B4B06E3, 7415804887A7528D, 89EF9F2F4F6CC538, EED8FE585C02AF99, C20EB506C486C145, 730E9AA7A3B591E6]");
    println!("-------------------------------");
}

fn sha2_512_fmt_for_to_string()
{
    println!("sha2_512_fmt_for_to_string");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "4253800692B979FD12F63DD77380BF391AAEC2FB7C599BD447A6E9690F1E7CC06ED615C61CB27514B64F56ACD423A3AC6BE2AEB637885786CE720F1516E38BAD");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "F4303A191D2C24F0990BF42A1BAFF613FDCA377C352CF7E1BAAAD599A799066762756E620DA5E8402607275E3F9CD70A2EA2FD63B2FCBC52B150EF62CAD2C9A5");
    println!("-------------------------------");
}

fn sha2_512_fmt_for_println()
{
    println!("sha2_512_fmt_for_println");
    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut hash = SHA2_512::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E960F9A2E39AEBB1C1654B5B7408819AE4507F6983F2D592F232CB64C2CD4DB7265DBF5BCDE9DADA7A1B26618E55B39E54C7A9EC6777B5DA70132160C8E4C837");

    // Example for SHA2_512_Expanded
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2_512 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    let mut my_hash = MySHA2_512::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "9346E639E089AFE7D39CD85EF7C61EF0941C0A2F6641E2AE39EA8C44DDBEEF098AB2C32761256FD7450AD70B1B0965038D477ECD8CE3A1EECA634A177DC9B975");
    println!("-------------------------------");
}

fn hash_sha2_384_main()
{
    sha2_384_quick_start();
    sha2_384_new();
    sha2_384_digest_c();
    sha2_384_digest();
    sha2_384_digest_str();
    sha2_384_digest_string();
    sha2_384_digest_array();
    sha2_384_digest_vec();
    sha2_384_ruminate_c();
    sha2_384_ruminate();
    sha2_384_ruminate_str();
    sha2_384_ruminate_string();
    sha2_384_ruminate_array();
    sha2_384_ruminate_vec();
    sha2_384_get_hash_value();
    sha2_384_get_hash_value_in_string();
    sha2_384_get_hash_value_in_array();
    sha2_384_get_hash_value_in_vec();
    sha2_384_put_hash_value_in_array();
    sha2_384_tangle();
    sha2_384_fmt_for_to_string();
    sha2_384_fmt_for_println();
}

fn sha2_384_quick_start()
{
    println!("sha2_384_quick_start");
    use std::string::*;
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "38B060A751AC96384CD9327EB1B1E36A21FDB71114BE07434C0CC7BF63F6E1DA274EDEBFE76F65FBD51AD2F14898B95B");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "AD14AAF25020BEF2FD4E3EB5EC0C50272CDFD66074B0ED037C9A11254321AAC0729985374BEEAA5B80A504D048BE1864");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "63DD51EE49AEDD57E85F8BF9A9CF53595FF212BF2E334845AC14CAD17F137C2221D065F8143FB39D3EB2612DD4B429CC");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "813660BD8EABF78896F5F33727067071635BDACE0E81C158E32E7EB3626820887C42F83E6D9CE973B71B6A0C50C753C0");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "435138D7CC9CE82E375B13FE3C75301EB670A8BAFDE4A1952D8D33225E464A62D5747F66F68C8289C5E8BB4C45E16A42");

    txt = "This algorithm SHA-2/384 is being tested with this message the length of which is one hundred twelve bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "31D21701C66D9226B1ECEEB713100ECE0C06A1DDCA1EA5B66286316E31B288C7E5147A796195C1A2D93006FFB5372B74");

    txt = "This algorithm SHA-2/384 is being tested for this message the length of which is one hundred sixty-five long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "8B2E5B6105E561A42FC0BE177FEB784321FC67A5024456A48C6A4B481FE483AA3F57A7F57FAE41471362870892465627");

    txt = "This algorithm SHA-2/384 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "6FE78FC9871EBAF5F19777B7C47B49DB400154DC58684808F06C47CAD1428B46C8AFF2F77C438CCFF287DCA6C60C72FC");
    println!("-------------------------------");
}

fn sha2_384_new()
{
    println!("sha2_384_new");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let hash = SHA2_384::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "CBBB9D5DC1059ED8629A292A367CD5079159015A3070DD17152FECD8F70E593967332667FFC00B318EB44A8768581511");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "CBBB9D5DC1059ED8629A292A367CD5079159015A3070DD17152FECD8F70E593967332667FFC00B318EB44A8768581511");
    println!("-------------------------------");
}

fn sha2_384_digest_c()
{
    println!("sha2_384_digest_c");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method digest_c().";
    hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "49B260A01AE5737FD153A7C08700489B23A1D3150657EA88CBD834D89A7305B223C4CE52A682E5259FD16B2EF5BBA14D");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_c().";
    my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C9282F31FFB5255BE04EA08708D85719C262AE1ABA9B9B1A99BC05E23DC5F8BE4E151BF327CD4C25342A292281FF9CA2");
    println!("-------------------------------");
}

fn sha2_384_digest()
{
    println!("sha2_384_digest");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "1EFF9CDB108E9FC430650DC0A8FB7195654B225B013ECF90F7949077A299D04A921997536D0E1941734ED63FA68AF5E2");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "0A02CF201EFC3EA6FD5EF340487CBFDF8EECF6EC97F917C6519635696352FF08FE171445C5A724849ACE4BC3475C6C32");
    println!("-------------------------------");
}

fn sha2_384_digest_str()
{
    println!("sha2_384_digest_str");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C1C8355C211B2DF4D562014768ECDF21973D60A25EC0C1038C11510E9996084F4871C15A3578BECDF6EAF2F62A8A56C1");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "07F84B565CBCAD7488A350405DF06BF061F158180C61B25AF384A48B971A9CF0211B0764DBB705F93F8BD02BFF6BB8D6");
    println!("-------------------------------");
}

fn sha2_384_digest_string()
{
    println!("sha2_384_digest_string");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "19EA6204374E0C4DB800813E7665350754E7B5E5E3A2FC9B95F3F164D7F1E0493D48F2C4ECC32E2F147EB7789F35B9A4");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "E6EC8180FDDCFAE34110E21512CDC75D481A72A8BED777A43B56845FEA29A993AFA558B3A2F07B9998A1C238BDAA8FE3");
    println!("-------------------------------");
}

fn sha2_384_digest_array()
{
    println!("sha2_384_digest_array");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9F1D9A9407C018C8D95E4CFBC839121AA45521AC2C6AE0F3140E657A1A55384D7F32ACCBD5FCABC27DD7499DC3DB3F6C");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "F823B958E739FD7F39AED9DB1D02D146028E1D3041FB922AE0F20C7F95216D0288F16148D5AA6438712F9C4502561C07");
    println!("-------------------------------");
}

fn sha2_384_digest_vec()
{
    println!("sha2_384_digest_vec");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9F1D9A9407C018C8D95E4CFBC839121AA45521AC2C6AE0F3140E657A1A55384D7F32ACCBD5FCABC27DD7499DC3DB3F6C");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "F823B958E739FD7F39AED9DB1D02D146028E1D3041FB922AE0F20C7F95216D0288F16148D5AA6438712F9C4502561C07");
    println!("-------------------------------");
}

fn sha2_384_ruminate_c()
{
    println!("sha2_384_ruminate_c");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method ruminate_c().";
    hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "536CCC8C5BDA57DA3F130EDF2FBD2077BCE4A77CCB6719037DEFE738F76672EA7DCF6BDDDEE8C2E0FBD6A6E97496018D");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_c().";
    my_hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "0F9AEA2C6450A05CBF6EDD67424B598EEB25DFFA347E16EDA1F23682AB16FDB59EECFB1D4FEC3FDBC47569102CB90A7C");
    println!("-------------------------------");
}

fn sha2_384_ruminate()
{
    println!("sha2_384_ruminate");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D192674BCF9C76D86BA8F0A0A3615A4909788D23928CE62260D981E9DE6A6A98B3095BF03B6F124004C9672E1D784270");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate().";
    my_hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "767C864B487A2DA254618CAB19F4C549386AB2AFBE0B7881306FD978F705D6D034C4BF5488BCB179A7CDC2B8850F585E");
    println!("-------------------------------");
}

fn sha2_384_ruminate_str()
{
    println!("sha2_384_ruminate_str");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "31C3BF7127338D6C50AC0F9206119A0259575E6F1501DB38424900B6F2F74C0C9BCE58D369460F6FD3AFFBCD4CD7E460");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "701A4470B80B6B186B923FAD153CFB26489DBAC7E4D118F2339D0EA7377E49F9BCFDE162CAEBC266AA99E95FE62BCB1F");
    println!("-------------------------------");
}

fn sha2_384_ruminate_string()
{
    println!("sha2_384_ruminate_string");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "172F67A32233BD0DBCD6B247133B068F9F8474530B05B14A7010792723799955C80A211DA827798E0831302345A6EEBC");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C9AB85F3121E58C31EFB5ED1C4D5E2B6716D4E07730C625102739A7A924142712C8C64417D48AFD5FFD31FBCFC933213");
    println!("-------------------------------");
}

fn sha2_384_ruminate_array()
{
    println!("sha2_384_ruminate_array");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "C4DD76B9055F78330DEAF6D39E2B8B377D86635BACC6C32D95FEA325BD2DCF8D7020AE239FD06BF20E3F429139F1C2E0");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "5159525B15E8FB22A4C9ACA9469D799EC0508B02024A332478997BBF00ADA27CF241D64677129816665114894BC9CB24");
    println!("-------------------------------");
}

fn sha2_384_ruminate_vec()
{
    println!("sha2_384_ruminate_vec");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "CA06A7B11B969F6A7A5C5AE9A60BBFE968F7A08F6AB56BC126E2CA526AD0B10D357570CF57684706539F3DEACE1D2657");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "798B6EEEE78278B41253DCD8C9E859DC3E566DD0C9AC9CC1B7106D1471C2FEA715B797357AA38F6E07C3B6A3B8C30E4B");
    println!("-------------------------------");
}

fn sha2_384_get_hash_value()
{
    println!("sha2_384_get_hash_value");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 48];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[E2, 4E, 92, EA, 76, 4B, 51, 3D, C2, 9C, DD, 6D, AD, A0, F7, 0E, 76, BB, A2, 90, 14, 07, F1, 58, E9, E3, 5B, C4, ED, AB, 1C, 7B, FF, 09, 55, AF, 11, 06, 42, B8, 01, 05, D1, 2D, 07, E1, 65, 0A]");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 48];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[0D, 87, C0, 3B, C1, 51, 82, 3A, 47, 93, 57, 71, 7C, 35, 38, 7C, 91, 07, F3, 9C, 00, 65, DD, E0, FF, 3B, 00, D4, C8, FA, 31, 74, A9, CF, C0, 5A, BF, 08, 6B, B2, C3, E5, 5E, 67, A0, 9F, 05, 5C]");
    println!("-------------------------------");
}

fn sha2_384_get_hash_value_in_string()
{
    println!("sha2_384_get_hash_value_in_string");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "E7C4CC51082A86C9375152E4D5B3E56765BD977B01DD73FBAE40813C79489C3371C3F3BBFB32E24D92ADF5D7E9EBB3E6");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "B5A69AA6E8935C95A92732FE26C5DC08A5ABECC7C65EC0D9DDAA81DEB2C35C9313C97324BE6CF0D5BD2BDBE6847DC7AB");
    println!("-------------------------------");
}

fn sha2_384_get_hash_value_in_array()
{
    println!("sha2_384_get_hash_value_in_array");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[565536E8E13B2B77, 3198A350773A615F, 4A1662ACCA3E37BF, 0ACB01C0B4CA5835, AADFB96DEA6C3700, 9943E16090B5C3B2]");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[C2F79D1802AF32FB, 04A6C2DCE1469EDE, 873021406C6F5150, 668C5D447487D936, 0EE1E7FBFCE57874, E7B26F1CBDABDCE7]");
    println!("-------------------------------");
}

fn sha2_384_get_hash_value_in_vec()
{
    println!("sha2_384_get_hash_value_in_vec");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[98BFC20B66F8BB64, D836C83396818F2A, 2E7652BE1015779A, A04C5E74BE242153, F2E39D3E4803B94C, 7A3508C7DC8C54BA]");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[248315764518254F, 16261F8AC8A9417D, C0130B8A99AAC1F0, 24B1BAECBE3F2152, 289F33C7B3BAF31B, 139666D6111F3DBA]");
    println!("-------------------------------");
}

fn sha2_384_put_hash_value_in_array()
{
    println!("sha2_384_put_hash_value_in_array");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 6];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[101BAFFFD51E0B73, 4D42301EEF84B747, BD35EB42A7EC8FBA, F5B34F2847AFA64F, 9F4BB521F6DDA64B, 63D6B71D7B2F9276]");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 6];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[35B29BB56298C4D9, C2B7F762C276B7AF, 538A54F101A1DCB4, 3C32DC7529E9531F, D06D169C17EFD744, E98D5288D151530F]");
    println!("-------------------------------");
}

fn sha2_384_tangle()
{
    println!("sha2_384_tangle");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[A52945B3E9E6E2E0, 7208374E02CB1DFE, 9481D881D89B7946, C425DF584817FD25, 49001993DD7EB02E, A5BF4D24B77D621E]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[84BE10E10BEB5A66, AF72D1F8D4A763E7, 1B2DFA37B163EDC6, CEABC9EDAC24CB65, 7845447250E564EC, A4FAF9EAEECB878B]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[707481D3670B0FA8, B89726EA56C4170A, DF8C93E221E240BD, AA0DEAEA3D1C891D, 4B8DF37A322EF5FA, E88A2A9E835BAC4D]");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[8A515773672A0C7A, 8CA30FEB93D3A13D, CB81222CFD104F01, DEAA36FB688514FE, 01377A73FCD823E5, 1E44AB0506043A7F]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[D6DD49E21832A216, 3676FE0F8EEB0A8D, 4029F8BD7C7C64CC, D47CA3DAE698F1CE, 6BA349E4F33F2853, E1A939130FE9CD81]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[CD00FDD3A9E9F113, AF71F8BC3F147BBC, CF679991FC2D4957, 2DA56392E6B94D9F, 749AD435F6772132, 50CD667F09190781]");
    println!("-------------------------------");
}

fn sha2_384_fmt_for_to_string()
{
    println!("sha2_384_fmt_for_to_string");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "20109C9D8199547993C91DCC64C07771605EEBC0AADD939E84B98C54C4CCF419B0CD73D5C1D4178902C9CD115077656C");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "A636F434A3E693297DCF48ABFBAA335A824BB55936819E5EC047296AE2E454FFBCDB804C88CAA7DF88E920EE82ABDD00");
    println!("-------------------------------");
}

fn sha2_384_fmt_for_println()
{
    println!("sha2_384_fmt_for_println");
    // Example for SHA2_384
    use cryptocol::hash::SHA2_384;
    let mut hash = SHA2_384::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "222E7F7B1DB2A566D0665D5790B2A4373F006850F06C1E3E83CE6021AF8761BC738BBF10F75A8E45BA09BDB0814DD8E6");

    // Example for SHA2_384_Expanded
    use cryptocol::hash::SHA2_384_Expanded;
    type MySHA2 = SHA2_384_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FEB6C0BE39CD441862DB4FA05003BE1F9E519E88750C279F5D8807DFD771C10233134D334FF60137BDCFC14ECEE78F6B");
    println!("-------------------------------");
}

fn hash_sha2_512_256_main()
{
    sha2_512_256_quick_start();
    sha2_512_256_new();
    sha2_512_256_digest_c();
    sha2_512_256_digest();
    sha2_512_256_digest_str();
    sha2_512_256_digest_string();
    sha2_512_256_digest_array();
    sha2_512_256_digest_vec();
    sha2_512_256_ruminate_c();
    sha2_512_256_ruminate();
    sha2_512_256_ruminate_str();
    sha2_512_256_ruminate_string();
    sha2_512_256_ruminate_array();
    sha2_512_256_ruminate_vec();
    sha2_512_256_get_hash_value();
    sha2_512_256_get_hash_value_in_string();
    sha2_512_256_get_hash_value_in_array();
    sha2_512_256_get_hash_value_in_vec();
    sha2_512_256_put_hash_value_in_array();
    sha2_512_256_tangle();
    sha2_512_256_fmt_for_to_string();
    sha2_512_256_fmt_for_println();
}

fn sha2_512_256_quick_start()
{
    println!("sha2_512_256_quick_start");
    use std::string::*;
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "C672B8D1EF56ED28AB87C3622C5114069BDD3AD7B8F9737498D0C01ECEF0967A");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "65A992AD19967492B5780D76A4733AF553F796F688B79102D01EC7FDE5590CAB");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "E4AF36E824AFDB9E42291983AFA292B894DED2CCAFCCF53346B223FCA846694D");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4E730BDADF49EC9F3E920F72EAD3AC8D09B459900BE4F6E27848652632277205");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "AE0EAB6824897F575FCC051DBC2D1AA7F7BF0DB2C80172F639CE20B3B498C9D5");

    txt = "This algorithm SHA-2/512/256 is being tested with this message, the length of which is one hundred twelve bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "7876C6F1285C4B6EC6A2F4A76BBF81815B470536F3A38B7028AA88A3C5C31651");

    txt = "This algorithm SHA-2/512/256 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "6FCE377EA6116BEAC9C11606C59A5D034C8C6EF5A1920B783A9097E07BE36D31");

    txt = "This algorithm SHA-2/512/256 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "63FD06E11EF67F0F5EF598C3B2F2E221D5557AD1EEA46156D1B657F1EDF08D5D");
    println!("-------------------------------");
}

fn sha2_512_256_new()
{
    println!("sha2_512_256_new");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let hash = SHA2_512_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");
    println!("-------------------------------");
}

fn sha2_512_256_digest_c()
{
    println!("sha2_512_256_digest_c");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let txt = "This is an example of the method digest_c().";
    let mut hash = SHA2_512_256::new();
    hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "AE67F5B190BB09DC615859EC2D11736DA6CBE00340EE39396FE76257238E3AF1");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_c().";
    my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "03637D7BF4A7D12D2BE672F2D2F5E904EFCC59CC4D2A8844016A5704E79B976F");
    println!("-------------------------------");
}

fn sha2_512_256_digest()
{
    println!("sha2_512_256_digest");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FC30259340B6326E1C8C5B91AA8554A16B83959E36F446781E9C96B01D6B9BA1");
    println!("-------------------------------");
}

fn sha2_512_256_digest_str()
{
    println!("sha2_512_256_digest_str");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D0ED13389E431C8D74FE6E8DB5B6194682874B52E800524136E35D7E9CFA496B");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "178C940993A48B3D92CCEA2134756DD60914A50125A027F4E220B361908FB2AD");
    println!("-------------------------------");
}

fn sha2_512_256_digest_string()
{
    println!("sha2_512_256_digest_string");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "2824B79B5D8A1D02454612B72B9CD9544D0DF8E126E7A01E55AC479B0903297C");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "F5639225E9CD74CFB9EC5292F816053C1993E7ED1F98AF98C641E193349DD376");
    println!("-------------------------------");
}

fn sha2_512_256_digest_array()
{
    println!("sha2_512_256_digest_array");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "93D5013B2C9AD16AD2B661EC130D376C70958B20BE9CC85D02CA691795EDD39C");
    println!("-------------------------------");
}

fn sha2_512_256_digest_vec()
{
    println!("sha2_512_256_digest_vec");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "93D5013B2C9AD16AD2B661EC130D376C70958B20BE9CC85D02CA691795EDD39C");
    println!("-------------------------------");
}

fn sha2_512_256_ruminate_c()
{
    println!("sha2_512_256_ruminate_c");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method ruminate_c().";
    hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B5E60B697A54A96D45AAFA99A2B8AB144D6E95DABC67AD63885C7337348BA376");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_c().";
    my_hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "71E97C068FC379DC30E6208109C8166E5DB9DA1C9F3A6DA2270EE804D20554B9");
    println!("-------------------------------");
}

fn sha2_512_256_ruminate()
{
    println!("sha2_512_256_ruminate");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "927E9CC4F2CC0F074B450AA98D2ED3A98296664D4884B2786276E1CB1B6EC146");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate().";
    my_hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "403F0CE6772ECC2C7F0E58BED9C66F95C77CBB4620E1DBB81E70983D156DECC5");
    println!("-------------------------------");
}

fn sha2_512_256_ruminate_str()
{
    println!("sha2_512_256_ruminate_str");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "EBD1A2D79706B299B2E54F5573154CADF5D2FB18D1694B9F664F543D83EF3CA8");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "862323DFE80E5942C29EF8B32FCE0D17B6F3D2059EBF240B283D435ECDEA2FF6");
    println!("-------------------------------");
}

fn sha2_512_256_ruminate_string()
{
    println!("sha2_512_256_ruminate_string");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "8A0EF3F60607AF706173D25484B925F80E4C802624F936A017150C842F27C050");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "8E4BCD2DCD9C69643D3852FBF50F493EF9F1C80C361CD703A5EF9BB41729F076");
    println!("-------------------------------");
}

fn sha2_512_256_ruminate_array()
{
    println!("sha2_512_256_ruminate_array");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BB278FEAE8686D62A45B559FF031AF2143E0B88ED5D20B91C49C0F51013AFF22");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "677D8FBAFC9D1194AEF175492B154BF4B3AAD5198B12C0BE608586D660276DEC");
    println!("-------------------------------");
}

fn sha2_512_256_ruminate_vec()
{
    println!("sha2_512_256_ruminate_vec");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9C32D6722A1F9E22201475FC35C370C919AED2D6849398CE38D0CE1DCC2FCBF6");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "48C7317951B3E2B8DED2BE7F8A5CD2E4D97C4CA5B0F234EB13DF4477D1C53D15");
    println!("-------------------------------");
}

fn sha2_512_256_get_hash_value()
{
    println!("sha2_512_256_get_hash_value");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 32];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[50, EA, 83, BF, 41, 5D, 1C, C0, 15, 6C, BF, 90, 5B, AC, BD, 72, A3, BD, 62, 1B, 94, 3A, 64, 64, 13, 05, CF, 17, 43, 52, CF, AD]");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 32];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[36, 44, 2C, AC, AF, 14, EC, F0, E4, B0, 44, 0D, 1A, AD, 3A, 05, 72, 56, BC, 18, 7B, EF, BF, E7, 4B, B5, 50, 59, AB, 61, 06, 1D]");
    println!("-------------------------------");
}

fn sha2_512_256_get_hash_value_in_string()
{
    println!("sha2_512_256_get_hash_value_in_string");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "F3E8E24304CD04DBE509FE47FFA84DA4CF15E70EEFD447F34A069047735014DC");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "6A4EAEC0A428F07489AEBE4E84A86448DC3CF5B5F34C76517BC0AB75D96CB6C4");
    println!("-------------------------------");
}

fn sha2_512_256_get_hash_value_in_array()
{
    println!("sha2_512_256_get_hash_value_in_array");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[7770814665222F53, FAF871C4D20657F0, 4E3F488853C5C485, CDCFE5F1EB447C2F]");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[A92BE876D695E342, 9557B38AC53D9EB5, F6C467800206D9F9, A7F3F3A7B211E98B]");
    println!("-------------------------------");
}

fn sha2_512_256_get_hash_value_in_vec()
{
    println!("sha2_512_256_get_hash_value_in_vec");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[80A8B6995518FCAE, 88552E1A484EDBE2, 0D97F5D05378D628, 5B7CE15DDBCA6AFA]");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[5A084FC9248A2D4C, C6481CB9C8AD0EE5, 2905FB99FC1D7A43, FE66770474673D2C]");
    println!("-------------------------------");
}

fn sha2_512_256_put_hash_value_in_array()
{
    println!("sha2_512_256_put_hash_value_in_array");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 4];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[9BEF237372571C24, 77A1E2AFFDC98530, A0B9D10323B70681, 436DAE1631785347]");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 4];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[A4EBACCED2E44AAE, BC34C610998E7AB3, 0AB9B0536A150D76, 13C279370C829D2B]");
    println!("-------------------------------");
}

fn sha2_512_256_tangle()
{
    println!("sha2_512_256_tangle");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[FC36648637962C38, BDFBBAE5DEA75E0E, D72827D56EB79EF9, 4969BAA99DB0E42B]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[96CA6859E014C355, 6BBED0E8DA26FFAD, A4F89477C93C9E8C, 806148BDB037AE26]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[11F5369ABC9E3B5D, D3D869131E697AB2, 1899C8D791BB09FC, 0C6CE82AE3B9D583]");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[9771ACFA1FFE9B55, BF7CF746370F01E7, D68B291C1C3EEB8C, 5E8D5A2DBC792186]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[B4C1735DDC8677A6, 6AF607FE0979BF92, BFD34066C9E1317F, B51988A069D20E75]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array()), "[F12B54C8E3F7F9AB, 3EAD06A674A59791, CF3237564DCBF985, EA8A45DFBFD4B2C9]");
    println!("-------------------------------");
}

fn sha2_512_256_fmt_for_to_string()
{
    println!("sha2_512_256_fmt_for_to_string");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "5ED309022841125DE856B25C56A741166872A1D681DF5C69F84AD8B2F30E6DD8");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "E3E7A8D729A8B1187E62DAFA0B0875EEAFA850A07BC4FA7FC1ECDDDB13174875");
    println!("-------------------------------");
}

fn sha2_512_256_fmt_for_println()
{
    println!("sha2_512_256_fmt_for_println");
    // Example for SHA2_512_256
    use cryptocol::hash::SHA2_512_256;
    let mut hash = SHA2_512_256::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "660F8CA5DDC61C43BCEBAB6B8FFD4081F9015CE9A7800BFE29B5100709C3E232");

    // Example for SHA2_512_256_Expanded
    use cryptocol::hash::SHA2_512_256_Expanded;
    type MySHA2 = SHA2_512_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "E7743D83A0FDECCBC3244147451CC79ADBFCFD455810B8CA3A10D54CCA368F71");
    println!("-------------------------------");
}

fn hash_sha2_512_t_256_main()
{
    sha2_512_t_256_quick_start();
    sha2_512_t_256_new();
    sha2_512_t_256_new_with_seed_text();
    sha2_512_t_256_digest_c();
    sha2_512_t_256_digest();
    sha2_512_t_256_digest_str();
    sha2_512_t_256_digest_string();
    sha2_512_t_256_digest_array();
    sha2_512_t_256_digest_vec();
    sha2_512_t_256_ruminate_c();
    sha2_512_t_256_ruminate();
    sha2_512_t_256_ruminate_str();
    sha2_512_t_256_ruminate_string();
    sha2_512_t_256_ruminate_array();
    sha2_512_t_256_ruminate_vec();
    sha2_512_t_256_get_hash_value();
    sha2_512_t_256_get_hash_value_in_string();
    sha2_512_t_256_get_hash_value_in_array();
    sha2_512_t_256_get_hash_value_in_array_tm();
    sha2_512_t_256_get_hash_value_in_vec();
    sha2_512_t_256_put_hash_value_in_array();
    sha2_512_t_256_tangle();
    sha2_512_t_256_fmt_for_to_string();
    sha2_512_t_256_fmt_for_println();
}

fn sha2_512_t_256_quick_start()
{
    println!("sha2_512_t_256_quick_start");
    use std::string::*;
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "C672B8D1EF56ED28AB87C3622C5114069BDD3AD7B8F9737498D0C01ECEF0967A");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "65A992AD19967492B5780D76A4733AF553F796F688B79102D01EC7FDE5590CAB");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "E4AF36E824AFDB9E42291983AFA292B894DED2CCAFCCF53346B223FCA846694D");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4E730BDADF49EC9F3E920F72EAD3AC8D09B459900BE4F6E27848652632277205");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "AE0EAB6824897F575FCC051DBC2D1AA7F7BF0DB2C80172F639CE20B3B498C9D5");

    txt = "This algorithm SHA-2/512/256 is being tested with this message, the length of which is one hundred twelve bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "7876C6F1285C4B6EC6A2F4A76BBF81815B470536F3A38B7028AA88A3C5C31651");

    txt = "This algorithm SHA-2/512/256 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "6FCE377EA6116BEAC9C11606C59A5D034C8C6EF5A1920B783A9097E07BE36D31");

    txt = "This algorithm SHA-2/512/256 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "63FD06E11EF67F0F5EF598C3B2F2E221D5557AD1EEA46156D1B657F1EDF08D5D");
    println!("-------------------------------");
}

fn sha2_512_t_256_new()
{
    println!("sha2_512_t_256_new");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let hash = SHA2_512_t_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "B80E7C569545AF48629EF11E2E14B8204F74747C4F949C6D60FEB4CC233775A7");
    println!("-------------------------------");
}

fn sha2_512_t_256_new_with_seed_text()
{
    println!("sha2_512_t_256_new_with_seed_text");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let hash = SHA2_512_t_256::new_with_seed_text("샤-");
    // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6E231779CE7B233F74077E896D4ABCCA8B31054CB94168164E08BD8F31764DCB");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let my_hash = MySHA2::new_with_seed_text("샤-");
    // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "A15939C18C313184EA37451948F708F5C7B1FBE11E40F8795EF6BF52DB4EC9E9");
    println!("-------------------------------");
}

fn sha2_512_t_256_digest_c()
{
    println!("sha2_512_t_256_digest_c");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method digest_c().";
    hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "AE67F5B190BB09DC615859EC2D11736DA6CBE00340EE39396FE76257238E3AF1");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<0x123456789abcdef0, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_c().";
    my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "D9BE41EF1B7AFDCF7E3E8256661ACD436E3D0811FD433D5A6BF48823F2A004B4");
    println!("-------------------------------");
}

fn sha2_512_t_256_digest()
{
    println!("sha2_512_t_256_digest");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    my_hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "645C53583A01ABF44F279BEC2CC07AB072B57AA319962B524C73435DBE564CEF");
    println!("-------------------------------");
}

fn sha2_512_t_256_digest_str()
{
    println!("sha2_512_t_256_digest_str");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D0ED13389E431C8D74FE6E8DB5B6194682874B52E800524136E35D7E9CFA496B");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "2ABEF10487ECC51EA8953654E972C7C57817D674B12B89E175E569169F43ED9B");
    println!("-------------------------------");
}

fn sha2_512_t_256_digest_string()
{
    println!("sha2_512_t_256_digest_string");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "2824B79B5D8A1D02454612B72B9CD9544D0DF8E126E7A01E55AC479B0903297C");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "B9D855F972D884C200B5EFECB105B115065AC58540099777A84766623BF87C15");
    println!("-------------------------------");
}

fn sha2_512_t_256_digest_array()
{
    println!("sha2_512_t_256_digest_array");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "4AA24C35A21F9D0552E0C3A69A5A59EFE1936FD361ABA1C6E8F6DA22FC39D236");
    println!("-------------------------------");
}

fn sha2_512_t_256_digest_vec()
{
    println!("sha2_512_t_256_digest_vec");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "4AA24C35A21F9D0552E0C3A69A5A59EFE1936FD361ABA1C6E8F6DA22FC39D236");
    println!("-------------------------------");
}

fn sha2_512_t_256_ruminate_c()
{
    println!("sha2_512_t_256_ruminate_c");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method ruminate_c().";
    hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "71D8FB0BC160A3EAA18ED54D48EC54A2FBA4364D4592917CEB8846CAB1492DB6");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_c().";
    my_hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "A5098CDF4CAFEC47765E2D87557587D50CFA802385C39B3596A816B863C45F82");
    println!("-------------------------------");
}

fn sha2_512_t_256_ruminate()
{
    println!("sha2_512_t_256_ruminate");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "EBA9C4DE950CE07EDB662147C3246779660F03607D27493A0D62ECC6282C4501");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate().";
    my_hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "B60D75418A979C6B4E444B755D535257969C5FFC465FA84988026219FC7BD8B7");
    println!("-------------------------------");
}

fn sha2_512_t_256_ruminate_str()
{
    println!("sha2_512_t_256_ruminate_str");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D2ACE4176F3EBD2F5786EFD459D72AD44D24425D05494A8FFEFCA75BAB007FA6");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C5FE6148F6177C5208C8992D2ED20C3016681289ACF5B161D0AD95FB5C4CE5EA");
    println!("-------------------------------");
}

fn sha2_512_t_256_ruminate_string()
{
    println!("sha2_512_t_256_ruminate_string");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7E93544875E40C8F25DDD93AEC9A447C124B22C3DDCDB7479FAD6C144FFC74B2");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "8391EA69564967DF827872115E35A98DFEFF72894F6497C369D83C25C8C50E2E");
    println!("-------------------------------");
}

fn sha2_512_t_256_ruminate_array()
{
    println!("sha2_512_t_256_ruminate_array");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "5C8D4F9C47C99BD322E44AA2B6F265D7A788B8898F072E9E998122EB3DE256F9");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "1A86BD0B53A9CD64FB43CF6BD82107782210A3F5FEC34CAF23B33D51A3B66011");
    println!("-------------------------------");
}

fn sha2_512_t_256_ruminate_vec()
{
    println!("sha2_512_t_256_ruminate_vec");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "6A999B22F62122B781705BBEB635E0DFD6F922FB2B0921F912ACA585B618D7F0");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "2396C1C598B43E445261971C74170745DDB2FD0527684545FFB9818D1D0057AD");
    println!("-------------------------------");
}

fn sha2_512_t_256_get_hash_value()
{
    println!("sha2_512_t_256_get_hash_value");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 32];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[50, EA, 83, BF, 41, 5D, 1C, C0, 15, 6C, BF, 90, 5B, AC, BD, 72, A3, BD, 62, 1B, 94, 3A, 64, 64, 13, 05, CF, 17, 43, 52, CF, AD]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 32];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[D1, 15, 58, 28, FA, A6, 27, F9, 7E, DE, D0, 98, 74, C0, A1, DB, FA, 5E, C0, E9, A9, 98, 35, DD, B8, 00, DC, B4, 28, 79, A9, D3]");
    println!("-------------------------------");
}

fn sha2_512_t_256_get_hash_value_in_string()
{
    println!("sha2_512_t_256_get_hash_value_in_string");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "F3E8E24304CD04DBE509FE47FFA84DA4CF15E70EEFD447F34A069047735014DC");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "9FF616CE58C7CD8988DD1AF54F9AB5E8674ADF5037A7B059AF7B608023D44FBC");
    println!("-------------------------------");
}

fn sha2_512_t_256_get_hash_value_in_array()
{
    println!("sha2_512_t_256_get_hash_value_in_array");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let h = hash.get_hash_value_in_array();
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    assert_eq!(format!("{:016X?}", h), "[7770814665222F53, FAF871C4D20657F0, 4E3F488853C5C485, CDCFE5F1EB447C2F, 14F3BFD7D115FE93, 308218F3657D3CE6, 5D68300E49B0BE02, 9F8286AC65BAC220]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    let h = my_hash.get_hash_value_in_array();
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    assert_eq!(format!("{:016X?}", h), "[74688674B8E8B4BA, 439E1BEC604C9A30, C3E0398F8D52D970, 44B615CD387A9826, AE64BC4B33F5B6B3, 8D7A9CE85CB18255, E30515AA6BC2C25F, 2BA7DE436CE08812]");
    println!("-------------------------------");
}

fn sha2_512_t_256_get_hash_value_in_array_tm()
{
    println!("sha2_512_t_256_get_hash_value_in_array_tm");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method get_hash_value_in_array_tm().";
    hash.digest_str(txt);
    let h: [u64; 4] = hash.get_hash_value_in_array_tm();
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    assert_eq!(format!("{:016X?}", h), "[B2CB9E8954581373, CB03D5E9B4A232D3, B6A92CB91A33C2B6, A78A5A9914FFAAFD]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array_tm().";
    my_hash.digest_str(txt);
    let h: [u64; 4] = hash.get_hash_value_in_array_tm();
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    assert_eq!(format!("{:016X?}", h), "[B2CB9E8954581373, CB03D5E9B4A232D3, B6A92CB91A33C2B6, A78A5A9914FFAAFD]");
    println!("-------------------------------");
}

fn sha2_512_t_256_get_hash_value_in_vec()
{
    println!("sha2_512_t_256_get_hash_value_in_vec");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[80A8B6995518FCAE, 88552E1A484EDBE2, 0D97F5D05378D628, 5B7CE15DDBCA6AFA]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[CAE7273F660C1371, F0A69EBDB143A63D, 37701C05D8CAA659, 76C307D312210B47]");
    println!("-------------------------------");
}

fn sha2_512_t_256_put_hash_value_in_array()
{
    println!("sha2_512_t_256_put_hash_value_in_array");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 4];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[9BEF237372571C24, 77A1E2AFFDC98530, A0B9D10323B70681, 436DAE1631785347]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u64; 4];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    assert_eq!(format!("{:016X?}", hash_code), "[DFE2E754EF11C0D9, 01EAC6F6FF8C0BDB, FF5A2F28DA6C75FA, E0A9B70B2498F0AC]");
    println!("-------------------------------");
}

fn sha2_512_t_256_tangle()
{
    println!("sha2_512_t_256_tangle");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array_tm::<u64, 4>());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>()), "[FC36648637962C38, BDFBBAE5DEA75E0E, D72827D56EB79EF9, 4969BAA99DB0E42B]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>()), "[96CA6859E014C355, 6BBED0E8DA26FFAD, A4F89477C93C9E8C, 806148BDB037AE26]");
    hash.tangle(1);
    println!("Hash =\t{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>()), "[11F5369ABC9E3B5D, D3D869131E697AB2, 1899C8D791BB09FC, 0C6CE82AE3B9D583]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array_tm::<u64, 4>());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>()), "[C60A42A16859F8B8, 7EAB94538B024642, 654DD7795DDDD39B, 12E1A03748AEFFF3]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>()), "[05A82162DE47FEE5, 4B7C2320AF525665, 0D9A9FC79B16B8E6, B51D2D5242BADECD]");
    my_hash.tangle(1);
    println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>()), "[BC74B5902DD2AB00, 680C9FE85FED5E60, 4FAAF51214292837, B9292AFDBF94B64E]");
    println!("-------------------------------");
}

fn sha2_512_t_256_fmt_for_to_string()
{
    println!("sha2_512_t_256_fmt_for_to_string");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "5ED309022841125DE856B25C56A741166872A1D681DF5C69F84AD8B2F30E6DD8");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "377687DECF57B2340B282130B55C74C349376F8727BECA86C904673CD8CD50A7");
    println!("-------------------------------");
}

fn sha2_512_t_256_fmt_for_println()
{
    println!("sha2_512_t_256_fmt_for_println");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "660F8CA5DDC61C43BCEBAB6B8FFD4081F9015CE9A7800BFE29B5100709C3E232");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C44370416B0925577339DECCC8529A68D29D4E79083658F260D4219DA6C2B0D1");
    println!("-------------------------------");
}

fn hash_sha2_512_t_224_main()
{
    sha2_512_t_224_quick_start();
    sha2_512_t_224_new();
    sha2_512_t_224_new_with_seed_text();
    sha2_512_t_224_digest_c();
    sha2_512_t_224_digest();
    sha2_512_t_224_digest_str();
    sha2_512_t_224_digest_string();
    sha2_512_t_224_digest_array();
    sha2_512_t_224_digest_vec();
    sha2_512_t_224_ruminate_c();
    sha2_512_t_224_ruminate();
    sha2_512_t_224_ruminate_str();
    sha2_512_t_224_ruminate_string();
    sha2_512_t_224_ruminate_array();
    sha2_512_t_224_ruminate_vec();
    sha2_512_t_224_get_hash_value();
    sha2_512_t_224_get_hash_value_in_string();
    sha2_512_t_224_get_hash_value_in_array();
    sha2_512_t_224_get_hash_value_in_array_tm();
    sha2_512_t_224_get_hash_value_in_vec();
    sha2_512_t_224_put_hash_value_in_array();
    sha2_512_t_224_tangle();
    sha2_512_t_224_fmt_for_to_string();
    sha2_512_t_224_fmt_for_println();
}

fn sha2_512_t_224_quick_start()
{
    println!("sha2_512_t_224_quick_start");
    use std::string::*;
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "6ED0DD02806FA89E25DE060C19D3AC86CABB87D6A0DDD05C333B84F4");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "1DEF1E6A5344538A07A3C93A3A765FA1D2859A576947791A9047C3E6");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "021B7E0CFE3FBD598CF0366464AEB4C93A900BBA1DF8CADB5F611345");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "1E6EEBF17E8B2B1D2A41B14D9813561E44814E35F01119ED7BA3E19F");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "5251D628FE99DA19238D277DF9AC03382249FF3830AD764EF0A68CDA");

    txt = "This algorithm SHA-2/512/224 is being tested with this message, the length of which is one hundred twelve bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "225B3D39D9B91705E7C08DBBF66E5F34E88554685C78AF2535FD3CE2");

    txt = "This algorithm SHA-2/512/224 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "3DD5D6503AFE8247B37AFD72DFD56718E6CA70D0B425739928885D0F");

    txt = "This algorithm SHA-2/512/224 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D709EC6C2CAA1DAC61B0121675C3B131C23209F9E9ABC60392D99F52");
    println!("-------------------------------");
}

fn sha2_512_t_224_new()
{
    println!("sha2_512_t_224_new");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let hash = SHA2_512_t_224::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "8C3D37C819544DA273E1996689DCD4D61DFAB7AE32FF9C82679DD514");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let my_hash = MySHA2::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "6053A0C18224AF65E6633DEA9B125B74309B64519F70586FF009DFF5");
    println!("-------------------------------");
}

fn sha2_512_t_224_new_with_seed_text()
{
    println!("sha2_512_t_224_new_with_seed_text");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let hash = SHA2_512_t_224::new_with_seed_text("샤-");
    // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "6E231779CE7B233F74077E896D4ABCCA8B31054CB94168164E08BD8F");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let my_hash = MySHA2::new_with_seed_text("샤-");
    // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "A15939C18C313184EA37451948F708F5C7B1FBE11E40F8795EF6BF52");
    println!("-------------------------------");
}

fn sha2_512_t_224_digest_c()
{
    println!("sha2_512_t_224_digest_c");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method digest_c().";
    hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E7B8A450F1F3E90B361BED00083D6E14A90C2A074C71038D0743E384");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<0x123456789abcdef0, 160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_c().";
    my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "9B38B1C0434F66DB99A76273D167237ABC3BF8BC96F91DF051A3E31B");
    println!("-------------------------------");
}

fn sha2_512_t_224_digest()
{
    println!("sha2_512_t_224_digest");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "2269C5A3791E72D00337D9EDDE9BA9568539F4E131B7DB7555545633");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "1DCBF56DC6F3387734139CC5CA14FAC05DF67CD4B14AE86E474F421C");
    println!("-------------------------------");
}

fn sha2_512_t_224_digest_str()
{
    println!("sha2_512_t_224_digest_str");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "17E80E466E706474DB2C9E39691150805AC536319125AFB1E436BE8F");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "44C91FB5B6352E89DF5B5230A004B8594FC7B7AF6F61D3E332C4AC01");
    println!("-------------------------------");
}

fn sha2_512_t_224_digest_string()
{
    println!("sha2_512_t_224_digest_string");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E1423096CED4DC8D9522C75C8BBB12B59A4510093CFA4FD480D270FD");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "092A7203B3BD5C72852B0507989257577808C453C2C7F915BAD1CF5C");
    println!("-------------------------------");
}

fn sha2_512_t_224_digest_array()
{
    println!("sha2_512_t_224_digest_array");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "3F600A922240910231ACA350DEDD49BD875936BE5AAB8A034D09334B");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "504B348485EC8CF96E630DFC90D75DC1543A3A2B3B895A0261CAF0CE");
    println!("-------------------------------");
}

fn sha2_512_t_224_digest_vec()
{
    println!("sha2_512_t_224_digest_vec");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "3F600A922240910231ACA350DEDD49BD875936BE5AAB8A034D09334B");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "504B348485EC8CF96E630DFC90D75DC1543A3A2B3B895A0261CAF0CE");
    println!("-------------------------------");
}

fn sha2_512_t_224_ruminate_c()
{
    println!("sha2_512_t_224_ruminate_c");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method ruminate_c().";
    hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "4DA0CB3085D73CA7459E326D51349B5A7C065A270347558DA7FB3784");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_c().";
    my_hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "4AAE39BF545F153044E1A9D10CDAA98F56D048619C406770709FB015");
    println!("-------------------------------");
}

fn sha2_512_t_224_ruminate()
{
    println!("sha2_512_t_224_ruminate");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A3280359EA2135FE3E2667724FCA6996A47B362544FA60FD59D95DBF");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate().";
    my_hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "E587737E4BC3E1D859AA7FDDD90D3E769158173B7A22FA4BC76E47BA");
    println!("-------------------------------");
}

fn sha2_512_t_224_ruminate_str()
{
    println!("sha2_512_t_224_ruminate_str");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7FB2230906052932F044352E65F590C416C09C3A7290EF3BC39635EF");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "9FD72DC6516F3E01FAE29244B70D501F6AE73B25CB462F816F01C6F0");
    println!("-------------------------------");
}

fn sha2_512_t_224_ruminate_string()
{
    println!("sha2_512_t_224_ruminate_string");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C6887398FA4DA83CD5039DC2764BB363B65F1C557006D627F95B5392");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "9B1BFD42888B1655735F6E0F1122E25D33F8DBF6E65D54D4EA0884A3");
    println!("-------------------------------");
}

fn sha2_512_t_224_ruminate_array()
{
    println!("sha2_512_t_224_ruminate_array");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "987227F7EC37FCF30A83BE661BF7018616CE5B9C9553AA7892C738D3");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "BDF3FB6FCEAEB9A75FAA42CE759019A60FEB23E40DBC676F9BE36DE4");
    println!("-------------------------------");
}

fn sha2_512_t_224_ruminate_vec()
{
    println!("sha2_512_t_224_ruminate_vec");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "ACA9B9D5B327FFE4140F131642F92DCBDFD678FA5F7A42536D27BAF8");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "1AAC9EE0F6B45991CC58691DF19079E99422925DC600789343BEAA24");
    println!("-------------------------------");
}

fn sha2_512_t_224_get_hash_value()
{
    println!("sha2_512_t_224_get_hash_value");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 28];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[8B, 40, A3, E7, 02, A8, 18, 25, 12, 2C, C8, 55, 07, 4F, 5B, 0F, 73, BD, 30, 42, 5F, 3A, A9, 55, 92, 28, 27, 9E]");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 28];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[A7, A9, A3, 52, EB, E6, 06, 3E, 80, F1, 7E, 62, 27, 6B, AB, F6, 5C, 21, 8E, 56, B7, 2A, 04, 4C, 7D, 11, C5, 40]");
    println!("-------------------------------");
}

fn sha2_512_t_224_get_hash_value_in_string()
{
    println!("sha2_512_t_224_get_hash_value_in_string");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "412823A4ED7BBA8C052F3C9B218A9847CDD341818E773F5593011135");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "1E5E301B37EC1336D5EB0D9A4AE18833F418C93F8ADBD87BE6817922");
   println!("-------------------------------");
}

fn sha2_512_t_224_get_hash_value_in_array()
{
    println!("sha2_512_t_224_get_hash_value_in_array");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let h = hash.get_hash_value_in_array();
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    assert_eq!(format!("{:016X?}", h), "[5D4F9158C3082FB5, DA01BFDDDDC44C7B, 8E72845FCC6EF467, EBCF28927DFDDD35, E9ACDB58F2E01FAE, E62CF6411757DAD2, 9CC9EF5BD989F543, 055C3169B2B23276]");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    let h = my_hash.get_hash_value_in_array();
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    assert_eq!(format!("{:016X?}", h), "[66E3FC505AE3D81E, 09B33F26C109A6D8, 0965E49C9CEC98D8, BEDFF6D3ACA89A9C, 95C0831C1F650A37, BEAAC6B754481D81, D4EEECD4ADE7BF93, FFA8A6B1808E60DD]");
    println!("-------------------------------");
}

fn sha2_512_t_224_get_hash_value_in_array_tm()
{
    println!("sha2_512_t_256_get_hash_value_in_array_tm");
    // Example for SHA2_512_t_256
    use cryptocol::hash::SHA2_512_t_256;
    let mut hash = SHA2_512_t_256::new();
    let txt = "This is an example of the method get_hash_value_in_array_tm().";
    hash.digest_str(txt);
    let h: [u32; 7] = hash.get_hash_value_in_array_tm();
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, h);
    assert_eq!(format!("{:08X?}", h), "[54581373, B2CB9E89, B4A232D3, CB03D5E9, 1A33C2B6, B6A92CB9, 14FFAAFD]");

    // Example for SHA2_512_t_256_Expanded
    use cryptocol::hash::SHA2_512_t_256_Expanded;
    type MySHA2 = SHA2_512_t_256_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_array_tm().";
    my_hash.digest_str(txt);
    let h: [u32; 7] = my_hash.get_hash_value_in_array_tm();
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, h);
    assert_eq!(format!("{:08X?}", h), "[63D2D0E1, A3378487, 116930CC, 1DD5D525, 47DAE024, 0B502841, AC13B293]");
    println!("-------------------------------");
}

fn sha2_512_t_224_get_hash_value_in_vec()
{
    println!("sha2_512_t_224_get_hash_value_in_vec");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[CDDD34F4216F38F0, CF7779A43F982E1A, 964CE2DBE181F3D3, 95AFDE2500000000]");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[BEBD35E3ECE3EE83, 1A9F9889D1926D37, 08CF548C8A943F0A, 1BFECCAF00000000]");
    println!("-------------------------------");
}

fn sha2_512_t_224_put_hash_value_in_array()
{
    println!("sha2_512_t_224_put_hash_value_in_array");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 7];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[4475114A, BD7D631C, 0A487709, 8B533EBA, 29C83AAF, BD7EB4EF, 77256C3D]");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 7];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[3AAAFB02, A794F5B0, 917078C8, 20548087, 3A7C909F, 4F998CBE, 3A926E6B]");
    println!("-------------------------------");
}

fn sha2_512_t_224_tangle()
{
    println!("sha2_512_t_224_tangle");
    // Example for SHA2_512_t_224
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array_tm::<u32, 7>());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>()), "[72E2E82F, C78389DA, 112F494F, B415B8C4, EF993BFA, EDB5091B, 8C03F067]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>()), "[A7CED549, 2C050740, 9BC2F6E5, EAC6D908, 26148AE9, 966D5E72, ED5DF840]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>()), "[14C24EAE, B39CD243, 8C484722, CB1A03AA, F1F9F55E, 955A27D8, 70A3ED4F]");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array_tm::<u32, 7>());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>()), "[6EF90662, CD08A7EA, 93D0EDFC, 390175A6, 53368038, ADC8BCC8, 11351AB8]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>()), "[F7566CAF, B1039FF1, 722C9B99, 5AA84D67, E6C1182A, 3B4D2DBF, 7F1FA1C8]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>()), "[5B74C46E, F433ACC6, 6A402398, 39126678, 581E67AD, 14A4C823, 4B387049]");
    println!("-------------------------------");
}

fn sha2_512_t_224_fmt_for_to_string()
{
    println!("sha2_512_t_224_fmt_for_to_string");
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "0FFD651E288004466FF247808E1FF5B482AFF547E94C66FF507BF021");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "B130035D26D3BED1F991CB78DFC93F39F8CEF176BC4D7CF8B266027B");
    println!("-------------------------------");
}

fn sha2_512_t_224_fmt_for_println()
{
    println!("sha2_512_t_224_fmt_for_println");
    use cryptocol::hash::SHA2_512_t_224;
    let mut hash = SHA2_512_t_224::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7988DFC3FB4BB8DB449B189C5D906901921C1AC0D60D94376B498795");

    // Example for SHA2_512_t_224_Expanded
    use cryptocol::hash::SHA2_512_t_224_Expanded;
    type MySHA2 = SHA2_512_t_224_Expanded<160>;
    let mut my_hash = MySHA2::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "831E095076EAE29CBC5BD2960D074BAC9E07C9189B9A5FCAE29FD5DB");
    println!("-------------------------------");
}
