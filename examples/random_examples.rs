// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]


pub fn main()
{
    // random_quick_start();
    random_new();
    random_new_with_seeds();
    random_random_u8();
    random_random_u16();
    random_random_u32();
    random_random_u64();
    random_random_u128();
    random_random_usize();
    random_random_uint();
    random_random_under_uint();
    random_random_under_uint_();
    random_random_minmax_uint();
    random_random_minmax_uint_();
    random_random_odd_uint();
    random_random_odd_under_uint();
    random_random_odd_under_uint_();
    random_random_with_msb_set_uint();
    random_random_odd_with_msb_set_uint();
    random_random_prime_using_miller_rabin_uint();
    random_random_prime_with_msb_set_using_miller_rabin_uint();
    random_random_array();
    random_put_random_array();
    random_random_biguint();
    random_random_under_biguint();
    random_random_under_biguint_();
    random_random_odd_biguint();
    random_random_odd_under_biguint();
    random_random_odd_under_biguint_();
    random_random_with_msb_set_biguint();
    random_random_odd_with_msb_set_biguint();
    random_random_prime_using_miller_rabin_biguint();
    random_random_prime_with_msb_set_using_miller_rabin_biguint();
}

fn random_quick_start()
{
    println!("random_quick_start");
    use cryptocol::random::Random;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut rand = Random::new();
    println!("Random number = {}", rand.random_u128());
    println!("Random number = {}", rand.random_u64());
    println!("Random number = {}", rand.random_u32());
    println!("Random number = {}", rand.random_u16());
    println!("Random number = {}", rand.random_u8());

    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random number u64 = {}", num); }

    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random number u16 = {}", num); }

    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }

    println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random number {} => {}", i, num[i]); }

    let mut biguint: U512 = rand.random_biguint();
    println!("Random Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = rand.random_with_msb_set_biguint();
    println!("Random Number: {}", biguint);

    biguint = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_new()
{
    println!("random_new");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random number = {}", rand.random_u128());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    println!("Any number = {}", any.random_u64());

    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    println!("Any number = {}", any_num.random_u32());

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    println!("Any number = {}", any_num_c.random_u16());

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    println!("Any number = {}", any_md4.random_u8());

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    println!("Any number = {}", any_md5.random_usize());

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    println!("Any number = {}", any_sha0.random_uint::<u8>());

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    if let Some(num) = any_sha1.random_under_uint(1234_u16)
        { println!("Any number = {}", num); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    if let Some(num) = any_sha2_256.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any number = {}", num); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    println!("Any odd number = {}", any_sha2_512.random_odd_uint::<u64>());

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut random_sha2_512 = Random_SHA2_512::new();
    println!("Random prime number = {}", random_sha2_512.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    type MyAny = Random_Generic<MyMD4>;
    let mut any_md4_expanded = MyAny::new();
    println!("Any number = {}", any_md4_expanded.random_biguint::<u128, 4>());
    println!("-------------------------------");
}

fn random_new_with_seeds()
{
    println!("random_new_with_seeds");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new_with_seeds(10500872879054459758_u64, 15887751380961987625_u64);
    println!("Random number = {}", rand.random_u8());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new_with_seeds(100_u8, 25_u8);
    println!("Any number = {}", any.random_u16());

    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new_with_seeds(50558_u16, 18782_u16);
    println!("Any number = {}", any_num.random_u32());

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new_with_seeds(458861005_u32, 793621585_u32);
    println!("Any number = {}", any_num_c.random_u64());

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new_with_seeds(106842379157284697258430057945771605458_u128, 156987312156874563258964453432556087625_u128);
    println!("Any number = {}", any_md4.random_usize());

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new_with_seeds(58_usize, 161_usize);
    println!("Any number = {}", any_md5.random_u128());

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new_with_seeds(0_u8, 125_u8);
    println!("Any prime number = {}", any_sha0.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new_with_seeds(18782_u16, 50558_u16);
    println!("Any number = {}", any_sha1.random_uint::<u8>());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new_with_seeds(610458805_u32, 215793685_u32);
    if let Some(num) = any_sha2_256.random_under_uint(1234_u16)
        { println!("Any number = {}", num); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new_with_seeds(2879054410500759758_u64, 15887876257513809619_u64);
    if let Some(num) = any_sha2_512.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any number = {}", num); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut random_sha2_512 = Random_SHA2_512::new_with_seeds(156987312156874563258964453432556087625_u128, 106842379157284697258430057945771605458_u128);
    println!("Random odd number = {}", random_sha2_512.random_odd_uint::<u128>());

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    type MyAny = Random_Generic<MyMD5>;
    let mut any_md5_expanded = MyAny::new_with_seeds(6445332556087625_u64, 43057945771605458_u64);
    println!("Any number = {}", any_md5_expanded.random_biguint::<u64, 8>());
    println!("-------------------------------");
}

fn random_random_u8()
{
    println!("random_random_u8");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u8()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u8()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_u8()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_u8()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_u8()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_u8()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_u8()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_u8()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_u8()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_u8()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Any_SHA2_512) = {}", i, rand_sha2_512.random_u8()); }

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::SHA0_Expanded;
    type MySHA0 = SHA0_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    type MyAny = Random_Generic<MySHA0>;
    let mut any_sha0_expanded = MyAny::new_with_seeds(6445332556087625_u64, 43057945771605458_u64);
    for i in 0..10
        { println!(" {} Any number (Any_SHA2_512) = {}", i, any_sha0_expanded.random_u8()); }
    println!("-------------------------------");
}

fn random_random_u16()
{
    println!("random_random_u16");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u16()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u16()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_u16()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_u16()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_u16()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_u16()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_u16()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_u16()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_u16()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_u16()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand_sha2_512.random_u16()); }

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    type MyAny = Random_Generic<MySHA1>;
    let mut any_sha1_expanded = MyAny::new_with_seeds(6445332556087625_u64, 43057945771605458_u64);
    for i in 0..10
        { println!("{} Any number (Random_Generic) = {}", i, any_sha1_expanded.random_u16()); }
    println!("-------------------------------");
}

fn random_random_u32()
{
    println!("random_random_u32");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u32()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u32()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_u32()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_u32()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_u32()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_u32()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_u32()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_u32()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_u32()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_u32()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand_sha2_512.random_u32()); }

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::SHA2_256_Expanded;
    type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    type MyAny = Random_Generic<MySHA2>;
    let mut any_sha2_expanded = MyAny::new_with_seeds(6445332556087625_u64, 43057945771605458_u64);
    for i in 0..10
        { println!("{} Any number (Random_Generic) = {}", i, any_sha2_expanded.random_u32()); }
    println!("-------------------------------");
}

fn random_random_u64()
{
    println!("random_random_u64");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u64()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u64()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_u64()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_u64()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_u64()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_u64()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_u64()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_u64()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_u64()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_u64()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand_sha2_512.random_u64()); }

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    type MyAny = Random_Generic<MySHA2>;
    let mut any_sha2_expanded = MyAny::new();
    for i in 0..10
        { println!("{} Any number (Random_Generic) = {}", i, any_sha2_expanded.random_u64()); }
    println!("-------------------------------");
}

fn random_random_u128()
{
    println!("random_random_u128");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u128()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u128()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_u128()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_u128()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_u128()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_u128()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_u128()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_u128()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_u128()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_u128()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand_sha2_512.random_u128()); }

    // Example for Random_Generic
    use cryptocol::random::{ Random_Generic, AnyNumber_Engine_C_Generic };
    type MyAnyNumberEngineCExpanded = AnyNumber_Engine_C_Generic<1234567890987654321_u64, 987654321_u64>;
    type MyAny = Random_Generic<MyAnyNumberEngineCExpanded>;
    let mut any_num_c_expanded = MyAny::new();
    for i in 0..10
        { println!("{} Any number (Random_Generic) = {}", i, any_num_c_expanded.random_u128()); }
    println!("-------------------------------");
}

fn random_random_usize()
{
    println!("random_random_usize");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_usize()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_usize()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_usize()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_usize()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_usize()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_usize()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_usize()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_usize()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_usize()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_usize()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand_sha2_512.random_usize()); }

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1212_1212_1212_1212, 0x3434_3434_3434_3434, 0x5454_5454_5454_5454, 0x7878_7878_7878_7878, 0x9A9A_9A9A_9A9A_9A9A, 0xbcbc_bcbc_bcbc_bcbc, 0xdede_dede_dede_dede, 0xf0f0_f0f0_f0f0_f0f0, 240>;
    type MyAny = Random_Generic<MySHA2>;
    let mut any_sha2_expanded1 = MyAny::new();
    for i in 0..10
        { println!("{} Any number (Random_Generic) = {}", i, any_sha2_expanded1.random_usize()); }
    println!("-------------------------------");
}

fn random_random_uint()
{
    println!("random_random_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_uint::<u8>()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_uint::<u16>()); }
  
    // Example for Any_Num
    use cryptocol::random::Any_Num;
    let mut any_num = Any_Num::new();
    for i in 0..10
        { println!("{} Any number (Any_Num) = {}", i, any_num.random_uint::<u32>()); }

    // Example for Any_Num_C
    use cryptocol::random::Any_Num_C;
    let mut any_num_c = Any_Num_C::new();
    for i in 0..10
        { println!("{} Any number (Any_Num_C) = {}", i, any_num_c.random_uint::<u64>()); }

    // Example for Any_MD4
    use cryptocol::random::Any_MD4;
    let mut any_md4 = Any_MD4::new();
    for i in 0..10
        { println!("{} Any number (Any_MD4) = {}", i, any_md4.random_uint::<u128>()); }

    // Example for Any_MD5
    use cryptocol::random::Any_MD5;
    let mut any_md5 = Any_MD5::new();
    for i in 0..10
        { println!("{} Any number (Any_MD5) = {}", i, any_md5.random_uint::<usize>()); }

    // Example for Any_SHA0
    use cryptocol::random::Any_SHA0;
    let mut any_sha0 = Any_SHA0::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA0) = {}", i, any_sha0.random_uint::<u8>()); }

    // Example for Any_SHA1
    use cryptocol::random::Any_SHA1;
    let mut any_sha1 = Any_SHA1::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA1) = {}", i, any_sha1.random_uint::<u16>()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any_sha2_256 = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any_sha2_256.random_uint::<u32>()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any_sha2_512 = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any_sha2_512.random_uint::<u64>()); }

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand_sha2_512 = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand_sha2_512.random_uint::<u128>()); }

    // Example for Random_Generic
    use cryptocol::random::Random_Generic;
    use cryptocol::hash::SHA2_512_Expanded;
    type MySHA2 = SHA2_512_Expanded<0x1212_1212_1212_1212, 0x3434_3434_3434_3434, 0x5454_5454_5454_5454, 0x7878_7878_7878_7878, 0x9A9A_9A9A_9A9A_9A9A, 0xbcbc_bcbc_bcbc_bcbc, 0xdede_dede_dede_dede, 0xf0f0_f0f0_f0f0_f0f0, 240>;
    type MyAny = Random_Generic<MySHA2>;
    let mut any_sha2_expanded2 = MyAny::new();
    for i in 0..10
        { println!("{} Any number (Random_Generic) = {}", i, any_sha2_expanded2.random_uint::<u128>()); }
    println!("-------------------------------");
    panic!("Intentional Stop");
}

fn random_random_under_uint()
{
    println!("random_random_under_uint");
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    if let Some(num) = rand.random_under_uint(12_u8)
        { println!("Random number u8 = {}", num); }
    if let Some(num) = rand.random_under_uint(1234_u16)
        { println!("Random number u16 = {}", num); }
    if let Some(num) = rand.random_under_uint(12345678_u32)
        { println!("Random number u32 = {}", num); }
    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random number u64 = {}", num); }
    if let Some(num) = rand.random_under_uint(12345678901234567890_u128)
        { println!("Random number u128 = {}", num); }
    if let Some(num) = rand.random_under_uint(123456789_usize)
        { println!("Random number usize = {}", num); }
    if let Some(num) = rand.random_under_uint(0_usize)
        { println!("Random number usize = {}", num); }
    else
        { println!("No random unsigned number number under 0!"); }
    println!("-------------------------------");
}

fn random_random_under_uint_()
{
    println!("random_random_under_uint_");
    use cryptocol::random::Any_Num;
    let mut rand = Any_Num::new();

    let num = rand.random_under_uint_(12_u8);
    println!("Random number u8 = {}", num);

    let num = rand.random_under_uint_(1234_u16);
    println!("Random number u16 = {}", num);

    let num = rand.random_under_uint_(12345678_u32);
    println!("Random number u32 = {}", num);

    let num = rand.random_under_uint_(1234567890123456_u64);
    println!("Random number u64 = {}", num);

    let num = rand.random_under_uint_(12345678901234567890_u128);
    println!("Random number u128 = {}", num);

    let num = rand.random_under_uint_::<usize>(123456789_usize);
    println!("Random number usize = {}", num);

    // It will panic.
    // let num = rand.random_under_uint_::<usize>(0_usize);
    // println!("Random number usize = {}", num);
    println!("-------------------------------");
}

fn random_random_minmax_uint()
{
    println!("random_random_minmax_uint");
    use cryptocol::random::Any_MD4;
    let mut rand = Any_MD4::new();
    if let Some(num) = rand.random_minmax_uint(12_u8, 21)
        { println!("Random number u8 = {}", num); }
    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random number u16 = {}", num); }
    if let Some(num) = rand.random_minmax_uint(12345678_u32, 87654321)
        { println!("Random number u32 = {}", num); }
    if let Some(num) = rand.random_minmax_uint(1234567890123456_u64, 6543210987654321)
        { println!("Random number u64 = {}", num); }
    if let Some(num) = rand.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
        { println!("Random number u128 = {}", num); }
    if let Some(num) = rand.random_minmax_uint(123456789_usize, 987654321)
        { println!("Random number usize = {}", num); }
    if let Some(num) = rand.random_minmax_uint(10, 8_usize)
        { println!("Random number usize = {}", num); }
    else
        { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    println!("-------------------------------");
}

fn random_random_minmax_uint_()
{
    println!("random_random_minmax_uint_");
    use cryptocol::random::Any_MD5;
    let mut rand = Any_MD5::new();

    let num = rand.random_minmax_uint_(12_u8, 21);
    println!("Random number u8 = {}", num);

    let num = rand.random_minmax_uint_(1234_u16, 6321);
    println!("Random number u16 = {}", num);

    let num = rand.random_minmax_uint_(12345678_u32, 87654321);
    println!("Random number u32 = {}", num);

    let num = rand.random_minmax_uint_(1234567890123456_u64, 6543210987654321);
    println!("Random number u64 = {}", num);

    let num = rand.random_minmax_uint_(12345678901234567890_u128, 19876543210987654321);
    println!("Random number u128 = {}", num);

    let num = rand.random_minmax_uint_(123456789_usize, 987654321);
    println!("Random number usize = {}", num);

    // It will panic!
    // let num = rand.random_minmax_uint_(10, 8_usize);
    // println!("Random number usize = {}", num);
    println!("-------------------------------");
}

fn random_random_odd_uint()
{
    println!("random_random_odd_uint");
    use cryptocol::random::Any_SHA0;
    let mut rand = Any_SHA0::new();
    println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    println!("-------------------------------");
}

fn random_random_odd_under_uint()
{
    println!("random_random_odd_under_uint");
    use cryptocol::random::Any_SHA1;
    let mut rand = Any_SHA1::new();
    if let Some(num) = rand.random_odd_under_uint(12_u8)
        { println!("Random odd number u8 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678_u32)
        { println!("Random odd number u32 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
        { println!("Random odd number u64 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
        { println!("Random odd number u128 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(123456789_usize)
        { println!("Random odd number usize = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(0_usize)
        { println!("Random odd number usize = {}", num); }
    else
        { println!("No random unsigned number number under 0!"); }
    println!("-------------------------------");
}

fn random_random_odd_under_uint_()
{
    println!("random_random_odd_under_uint_");
    use cryptocol::random::Any_SHA2_256;
    let mut rand = Any_SHA2_256::new();

    let num = rand.random_odd_under_uint_(12_u8);
    println!("Random odd number u8 = {}", num);

    let num = rand.random_odd_under_uint_(1234_u16);
    println!("Random odd number u16 = {}", num);

    let num = rand.random_odd_under_uint_(12345678_u32);
    println!("Random odd number u32 = {}", num);

    let num = rand.random_odd_under_uint_(1234567890123456_u64);
    println!("Random odd number u64 = {}", num);

    let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    println!("Random odd number u128 = {}", num);

    let num = rand.random_odd_under_uint_::<usize>(123456789_usize);
    println!("Random odd number usize = {}", num);

    // It will panic.
    // let num = rand.random_odd_under_uint_::<usize>(0_usize);
    // println!("Random odd number usize = {}", num);
    println!("-------------------------------");
}

fn random_random_with_msb_set_uint()
{
    println!("random_random_with_msb_set_uint");
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    println!("Random 8-bit number u8 = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit number u32 = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit number u64 = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized number usize = {}", rand.random_with_msb_set_uint::<usize>());
    println!("-------------------------------");
}

fn random_random_odd_with_msb_set_uint()
{
    println!("random_random_odd_with_msb_set_uint");
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random 8-bit odd number u8 = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit odd number u32 = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit odd number u64 = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit odd number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized odd number usize = {}", rand.random_with_msb_set_uint::<usize>());
    println!("-------------------------------");
}

fn random_random_prime_using_miller_rabin_uint()
{
    println!("random_random_prime_using_miller_rabin_uint");
    use cryptocol::random::Any;
    let mut rand = Any::new();
    println!("Random prime number u8 = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Random prime number u16 = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Random prime number u32 = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random prime number u128 = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Random prime number usize = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    println!("-------------------------------");
}

fn random_random_prime_with_msb_set_using_miller_rabin_uint()
{
    println!("random_random_prime_with_msb_set_using_miller_rabin_uint");
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random 8-bit prime number u8 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number u16 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number u32 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number u64 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number u128 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    println!("-------------------------------");
}

fn random_random_array()
{
    println!("random_random_array");
    use cryptocol::random::Any;
    let mut rand = Any::new();
    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random number {} => {}", i, num[i]); }
    println!("-------------------------------");
}

fn random_put_random_array()
{
    println!("random_random_array");
    use cryptocol::random::Any_MD4;
    let mut rand = Any_MD4::new();
    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random number {} => {}", i, num[i]); }
    println!("-------------------------------");
}

fn random_random_biguint()
{
    println!("random_random_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_MD5;

    define_utypes_with!(u128);
    let mut rand = Any_MD5::new();
    let biguint: U512 = rand.random_biguint();
    println!("Random Number: {}", biguint);
    println!("-------------------------------");
}

fn random_random_under_biguint()
{
    println!("random_random_under_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA0;

    define_utypes_with!(u64);
    let mut rand = Any_SHA0::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    println!("-------------------------------");
}

fn random_random_under_biguint_()
{
    println!("random_random_under_biguint_");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA1;

    define_utypes_with!(u32);
    let mut rand = Any_SHA1::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_odd_biguint()
{
    println!("random_random_odd_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA2_256;

    define_utypes_with!(u16);
    let mut rand = Any_SHA2_256::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_odd_under_biguint()
{
    println!("random_random_odd_under_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_MD4;

    define_utypes_with!(u128);
    let mut rand = Any_MD4::new();
    let ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    println!("-------------------------------");
}

fn random_random_odd_under_biguint_()
{
    println!("random_random_odd_under_biguint_");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA2_512;

    define_utypes_with!(u8);
    let mut rand = Any_SHA2_512::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_with_msb_set_biguint()
{
    println!("random_random_with_msb_set_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_MD5;

    define_utypes_with!(u64);
    let mut rand = Any_MD5::new();
    let biguint: U512 = rand.random_with_msb_set_biguint();
    println!("Random Number: {}", biguint);
    println!("-------------------------------");
}

fn random_random_odd_with_msb_set_biguint()
{
    println!("random_random_odd_with_msb_set_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any;

    define_utypes_with!(u32);
    let mut rand = Any::new();
    let num:U512 = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random Odd Number = {}", num);
    assert!(num > U512::halfmax());
    assert!(num.is_odd());
    println!("-------------------------------");
}

fn random_random_prime_using_miller_rabin_biguint()
{
    println!("random_random_prime_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Random;

    define_utypes_with!(u16);
    let mut rand = Random::new();
    let num:U512 = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random Prime Number = {}", num);
    assert!(num.is_odd());
    println!("-------------------------------");
}

fn random_random_prime_with_msb_set_using_miller_rabin_biguint()
{
    println!("random_random_prime_with_msb_set_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA1;

    define_utypes_with!(u16);
    let mut rand = Any_SHA1::new();
    let num:U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random Prime Number = {}", num);
    assert!(num.is_odd());
    println!("-------------------------------");
}


/*
use std::ops::*;
use std::fmt::{ Display, Debug };
use rand::{ rngs, RngCore };

use cryptocol::number::SmallUInt;
use cryptocol::random::{ random_Engine, random_Generic };

pub struct OsRng;
{
    hash_code: [u64; 8]
}

impl random_Engine for OsRng
{
    #[inline]
    fn new() -> Self
    {
        Self { hash_code: [0_u64; 8] }
    }

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
    {
        Self::new()
    }

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

fn Hash_OS_Rng_main()
{
    pub type random_OsRng = random_Generic<OsRng>;
    
    let mut r = random_OsRng::new();
    println!("random_OsRng u8 = {}", r.random_u8());
    println!("random_OsRng u16 = {}", r.random_u16());
    println!("random_OsRng u32 = {}", r.random_u32());
    println!("random_OsRng u64 = {}", r.random_u64());
    println!("random_OsRng u128 = {}", r.random_u128());
    println!("random_OsRng under 123456789 = {}", r.random_under_uint_(123456789_u64));
    println!("random_OsRng prime number = {}", r.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("random_OsRng under BigUInt = {}", r.random_biguint::<u64, 8>());
    println!("random_OsRng odd BigUInt = {}", r.random_odd_biguint::<u64, 8>());
    println!("random_OsRng BigUInt prime number = {}", r.random_prime_using_miller_rabin_biguint::<u64, 8>(5));
}
*/
