// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]
/*
use Cryptocol::number::*;
use Cryptocol::define_utypes_with;
use Cryptocol::define_Utypes_with_utypes;
use Cryptocol::define_utypes_with_u128;
use Cryptocol::define_utypes_with_u64;
use Cryptocol::define_utypes_with_u32;
use Cryptocol::define_utypes_with_u16;
use Cryptocol::define_utypes_with_u8;
*/


fn main()
{
    // BigUInt_quick_start___main();
    // BigUInt_constructors___main();
    // BigUInt_random_number___main();
    // BigUInt_get_size___main();
    // BigUInt_get_set_check___main();
    // BigUInt_check_bits___main();
    // BigUInt_comparison_uint___main();
    // BigUInt_comparison_biguint___main();
    BigUInt_arithmatic_operation_uint___main();
    // BigUInt_exponentiation_logarithm_uint___main();
    // BigUInt_arithmatic_operation_biguint___main();
    // BigUInt_exponentiation_logarithm_biguint___main();
    // BigUInt_bit_operation___main();
    // BigUInt_conversion___main();
    // BigUInt_flag_manipulation___main();
}

fn BigUInt_quick_start___main()
{
    // BigUInt_quick_start1();
    // BigUInt_quick_start2();
}

fn BigUInt_quick_start1___main()
{
    println!("BigUInt_quick_start1");
    use std::str::FromStr;
    use Cryptocol::number::*;

    type u1024 = BigUInt::<u128, 8>;

    let a = u1024::from([1;8]);
    let b = u1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
    let c = u1024::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();

    println!("a = {:?}", a);
    assert_eq!(format!("{:?}", a), "BigUInt { number: [1, 1, 1, 1, 1, 1, 1, 1], flag: 0 }");

    println!("a = {}", a.to_string_with_radix(16).unwrap());
    assert_eq!(a.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    println!("b = {:?}", b);
    assert_eq!(format!("{:?}", b), "BigUInt { number: [1, 1, 1, 1, 1, 1, 1, 1], flag: 0 }");

    println!("b = {}", b.to_string_with_radix(16).unwrap());
    assert_eq!(b.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    println!("c = {}", c);
    assert_eq!(c.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");

    let mut d = c.clone() + b.clone();
    println!("c + b = {}", d);
    assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");

    d = c.clone() - b.clone();
    println!("c - b = {}", d);
    assert_eq!(d.to_string(), "179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474110969959963482268385702277221395399967835253147397865027956454455971545930224953627282297099651332502986953874099069742291515547035859317810474001329894911");

    d = c.clone() * b.clone();
    println!("c * b = {}", d);
    assert_eq!(d.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");

    d = b.clone() / c.clone();
    println!("b / c = {}", d);
    assert_eq!(d.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");

    d = b.clone() % c.clone();
    println!("b % c = {}", d);
    assert_eq!(d.to_string(), "13407807929942597099574024998205846128493599681261533778420315443464330801466754909627229853216489898311858516129779051425416284792074901709655664589144577");

    d = b.clone() + 5_u128;
    println!("b + 5 = {}", d);
    assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");

    d = b.clone() - 1_u128;
    println!("b - 1 = {}", d);
    assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");

    d = b.clone() * 42_u128;
    println!("b * 42 = {}", d);
    assert_eq!(d.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");

    d = b.clone() / 5_u128;
    println!("b / 5 = {}", d);
    assert_eq!(d.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");

    let e = b.clone() % 5_u128;
    println!("b % 5 = {}", e);
    assert_eq!(e, 3);
    println!("-------------------------------");
}

fn BigUInt_quick_start2()
{
    println!("BigUInt_quick_start2()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    
    define_utypes_with!(u128);

    let a = u1024::from([1;8]);
    let b = u1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
    let c = U128::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();

    println!("a = {:?}", a);
    assert_eq!(format!("{:?}", a), "BigUInt { number: [1, 1, 1, 1, 1, 1, 1, 1], flag: 0 }");

    println!("a = {}", a.to_string_with_radix(16).unwrap());
    assert_eq!(a.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    println!("b = {:?}", b);
    assert_eq!(format!("{:?}", b), "BigUInt { number: [1, 1, 1, 1, 1, 1, 1, 1], flag: 0 }");

    println!("b = {}", b.to_string_with_radix(16).unwrap());
    assert_eq!(b.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    println!("c = {}", c);
    assert_eq!(c.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");

    let mut d = c.wrapping_add(&b);
    println!("c + b = {}", d);
    assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");

    d = c.wrapping_sub(&b);
    println!("c - b = {}", d);
    assert_eq!(d.to_string(), "179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474110969959963482268385702277221395399967835253147397865027956454455971545930224953627282297099651332502986953874099069742291515547035859317810474001329894911");

    d = c.wrapping_mul(&b);
    println!("c * b = {}", d);
    assert_eq!(d.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");

    d = b.wrapping_div(&c);
    println!("b / c = {}", d);
    assert_eq!(d.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");

    d = b.wrapping_rem(&c);
    println!("b % c = {}", d);
    assert_eq!(d.to_string(), "13407807929942597099574024998205846128493599681261533778420315443464330801466754909627229853216489898311858516129779051425416284792074901709655664589144577");

    d = b.wrapping_add_uint(5_u128);
    println!("b + 5 = {}", d);
    assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");

    d = b.wrapping_sub_uint(1_u128);
    println!("b - 1 = {}", d);
    assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");

    d = b.wrapping_mul_uint(42_u128);
    println!("b * 42 = {}", d);
    assert_eq!(d.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");

    d = b.wrapping_div_uint(5_u128);
    println!("b / 5 = {}", d);
    assert_eq!(d.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");

    let e = b.wrapping_rem_uint(5_u128);
    println!("b % 5 = {}", e);
    assert_eq!(e, 3);
    println!("-------------------------------");
}

fn BigUInt_constructors___main()
{
    BigUInt_new();
    BigUInt_zero();
    BigUInt_one();
    BigUInt_max();
    BigUInt_submax();
    BigUInt_halfmax();
    BigUInt_from_uint();
    BigUInt_from_array();
    BigUInt_from_biguint();
    BigUInt_from_be();
    BigUInt_from_be_bytes();
    BigUInt_from_le();
    BigUInt_from_le_bytes();
    BigUInt_from_string();
    BigUInt_from_str_radix();
    BigUInt_generate_check_bits_();
    BigUInt_generate_check_bits();
}

fn BigUInt_new()
{
    println!("BigUInt_new");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let obj = u256::new();
    let zero = u256::zero();
    println!("obj = {}", obj);
    assert_eq!(obj, zero);
    println!("-------------------------------");
}

fn BigUInt_zero()
{
    println!("BigUInt_zero");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let zero = u256::zero();
    let obj = u256::new();
    println!("zero = {}", zero);
    assert_eq!(zero, obj);
    println!("-------------------------------");
}

fn BigUInt_one()
{
    println!("BigUInt_one");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let one = u256::one();
    let mut obj = u256::new();
    obj.set_uint(1_u32);
    println!("one = {}", one);
    assert_eq!(one, obj);
    println!("-------------------------------");
}

fn BigUInt_max()
{
    println!("BigUInt_max");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let maximum = u256::max();
    println!("maximum =\t{}", maximum);
    assert_eq!(maximum, u256::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap());
    println!("---------------------------");
}

fn BigUInt_submax()
{
    println!("BigUInt_submax");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let half = u256::submax(128_usize);
    println!("half maximum = \t{}", half);
    assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    println!("---------------------------");
}

fn BigUInt_halfmax()
{
    println!("BigUInt_halfmax");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let half = u256::halfmax();
    println!("half maximum = \t{}", half);
    assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    println!("---------------------------");
}

fn BigUInt_from_uint()
{
    BigUInt_from_uint_u8();
    BigUInt_from_uint_u16();
    BigUInt_from_uint_u32();
    BigUInt_from_uint_u64();
    BigUInt_from_uint_u128();
    BigUInt_from_uint_usize();
}

fn BigUInt_from_uint_u8()
{
    println!("BigUInt_from_uint_u8");
    use Cryptocol::define_utypes_with_u16;
    define_utypes_with_u16!();
     
    let aa = u512::from_uint(123_u8);
    println!("aa = {}", aa);
    assert_eq!(aa.into_u8(), 123_u8);
    println!("---------------------------");
}

fn BigUInt_from_uint_u16()
{
    println!("BigUInt_from_uint_u16");
    use Cryptocol::define_utypes_with_u64;
    define_utypes_with_u64!();
     
    let bb = u512::from_uint(12345_u16);
    println!("bb = {}", bb);
    assert_eq!(bb.into_u16(), 12345_u16);
    println!("---------------------------");
}

fn BigUInt_from_uint_u32()
{
    println!("BigUInt_from_uint_u32");
    use Cryptocol::define_utypes_with_u8;
    define_utypes_with_u8!();
     
    let cc = u512::from_uint(1234567890_u32);
    println!("cc = {}", cc);
    assert_eq!(cc.into_u32(), 1234567890_u32);
    println!("---------------------------");
}

fn BigUInt_from_uint_u64()
{
    println!("BigUInt_from_uint_u64");
    use Cryptocol::define_utypes_with_u128;
    define_utypes_with_u128!();
     
    let dd = u512::from_uint(12345678901234567890_u64);
    println!("dd = {}", dd);
    assert_eq!(dd.into_u64(), 12345678901234567890_u64);
    println!("---------------------------");
}

fn BigUInt_from_uint_u128()
{
    println!("BigUInt_from_uint_u128");
    use Cryptocol::define_utypes_with_u128;
    define_utypes_with_u128!();
     
    let ee = u512::from_uint(123456789012345678901234567890123456789_u128);
    println!("ee = {}", ee);
    assert_eq!(ee.into_u128(), 123456789012345678901234567890123456789_u128);
    println!("---------------------------");
}

fn BigUInt_from_uint_usize()
{
    println!("BigUInt_from_uint_usize");
    use Cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let ff = u512::from_uint(123_usize);
    println!("ff = {}", ff);

    assert_eq!(ff.into_usize(), 123_usize);
    println!("---------------------------");
}

fn BigUInt_from_array()
{
    println!("BigUInt_from_array");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let big_num = u256::from_array(&[1_u8;32]);
    println!("big_num = {}", big_num.to_string_with_radix(2).unwrap());
    assert_eq!(big_num, u256::from_str_radix("00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001", 2).unwrap());
    println!("---------------------------");
}

fn BigUInt_from_biguint()
{
    BigUInt_from_biguint_same_length();
    BigUInt_from_biguint_shorter_length();
    BigUInt_from_biguint_longer_length();
}

fn BigUInt_from_biguint_same_length()
{
    println!("BigUInt_from_biguint_same_length");
    use std::str::FromStr;
    use Cryptocol::number::*;

    let a = u256_with_u8::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = u256_with_u16::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a.to_string(), b.to_string());
    println!("---------------------------");
}

fn BigUInt_from_biguint_shorter_length()
{
    println!("BigUInt_from_biguint_shorter_length");
    use std::str::FromStr;
    use Cryptocol::number::*;

    let a = u256_with_u8::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = u512_with_u16::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a.to_string(), b.to_string());
    println!("---------------------------");
}

fn BigUInt_from_biguint_longer_length()
{
    println!("BigUInt_from_biguint_longer_length");
    use std::str::FromStr;
    use Cryptocol::number::*;

    let a = u512_with_u8::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = u256_with_u16::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a.to_string(), b.to_string());
    println!("---------------------------");
}

fn BigUInt_from_be()
{
    println!("BigUInt_from_be");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let be = u256::from_array(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
                                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                                0x99, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
                                0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x70, 0x89]);
    let le = u256::from_be(&be);
    println!("be = 0x{}", be.to_string_with_radix(16).unwrap());
    println!("le = 0x{}", le.to_string_with_radix(16).unwrap());
    assert_eq!(be.to_string_with_radix(16).unwrap(), "89706A5B4C3D2E1FFFEEDDCCBBAA00998877665544332211EFCDAB9078563412");
    assert_eq!(le.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    println!("---------------------------");
}

fn BigUInt_from_be_bytes()
{
    println!("BigUInt_from_be");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let be_array = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
                    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                    0x99, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
                    0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x70, 0x89];
    let le = u256::from_be_bytes(&be_array);
    println!("be_array = {:?}", be_array);
    println!("le = {:?}", le);
    assert_eq!(le.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    println!("---------------------------");
}

fn BigUInt_from_le()
{
    println!("BigUInt_from_le");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let le1 = u256::from_array(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
                                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                                0x99, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
                                0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x70, 0x89]);
    let le2 = u256::from_le(&le1);
    println!("le1 = 0x{}", le1.to_string_with_radix(16).unwrap());
    println!("le2 = 0x{}", le2.to_string_with_radix(16).unwrap());
    assert_eq!(le1.to_string_with_radix(16).unwrap(), "89706A5B4C3D2E1FFFEEDDCCBBAA00998877665544332211EFCDAB9078563412");
    assert_eq!(le2.to_string_with_radix(16).unwrap(), "89706A5B4C3D2E1FFFEEDDCCBBAA00998877665544332211EFCDAB9078563412");
    println!("---------------------------");
}

fn BigUInt_from_le_bytes()
{
    println!("BigUInt_from_le_bytes");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let le_array = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
                    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                    0x99, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
                    0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x70, 0x89];
    let le = u256::from_le_bytes(&le_array);
    println!("le_array = {:?}", le_array);
    println!("le = {:?}", le);
    assert_eq!(le.to_string_with_radix(16).unwrap(), "89706A5B4C3D2E1FFFEEDDCCBBAA00998877665544332211EFCDAB9078563412");
    println!("---------------------------");
}

fn BigUInt_from_string()
{
    println!("BigUInt_from_string");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    println!("a = {}", a);
    assert_eq!(a.to_string(), "1234567890123456789012345678901234567890123456789012345678901234567890");
    println!("---------------------------");
}

fn BigUInt_from_str_radix()
{
    println!("BigUInt_from_str_radix");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    let a = u512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16).unwrap();
    println!("a = {}", a);
    assert_eq!(a.to_string(), "953444119462584670231660883005169236350945453535049253076624239367818227875140724454335257332337691463184490358643394140772086144551847644877923949534960");
    println!("---------------------------");
}

fn BigUInt_generate_check_bits()
{
    println!("BigUInt_generate_check_bits");
    use Cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let a_0 = u256::generate_check_bits(0).unwrap();
    println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    
    let a_12 = u256::generate_check_bits(12).unwrap();
    println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");

    let a_255 = u256::generate_check_bits(255).unwrap();
    println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");

    let a_256 = u256::generate_check_bits(256);
    println!("a_256 = {:?}", a_256);
    assert_eq!(a_256, None);
    println!("---------------------------");
}

fn BigUInt_generate_check_bits_()
{
    println!("BigUInt_generate_check_bits_");
    use Cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let a_0 = u256::generate_check_bits_(0);
    println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    
    let a_12 = u256::generate_check_bits_(12);
    println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");

    let a_255 = u256::generate_check_bits_(255);
    println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");

    // It will panic!
    // let a_256 = u256::generate_check_bits(256);
    println!("---------------------------");
}

fn BigUInt_random_number___main()
{
    BigUInt_any();
    BigUInt_any_odd();
    BigUInt_any_less_than();
    BigUInt_any_odd_less_than();
    BigUInt_any_with_MSB_set();
    BigUInt_any_odd_with_MSB_set();
    BigUInt_any_prime_using_Miller_Rabin();
    BigUInt_turn_any();
    BigUInt_random();
    BigUInt_random_odd();
    BigUInt_random_less_than();
    BigUInt_random_odd_less_than();
    BigUInt_random_with_MSB_set();
    BigUInt_random_odd_with_MSB_set();
    BigUInt_random_prime_using_Miller_Rabin();
    BigUInt_randomize();
    BigUInt_is_prime_using_Miller_Rabin();
}

fn BigUInt_any()
{
    println!("BigUInt_any");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    println!("Random Number: {}", u1024::any());
    println!("---------------------------");
}

fn BigUInt_any_odd()
{
    println!("BigUInt_any_odd");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    let r = u1024::any_odd();
    println!("Random Odd Number: {}", r);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn BigUInt_any_less_than()
{
    println!("BigUInt_any_less_than");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    let r = u1024::any_less_than(&ceiling);
    println!("Random Number less than {} is {}", ceiling, r);
    assert!(r < ceiling);
    println!("---------------------------");
}

fn BigUInt_any_odd_less_than()
{
    println!("BigUInt_any_odd_less_than");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    let r = u1024::any_odd_less_than(&ceiling);
    println!("Random Odd Number less than {} is {}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn BigUInt_any_with_MSB_set()
{
    println!("BigUInt_any_with_MSB_set");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    let num = u1024::any_with_MSB_set();
    println!("Random Number = {}", u1024::any());
    println!("1024-bit Random Number = {}", num);
    assert!(num > u1024::submax(1023));
    println!("---------------------------");
}

fn BigUInt_any_odd_with_MSB_set()
{
    println!("BigUInt_any_odd_with_MSB_set");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let num = u1024::any_odd_with_MSB_set();
    println!("Random Number = {}", u1024::any());
    println!("1024-bit Random Odd Number = {}", num);
    assert!(num > u1024::submax(1023));
    assert!(num.is_odd());
    println!("---------------------------");
}

fn BigUInt_any_prime_using_Miller_Rabin()
{
    println!("BigUInt_any_prime_using_Miller_Rabin");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    let num = u256::any_prime_using_Miller_Rabin(5);
    println!("Random Prime Number = {}", num);
    assert!(num.is_prime_using_Miller_Rabin(5));
    println!("---------------------------");
    
}

fn BigUInt_turn_any()
{
    println!("BigUInt_turn_any");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    let mut r = u256::new();
    println!("original number = {}", r);
    assert_eq!(r, u256::zero());
    r.turn_any();
    println!("random number = {}", r);
    assert_ne!(r, u256::zero());
    println!("---------------------------");
}


fn BigUInt_random()
{
    println!("BigUInt_random");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    println!("Random Number: {}", u1024::random());
    println!("---------------------------");
}

fn BigUInt_random_odd()
{
    println!("BigUInt_random_odd");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let r = u1024::random_odd();
    println!("Random Odd Number: {}", r);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn BigUInt_random_less_than()
{
    println!("BigUInt_random_less_than");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    let r = u1024::random_less_than(&ceiling);
    println!("Random Number less than {} is {}", ceiling, r);
    assert!(r < ceiling);
    println!("---------------------------");
}

fn BigUInt_random_odd_less_than()
{
    println!("BigUInt_random_odd_less_than");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    let r = u1024::random_odd_less_than(&ceiling);
    println!("Random Odd Number less than {} is {}", ceiling, u1024::random_odd_less_than(&ceiling));
    assert!(r < ceiling);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn BigUInt_random_with_MSB_set()
{
    println!("BigUInt_random_with_MSB_set");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    let num = u1024::random_with_MSB_set();
    println!("Random Number = {}", u1024::random());
    println!("1024-bit Random Number = {}", num);
    assert!(num > u1024::submax(1023));
    println!("---------------------------");
}

fn BigUInt_random_odd_with_MSB_set()
{
    println!("BigUInt_random_odd_with_MSB_set");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let num = u1024::random_odd_with_MSB_set();
    println!("Random Number = {}", u1024::random());
    println!("1024-bit Random Odd Number = {}", num);
    assert!(num > u1024::submax(1023));
    assert!(num.is_odd());
    println!("---------------------------");
}

fn BigUInt_random_prime_using_Miller_Rabin()
{
    println!("BigUInt_random_prime_using_Miller_Rabin");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    let num = u1024::random_prime_using_Miller_Rabin(5);
    assert!(num.is_prime_using_Miller_Rabin(5));
    println!("Random Prime Number = {}", num);
    println!("---------------------------");
}

fn BigUInt_randomize()
{
    println!("BigUInt_randomize");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut r = u1024::new();
    println!("original number = {}", r);
    assert_eq!(r, u1024::zero());
    r.randomize();
    println!("random number = {}", r);
    assert_ne!(r, u1024::zero());
    println!("---------------------------");
}

fn BigUInt_is_prime_using_Miller_Rabin()
{
    println!("BigUInt_is_prime_using_Miller_Rabin");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let num = u1024::from_string("157847659859841049478697210209054499132116730052547470511818639401226705057924429751936169954758794979780692256039595351594450957429818931145981533862363167515145703012676459279601554094177152095755375227908501443524236048737351327752857335149319939532219166843564206337168180636940438709755340632429325500479").unwrap();
    let yes = num.is_prime_using_Miller_Rabin(5);
    println!("Is {} a prime number? => {}", num, yes);
    if yes  { assert!(yes); }
    else    { assert!(!yes); }
    println!("---------------------------");
}

fn BigUInt_get_size___main()
{
    BigUInt_size_in_bytes();
    BigUInt_size_in_bits();
    BigUInt_length_in_bytes();
    BigUInt_length_in_bits();
}

fn BigUInt_size_in_bytes()
{
    println!("BigUInt_size_in_bytes");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    println!("u256 is {}-byte integer.", u256::size_in_bytes());
    assert_eq!(u256::size_in_bytes(), 32);
    println!("---------------------------");
}

fn BigUInt_size_in_bits()
{
    println!("BigUInt_size_in_bits");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    println!("u256 is {}-bit integer.", u256::size_in_bits());
    assert_eq!(u256::size_in_bits(), 256);
    println!("---------------------------");
}

fn BigUInt_length_in_bytes()
{
    println!("BigUInt_length_in_bytes");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a = u256::from_str_radix("A16F", 16).unwrap();
    println!("a is {}-byte integer.", a.length_in_bytes());
    assert_eq!(a.length_in_bytes(), 32);
    println!("---------------------------");
}

fn BigUInt_length_in_bits()
{
    println!("BigUInt_length_in_bits");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u256::from_str_radix("A16F", 16).unwrap();
    println!("a is {}-bit integer.", a.length_in_bits());
    assert_eq!(a.length_in_bits(), 256);
    println!("---------------------------");
}

fn BigUInt_get_set_check___main()
{
    BigUInt_turn_check_bits();
    BigUInt_is_bit_set();
    BigUInt_is_bit_set_();
    BigUInt_get_upper_portion();
    BigUInt_get_lower_portion();
    BigUInt_get_num();
    BigUInt_get_num_();
    BigUInt_set_num();
    BigUInt_set_num_();
    BigUInt_get_number();
    BigUInt_set_number();
    BigUInt_copy_within();
    BigUInt_set_zero();
    BigUInt_is_zero();
    BigUInt_set_one();
    BigUInt_is_one();
    BigUInt_is_zero_or_one();
    BigUInt_set_max();
    BigUInt_set_submax();
    BigUInt_set_halfmax();
    BigUInt_is_max();
    BigUInt_set_msb();
    BigUInt_set_lsb();
    BigUInt_set_uint();
    BigUInt_is_uint();
    BigUInt_is_odd();
    BigUInt_is_even();

}

fn BigUInt_turn_check_bits()
{
    println!("BigUInt_turn_check_bits");
    use Cryptocol::define_utypes_with;
    
    define_utypes_with!(u128);
    let mut a = u256::any();
    println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    a.turn_check_bits(102);
    println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a, u256::from_str_radix("1000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000", 2).unwrap());
    println!("---------------------------");
}

fn BigUInt_is_bit_set()
{
    println!("BigUInt_is_bit_set");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a = {}_u256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    let mut res = a.is_bit_set(151);
    match res
    {
        Some(r) => {
            println!("The {}th bit is set: {}", 151, r);
            assert_eq!(a.is_bit_set_(151), true);
        },
        None => {
            println!("{}_u256 does not have the {}th bit.", a, 151);
        }
    }

    res = a.is_bit_set(200);
    match res
    {
        Some(r) => {
            println!("The {}th bit is set: {}", 200, r);
            assert_eq!(a.is_bit_set_(200), false);
        },
        None => {
            println!("{}_u256 does not have the {}th bit.", a, 200);
        }
    }

    res = a.is_bit_set(300);
    match res
    {
        Some(r) => {
            println!("The {}th bit is set: {}", 300, r);
            assert_eq!(a.is_bit_set_(300), true);
        },
        None => {
            println!("{}_u256 does not have the {}th bit.", a, 300);
        }
    }
    println!("---------------------------");
}

fn BigUInt_is_bit_set_()
{
    println!("BigUInt_is_bit_set_");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a = {}_u256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    println!("The {}th bit is set: {}", 151, a.is_bit_set_(151));
    assert_eq!(a.is_bit_set_(151), true);
    println!("The {}th bit is set: {}", 200, a.is_bit_set_(200));
    assert_eq!(a.is_bit_set_(200), false);
    // It will panic!!!
    // println!("The {}th bit is set: {}", 300, a.is_bit_set_(300));
    println!("---------------------------");
}

fn BigUInt_get_upper_portion()
{
    println!("BigUInt_get_upper_portion");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a = {}_u256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    let b = a.get_upper_portion(10);
    println!("The 10-bit upper portion of {}_u256 is {}_u256", a, b);
    assert_eq!(b.to_string(), "873");
    println!("---------------------------");
}

fn BigUInt_get_lower_portion()
{
    println!("BigUInt_get_lower_portion");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a = {}_u256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    let b = a.get_lower_portion(10);
    println!("The 10-bit lower portion of {}_u256 is {}_u256", a, b);
    assert_eq!(b.to_string(), "857");
    println!("---------------------------");
}

fn BigUInt_get_num()
{
    println!("BigUInt_get_num");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    let e = a.get_num(3);
    match a.get_num(3)
    {
        Some(num) => {
            println!("a.get_num(3).unwrap() = {}", num);
            assert_eq!(num, 30);
        },
        None => {
            println!("There is no third element.");
            assert_eq!(e, None);
        },
    }
    let f = a.get_num(8);
    match f
    {
        Some(num) => {
            println!("a.get_num(3).unwrap() = {}", num);
            assert_eq!(num, 30);
        },
        None => {
            println!("There is no third element.");
            assert_eq!(f, None);
        },
    }
    println!("---------------------------");
}

fn BigUInt_get_num_()
{
    println!("BigUInt_get_num_");
    use Cryptocol::define_utypes_with;
    
    define_utypes_with!(u32);
    let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    let b = a.get_num_(3);
    println!("a.get_num_(3) = {}", b);
    assert_eq!(b, 30);
    // It will panic.
    // let c = a.get_num_(8);
    println!("---------------------------");
}

fn BigUInt_set_num()
{
    println!("BigUInt_set_num");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a = u256::from([0_u64, 10, 20, 30]);
    let mut num = a.get_num_(3);
    println!("a.get_num(3).unwrap() = {}", num);
    let b = a.set_num(3, 0);
    num = a.get_num_(3);
    println!("a.get_num(3).unwrap() = {}", num);
    assert!(b);
    assert_eq!(num, 0);

    let c = a.set_num(4, 0);
    if !c
        { println!("There is no fourth element."); }
    assert!(!c);
    println!("---------------------------");
}

fn BigUInt_set_num_()
{
    println!("BigUInt_set_num_");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = u256::from([10_u128, 20]);
    let mut num = a.get_num_(1);
    println!("a.get_num(1).unwrap() = {}", num);
    let b = a.set_num_(1, 0);
    num = a.get_num_(1);
    println!("a.get_num(1).unwrap() = {}", num);
    assert_eq!(num, 0);

    // It will panic.
    // let c = a.set_num_(4, 0);
    println!("---------------------------");
}

fn BigUInt_get_number()
{
    println!("BigUInt_get_number");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    if let Ok(a) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<u256>()
    {
        let arr = a.get_number();
        println!("arr = {:?}", arr);
        assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    }
    println!("---------------------------");
}

fn BigUInt_set_number()
{
    println!("BigUInt_set_number");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    let mut a = u256::new();
    println!("arr = {:?}", a);
    a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    println!("arr = {:?}", a);
    assert_eq!(a.get_number(), &[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    println!("---------------------------");
}

fn BigUInt_copy_within()
{
    println!("BigUInt_copy_within");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    let mut a = u256::new();
    a.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    println!("a = {:?}", a);
    a.copy_within(3..10, 6);
    println!("a = {:?}", a);
    assert_eq!(a.get_number(), &[0, 1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
    println!("---------------------------");
}

fn BigUInt_set_zero()
{
    println!("BigUInt_set_zero");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = u256::new();
    a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    println!("a = {}", a);
    a.set_zero();
    println!("a = {}", a);
    assert_eq!(a, u256::zero());
    println!("---------------------------");
}

fn BigUInt_is_zero()
{
    println!("BigUInt_is_zero");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let mut a = u1024::zero();
    if a.is_zero()
        { println!("a is Zero"); }
    else
        { println!("a is Not Zero"); }
    assert!(a.is_zero());

    a.set_one();
    if a.is_zero()
        { println!("a is Zero"); }
    else
        { println!("a is Not Zero"); }
    assert!(!a.is_zero());
    println!("---------------------------");
}

fn BigUInt_set_one()
{
    println!("BigUInt_set_one");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = u256::new();
    a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    println!("a = {}", a);
    a.set_one();
    println!("a = {}", a);
    assert_eq!(a, u256::one());
    println!("---------------------------");
}

fn BigUInt_is_one()
{
    println!("BigUInt_is_one");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let mut a = u256::one();
    if a.is_one()
        { println!("a is One"); }
    else
        { println!("a is Not One"); }
    assert!(a.is_one());

    a.set_max();
    if a.is_one()
        { println!("a is One"); }
    else
        { println!("a is Not One"); }
    assert!(!a.is_one());
    println!("---------------------------");
}

fn BigUInt_is_zero_or_one()
{
    println!("BigUInt_is_zero_or_one");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a = u256::zero();
    println!("a = {}", a);
    if a.is_zero_or_one()
        { println!("a is One or Zero."); }
    else
        { println!("a is Neither One nor Zero."); }
    assert!(a.is_zero_or_one());

    a.wrapping_add_assign_uint(1_u8);
    println!("a = {}", a);
    if a.is_zero_or_one()
        { println!("a is One or Zero."); }
    else
        { println!("a is Neither One nor Zero."); }
    assert!(a.is_zero_or_one());

    a.wrapping_add_assign_uint(1_u8);
    println!("a = {}", a);
    if a.is_zero_or_one()
        { println!("a is One or Zero."); }
    else
        { println!("a is Neither One nor Zero."); }
    assert!(!a.is_zero_or_one());
    println!("---------------------------");
}

fn BigUInt_set_max()
{
    println!("BigUInt_set_max");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a = u256::new();
    println!("a = {}", a);
    a.set_max();
    println!("a = {}", a);
    assert_eq!(a.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    println!("---------------------------");
}

fn BigUInt_set_submax()
{
    println!("BigUInt_set_submax");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = u256::new();
    a.set_max();
    println!("a = {}", a);
    assert_eq!(a, u256::max());

    a.set_submax(200_usize);
    println!("a = {}", a);
    assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    println!("---------------------------");
}

fn BigUInt_set_halfmax()
{
    println!("BigUInt_set_halfmax");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a = u256::new();
    println!("a = {}", a);
    a.set_halfmax();
    println!("a = {}", a);
    assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    println!("---------------------------");
}

fn BigUInt_is_max()
{
    println!("BigUInt_is_max");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::max();
    println!("Is {} a 256-bit maximun? - {}", a, a.is_max());
    assert_eq!(a.is_max(), true);
    println!("---------------------------");
}

fn BigUInt_set_msb()
{
    println!("BigUInt_set_msb");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a = u256::new();
    println!("a = {}", a);
    a.set_MSB();
    println!("a = {}", a);
    assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    println!("---------------------------");
}

fn BigUInt_set_lsb()
{
    println!("BigUInt_set_lsb");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a = u256::new();
    println!("a = {}", a);
    a.set_LSB();
    println!("a = {}", a);
    assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "1");
    println!("---------------------------");
}

fn BigUInt_set_uint()
{
    println!("BigUInt_set_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a = u1024::new();
    println!("a = {}", a);
    a.set_uint(340282366920938463453374607431768211455_u128);
    println!("a = {}", a);
    assert_eq!(a.to_string(), "340282366920938463453374607431768211455");
    println!("---------------------------");
}

fn BigUInt_is_uint()
{
    println!("BigUInt_is_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a = u1024::one() + 50_u16;
    println!("Question: Is a 51?\nAnswer: {}", a.is_uint(51_u32));
    assert!(a.is_uint(51_u16));
    println!("---------------------------");
}

fn BigUInt_is_odd()
{
    println!("BigUInt_is_odd");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a = u256::new();
    a.set_uint(340282366920938463453374697431768211455_u128);
    if a.is_odd()
        { println!("{} is odd", a); }
    else
        { println!("{} is even", a); }
    assert!(a.is_odd());

    a <<= 1;
    if a.is_odd()
        { println!("{} is odd", a); }
    else
        { println!("{} is even", a); }
    assert!(!a.is_odd());
    println!("---------------------------");
}

fn BigUInt_is_even()
{
    println!("BigUInt_is_even");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = u256::new();
    a.set_uint(169743176821145534028236692093846345739_u128);
    if a.is_even()
        { println!("{} is even", a); }
    else
        { println!("{} is odd", a); }
    assert!(!a.is_even());

    a <<= 1;
    if a.is_even()
        { println!("{} is even", a); }
    else
        { println!("{} is odd", a); }
    assert!(a.is_even());
    println!("---------------------------");
}

fn BigUInt_check_bits___main()
{
    BigUInt_count_ones();
    BigUInt_count_zeros();
    BigUInt_leading_ones();
    BigUInt_leading_zeros();
    BigUInt_trailing_ones();
    BigUInt_trailing_zeros();
    BigUInt_leading_max_elements();
    BigUInt_leading_zero_elements();
    BigUInt_trailing_max_elements();
    BigUInt_trailing_zero_elements();
}

fn BigUInt_count_ones()
{
    println!("BigUInt_count_ones");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    println!("{} has {} ones in binary.", a, a.count_ones());
    assert_eq!(a.count_ones(), 107);
    println!("---------------------------");
}

fn BigUInt_count_zeros()
{
    println!("BigUInt_count_zeros");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<u256>().unwrap();
    println!("{} has {} zeros in binary.", a, a.count_zeros());
    assert_eq!(a.count_zeros(), 149);
    println!("---------------------------");
}

fn BigUInt_leading_ones()
{
    println!("BigUInt_leading_ones");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    println!("{} has {} leading ones in binary.", a, a.leading_ones());
    assert_eq!(a.leading_ones(), 2);
    println!("---------------------------");
}

fn BigUInt_leading_zeros()
{
    println!("BigUInt_leading_zeros");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<u256>().unwrap();
    println!("{} has {} leading ones in binary.", a, a.leading_zeros());
    assert_eq!(a.leading_zeros(), 0);
    println!("---------------------------");
}

fn BigUInt_trailing_ones()
{
    println!("BigUInt_trailing_ones");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_str("111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();
    println!("{} has {} trailing ones in binary.", a, a.trailing_ones());
    assert_eq!(a.trailing_ones(), 3);
    println!("---------------------------");
}

fn BigUInt_trailing_zeros()
{
    println!("BigUInt_trailing_zeros");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a = "111111111111111111111111111111111111111111111111111111111111111111111111111111".parse::<u256>().unwrap();
    println!("{} has {} trailing zeros in binary.", a, a.trailing_zeros());
    assert_eq!(a.trailing_zeros(), 0);
    println!("---------------------------");
}

fn BigUInt_leading_max_elements()
{
    println!("BigUInt_leading_max_elements");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999_88888888", 16).unwrap();
    println!("{} has {} leading max elements in array.", a, a.leading_max_elements());
    assert_eq!(a.leading_max_elements(), 4);
    println!("---------------------------");
}

fn BigUInt_leading_zero_elements()
{
    println!("BigUInt_leading_zero_elements");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u256::from_str_radix("00000000_FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999", 16).unwrap();
    println!("{} has {} leading zero elements in array.", a, a.leading_zero_elements());
    assert_eq!(a.leading_zero_elements(), 1);
    println!("---------------------------");
}

fn BigUInt_trailing_max_elements()
{
    println!("BigUInt_trailing_max_elements");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a = u256::from_str_radix("88888888_99999999_AAAAAAAA_BBBBBBBB_CCCCCCCC_DDDDDDDD_EEEEEEEE_FFFFFFFF", 16).unwrap();
    println!("{} has {} leading max elements in array.", a, a.trailing_max_elements());
    assert_eq!(a.trailing_max_elements(), 2);
    println!("---------------------------");
}

fn BigUInt_trailing_zero_elements()
{
    println!("BigUInt_trailing_zero_elements");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_9999999_900000000", 16).unwrap();
    println!("{} has {} leading zero elements in array.", a, a.trailing_zero_elements());
    assert_eq!(a.trailing_zero_elements(), 4);
    println!("---------------------------");
}

fn BigUInt_comparison_uint___main()
{
    BigUInt_partial_cmp_uint();
    BigUInt_lt_uint();
    BigUInt_gt_uint();
    BigUInt_le_uint();
    BigUInt_ge_uint();
    BigUInt_eq_uint();
}

fn BigUInt_partial_cmp_uint()
{
    println!("BigUInt_partial_cmp_uint");
    use std::cmp::Ordering;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut res = U32::from_uint(100_u8).partial_cmp_uint(90_u128).unwrap();
    match res
    {
        Ordering::Greater => { println!("100 > 90"); }
        Ordering::Less => { println!("100 < 90"); }
        Ordering::Equal => { println!("100 = 90"); }
    }
    assert_eq!(res, Ordering::Greater);

    res = U32::from_uint(100_u8).partial_cmp_uint(110_u128).unwrap();
    match res
    {
        Ordering::Greater => { println!("100 > 110"); }
        Ordering::Less => { println!("100 < 110"); }
        Ordering::Equal => { println!("100 = 110"); }
    }
    assert_eq!(res, Ordering::Less);

    res = U32::from_uint(100_u8).partial_cmp_uint(100_u128).unwrap();
    match res
    {
        Ordering::Greater => { println!("100 > 100"); }
        Ordering::Less => { println!("100 < 100"); }
        Ordering::Equal => { println!("100 = 100"); }
    }
    assert_eq!(res, Ordering::Equal);
    println!("---------------------------");
}

fn BigUInt_lt_uint()
{
    println!("BigUInt_lt_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut res = U32::from_uint(100_u16).lt_uint(110_u64);
    if res
        { println!("100 < 110"); }
    else
        { println!("100 >= 110"); }
    assert_eq!(res, true);

    res = U32::from_uint(100_u16).lt_uint(90_u64);
    if res
        { println!("100 < 90"); }
    else
        { println!("100 >= 90"); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn BigUInt_gt_uint()
{
    println!("BigUInt_gt_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut res = U32::from_uint(100_u32).gt_uint(90_u32);
    if res
        { println!("100 > 90"); }
    else
        { println!("100 <= 90"); }
    assert_eq!(res, true);

    res = U32::from_uint(100_u32).gt_uint(110_u32);
    if res
        { println!("100 > 110"); }
    else
        { println!("100 <= 110"); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn BigUInt_le_uint()
{
    println!("BigUInt_le_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut res = U32::from_uint(100_u64).le_uint(110_u16);
    if res
        { println!("100 <= 110"); }
    else
        { println!("100 > 110"); }
    assert_eq!(res, true);

    res = U32::from_uint(100_u64).lt_uint(90_u16);
    if res
        { println!("100 < 90"); }
    else
        { println!("100 >= 90"); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn BigUInt_ge_uint()
{
    println!("BigUInt_ge_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut res = U32::from_uint(100_u128).gt_uint(90_u8);
    if res
        { println!("100 >= 90"); }
    else
        { println!("100 <= 90"); }
    assert_eq!(res, true);

    res = U32::from_uint(100_u128).gt_uint(110_u8);
    if res
        { println!("100 > 110"); }
    else
        { println!("100 <= 110"); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn BigUInt_eq_uint()
{
    println!("BigUInt_eq_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut res = U32::from_uint(100_u32).eq_uint(100_u8);
    if res
        { println!("100 = 100"); }
    else
        { println!("100 != 100"); }
    assert_eq!(res, true);

    res = U32::from_uint(100_u64).eq_uint(200_u16);
    if res
        { println!("100 = 200"); }
    else
        { println!("100 != 200"); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn BigUInt_comparison_biguint___main()
{
    BigUInt_eq_biguint();
    BigUInt_partial_cmp_biguint();
}

fn BigUInt_eq_biguint()
{
    println!("BigUInt_eq_biguint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let mut res = U32::from_string(num_str).unwrap().eq(&U32::from_string(num_str).unwrap());
    if res
        { println!("{0} = {0}", num_str); }
    else
        { println!("{0} != {0}", num_str); }
    assert_eq!(res, true);

    res = U32::from_string(num_str).unwrap().eq(&U32::from_uint(100_u8));
    if res
        { println!("{} = 100", num_str); }
    else
        { println!("{} != 100", num_str); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn BigUInt_partial_cmp_biguint()
{
    println!("BigUInt_partial_cmp_biguint");
    use std::cmp::Ordering;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    let num_str2 = "60000000000000000000000000000000000000000000000000000000000000000000000000000";
    let num_str3 = "80000000000000000000000000000000000000000000000000000000000000000000000000000";
    let num1 = num_str1.parse::<U32>().unwrap();
    let num2 = num_str2.parse::<U32>().unwrap();
    let num3 = num_str3.parse::<U32>().unwrap();

    let mut res = num1.partial_cmp(&num2).unwrap();
    match res
    {
        Ordering::Greater => { println!("{} > {}", num1, num2); }
        Ordering::Less => { println!("{} < {}", num1, num2); }
        Ordering::Equal => { println!("{} = {}", num1, num2); }
    }
    assert_eq!(res, Ordering::Greater);

    res = num1.partial_cmp(&num3).unwrap();
    match res
    {
        Ordering::Greater => { println!("{} > {}", num1, num3); }
        Ordering::Less => { println!("{} < {}", num1, num3); }
        Ordering::Equal => { println!("{} = {}", num1, num3); }
    }
    assert_eq!(res, Ordering::Less);

    res = num1.partial_cmp(&num1).unwrap();
    match res
    {
        Ordering::Greater => { println!("{0} > {0}", num1); }
        Ordering::Less => { println!("{0} < {0}", num1); }
        Ordering::Equal => { println!("{0} = {0}", num1); }
    }
    assert_eq!(res, Ordering::Equal);
    println!("---------------------------");
}

fn BigUInt_arithmatic_operation_uint___main()
{
    BigUInt_add_uint();
    BigUInt_sub_uint();
    BigUInt_mul_uint();
    BigUInt_div_uint();
    BigUInt_rem_uint();
    BigUInt_next_multiple_uint();
    BigUInt_exponentiation_logarithm_uint();
}

fn BigUInt_add_uint()
{
    BigUInt_carrying_add_uint();
    BigUInt_carrying_add_assign_uint();
    BigUInt_wrapping_add_uint();
    BigUInt_wrapping_add_assign_uint();
    BigUInt_overflowing_add_uint();
    BigUInt_overflowing_add_assign_uint();
    BigUInt_checked_add_uint();
    BigUInt_unchecked_add_uint();
    BigUInt_saturating_add_uint();
    BigUInt_saturating_add_assign_uint();
    BigUInt_modular_add_uint();
    BigUInt_modular_add_assign_uint();
}

fn BigUInt_carrying_add_uint()
{
    println!("BigUInt_carrying_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let num1 = U32::from_str_radix(num_str1, 16).unwrap();
    let num2 = U32::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;

    let (mut sum, mut carry) = num1.carrying_add_uint(num_uint, false);
    println!("{} + {} = {}\ncarry = {}", num1, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722605");
    assert_eq!(carry, false);

    (sum, carry) = num1.carrying_add_uint(num_uint, true);
    println!("{} + {} = {}\ncarry = {}", num1, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722606");
    assert_eq!(carry, false);

    (sum, carry) = num2.carrying_add_uint(num_uint, false);
    println!("{} + {} = {}\ncarry = {}", num2, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "22774453838368691933710012711845097214");
    assert_eq!(carry, true);

    (sum, carry) = num2.carrying_add_uint(num_uint, true);
    println!("{} + {} = {}\ncarry = {}", num2, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "22774453838368691933710012711845097215");
    assert_eq!(carry, true);
    println!("---------------------------");
}

fn BigUInt_carrying_add_assign_uint()
{
    println!("BigUInt_carrying_add_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let mut num1 = u256::from_str_radix(num_str1, 16).unwrap();
    let mut num2 = u256::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFF_u64;

    println!("Originally,\tnum1 = {}", num1);
    let mut num3 = num1.clone();
    let mut carry = num1.carrying_add_assign_uint(num_uint, false);
    println!("After num1 += {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302573");
    assert_eq!(carry, false);

    num1 = num3;
    println!("Originally,\tnum1 = {}", num1);
    carry = num1.carrying_add_assign_uint(num_uint, true);
    println!("After num1 += {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302574");
    assert_eq!(carry, false);

    num3 = num2.clone();
    println!("Originally,\tnum2 = {}", num2);
    carry = num2.carrying_add_assign_uint(num_uint, false);
    println!("After num2 += {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    assert_eq!(num2.to_string(), "11024999611375677182");
    assert_eq!(carry, true);

    num2 = num3;
    println!("Originally,\tnum2 = {}", num2);
    carry = num2.carrying_add_assign_uint(num_uint, true);
    println!("After num2 += {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    assert_eq!(num2.to_string(), "11024999611375677183");
    assert_eq!(carry, true);
    println!("---------------------------");
}

fn BigUInt_wrapping_add_uint()
{
    println!("BigUInt_wrapping_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u512::max().wrapping_sub_uint(1_u8);
    let b = a.wrapping_add_uint(1_u8);
    let c = a.wrapping_add_uint(2_u8);
    let d = a.wrapping_add_uint(3_u8);
    
    println!("{} + 1 = {}", a, b);
    assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");

    println!("{} + 2 = {}", a, c);
    assert_eq!(c.to_string(), "0");

    println!("{} + 3 = {}", a, d);
    assert_eq!(d.to_string(), "1");
    println!("---------------------------");
}

fn BigUInt_wrapping_add_assign_uint()
{
    println!("BigUInt_wrapping_add_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = U64::max().wrapping_sub_uint(1_u8);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    
    a.wrapping_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    
    a.wrapping_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "0");
    
    a.wrapping_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "1");
    println!("---------------------------");
}

fn BigUInt_overflowing_add_uint()
{
    println!("BigUInt_overflowing_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u512::max().wrapping_sub_uint(1_u8);
    let (b, overflow) = a.overflowing_add_uint(1_u8);
    println!("{} + 1 = {}\noverflow = {}", a, b, overflow);
    assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(overflow, false);

    let (c, overflow) = a.overflowing_add_uint(2_u8);
    println!("{} + 2 = {}\noverflow = {}", a, c, overflow);
    assert_eq!(c.to_string(), "0");
    assert_eq!(overflow, true);

    let (d, overflow) = a.overflowing_add_uint(3_u8);
    println!("{} + 3 = {}\noverflow = {}", a, d, overflow);
    assert_eq!(d.to_string(), "1");
    assert_eq!(overflow, true);
    println!("---------------------------");
}

fn BigUInt_overflowing_add_assign_uint()
{
    println!("BigUInt_overflowing_add_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = U64::max().wrapping_sub_uint(1_u8);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");

    let mut overflow = a.overflowing_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}\noverflow = {}", a, overflow);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(overflow, false);

    overflow = a.overflowing_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}\noverflow = {}", a, overflow);
    assert_eq!(a.to_string(), "0");
    assert_eq!(overflow, true);

    overflow = a.overflowing_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}\noverflow = {}", a, overflow);
    assert_eq!(a.to_string(), "1");
    assert_eq!(overflow, false);
    println!("---------------------------");
}

fn BigUInt_checked_add_uint()
{
    println!("BigUInt_checked_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u512::max().wrapping_sub_uint(1_u8);
    let b = a.checked_add_uint(1_u8);
    match b
    {
        Some(num) => {
            println!("{} + 1 = {}", a, num);
            assert_eq!(num.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
        },
        None => {
            println!("{} + 1 = overflow", a);
        }
    }

    let c = a.checked_add_uint(2_u8);
    match c
    {
        Some(num) => {
            println!("{} + 2 = {}", a, num);
        },
        None => {
            println!("{} + 2 = overflow", a);
            assert_eq!(c, None);
        }
    }

    let d = a.checked_add_uint(3_u8);
    match d
    {
        Some(num) => {
            println!("{} + 3 = {}", a, num);
        },
        None => {
            println!("{} + 3 = overflow", a);
            assert_eq!(d, None);
        }
    }
    println!("---------------------------");
}

fn BigUInt_unchecked_add_uint()
{
    println!("BigUInt_unchecked_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = U64::max().wrapping_sub_uint(1_u8);
    let b = a.unchecked_add_uint(1_u8);
    println!("{} + 1 = {}", a, b);
    assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");

    // It will panic.
    // let c = a.unchecked_add_uint(2_u8);
    println!("---------------------------");
}

fn BigUInt_saturating_add_uint()
{
    println!("BigUInt_saturating_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u512::max().wrapping_sub_uint(2_u8);
    let b = a.saturating_add_uint(1_u8);
    let c = a.saturating_add_uint(2_u8);
    let d = a.saturating_add_uint(3_u8);
    
    println!("{} + 1 = {}", a, b);
    assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");

    println!("{} + 2 = {}", a, c);
    assert_eq!(c.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");

    println!("{} + 3 = {}", a, d);
    assert_eq!(d.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    println!("---------------------------");
}

fn BigUInt_saturating_add_assign_uint()
{
    println!("BigUInt_saturating_add_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = U64::max().wrapping_sub_uint(2_u8);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    
    a.saturating_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    
    a.saturating_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    
    a.saturating_add_assign_uint(1_u8);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    println!("---------------------------");
}

fn BigUInt_modular_add_uint()
{
    println!("BigUInt_modular_add_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a.wrapping_add_uint(2_u8);
    let b = a.modular_add_uint(1_u8, &m);
    let c = a.modular_add_uint(2_u8, &m);
    let d = a.modular_add_uint(3_u8, &m);
    
    println!("{} + 1 = {}", a, b);
    assert_eq!(b.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");

    println!("{} + 2 = {}", a, c);
    assert_eq!(c.to_string(), "0");

    println!("{} + 3 = {}", a, d);
    assert_eq!(d.to_string(), "1");
    println!("---------------------------");
}

fn BigUInt_modular_add_assign_uint()
{
    println!("BigUInt_modular_add_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a = u256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    let m = a.wrapping_add_uint(2_u8);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    
    a.modular_add_assign_uint(1_u8, &m);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    
    a.modular_add_assign_uint(1_u8, &m);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "0");
    
    a.modular_add_assign_uint(1_u8, &m);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a.to_string(), "1");
    println!("---------------------------");
}


fn BigUInt_sub_uint()
{
    BigUInt_borrowing_sub_uint();
    BigUInt_borrowing_sub_assign_uint();
    BigUInt_wrapping_sub_uint();
    BigUInt_wrapping_sub_assign_uint();
    BigUInt_overflowing_sub_uint();
    BigUInt_overflowing_sub_assign_uint();
    BigUInt_checked_sub_uint();
    BigUInt_unchecked_sub_uint();
    BigUInt_saturating_sub_uint();
    BigUInt_saturating_sub_assign_uint();
    BigUInt_modular_sub_uint();
    BigUInt_modular_sub_assign_uint();
    BigUInt_abs_diff_uint();
}

fn BigUInt_borrowing_sub_uint()
{
    println!("BigUInt_borrowing_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let num_str2 = "11223344_55667788_9900AABB_CCDDEEEe";
    let num1 = U32::from_str_radix(num_str1, 16).unwrap();
    let num2 = U32::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;

    let (mut dif, mut carry) = num1.borrowing_sub_uint(num_uint, false);
    println!("{} - {} = {}\ncarry = {}", num1, num_uint, dif, carry);
    assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528175");
    assert_eq!(carry, false);

    (dif, carry) = num1.borrowing_sub_uint(num_uint, true);
    println!("{} - {} = {}\ncarry = {}", num1, num_uint, dif, carry);
    assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528174");
    assert_eq!(carry, false);

    (dif, carry) = num2.borrowing_sub_uint(num_uint, false);
    println!("{} - {} = {}\ncarry = {}", num2, num_uint, dif, carry);
    assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639919");
    assert_eq!(carry, true);

    (dif, carry) = num2.borrowing_sub_uint(num_uint, true);
    println!("{} - {} = {}\ncarry = {}", num2, num_uint, dif, carry);
    assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639918");
    assert_eq!(carry, true);
    println!("---------------------------");
}

fn BigUInt_borrowing_sub_assign_uint()
{
    println!("BigUInt_borrowing_sub_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let num_str2 = "9900AABB_CCDDEEFe";
    let mut num1 = u256::from_str_radix(num_str1, 16).unwrap();
    let mut num2 = u256::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFf_u64;

    println!("Originally,\tnum1 = {}", num1);
    let mut num3 = num1.clone();
    let mut carry = num1.borrowing_sub_assign_uint(num_uint, false);
    println!("After num1 -= {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948207");
    assert_eq!(carry, false);

    num1 = num3;
    println!("Originally,\tnum1 = {}", num1);
    carry = num1.borrowing_sub_assign_uint(num_uint, true);
    println!("After num1 -= {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948206");
    assert_eq!(carry, false);

    num3 = num2.clone();
    println!("Originally,\tnum2 = {}", num2);
    carry = num2.borrowing_sub_assign_uint(num_uint, false);
    println!("After num2 -= {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    assert_eq!(num2.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(carry, true);

    num2 = num3;
    println!("Originally,\tnum2 = {}", num2);
    carry = num2.borrowing_sub_assign_uint(num_uint, true);
    println!("After num2 -= {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    assert_eq!(num2.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639934");
    assert_eq!(carry, true);
    println!("---------------------------");
}

fn BigUInt_wrapping_sub_uint()
{
    println!("BigUInt_wrapping_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = u512::one();
    let b = a.wrapping_sub_uint(1_u8);
    let c = a.wrapping_sub_uint(2_u8);
    let d = a.wrapping_sub_uint(3_u8);
    
    println!("{} - 1 = {}", a, b);
    assert_eq!(b.to_string(), "0");

    println!("{} - 2 = {}", a, c);
    assert_eq!(c.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");

    println!("{} - 3 = {}", a, d);
    assert_eq!(d.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    println!("---------------------------");
}

fn BigUInt_wrapping_sub_assign_uint()
{
    println!("fn BigUInt_wrapping_sub_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = U64::one();
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "1");
    
    a.wrapping_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "0");
    
    a.wrapping_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    
    a.wrapping_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    println!("---------------------------");
}

fn BigUInt_overflowing_sub_uint()
{
    println!("BigUInt_overflowing_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u512::one();
    let (b, underflow) = a.overflowing_sub_uint(1_u8);
    println!("{} - 1 = {}\nunderflow = {}", a, b, underflow);
    assert_eq!(b.to_string(), "0");
    assert_eq!(underflow, false);

    let (c, underflow) = a.overflowing_sub_uint(2_u8);
    println!("{} - 2 = {}\nunderflow = {}", a, c, underflow);
    assert_eq!(c.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(underflow, true);

    let (d, underflow) = a.overflowing_sub_uint(3_u8);
    println!("{} - 3 = {}\nunderflow = {}", a, d, underflow);
    assert_eq!(d.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(underflow, true);
    println!("---------------------------");
}

fn BigUInt_overflowing_sub_assign_uint()
{
    println!("BigUInt_overflowing_sub_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a = U64::one();
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "1");

    let mut underflow = a.overflowing_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}\nunderflow = {}", a, underflow);
    assert_eq!(a.to_string(), "0");
    assert_eq!(underflow, false);

    underflow = a.overflowing_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}\nunderflow = {}", a, underflow);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(underflow, true);

    underflow = a.overflowing_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}\nunderflow = {}", a, underflow);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(underflow, false);
    println!("---------------------------");
}

fn BigUInt_checked_sub_uint()
{
    println!("BigUInt_checked_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = u512::one();
    let b = a.checked_sub_uint(1_u8);
    match b
    {
        Some(num) => {
            println!("{} - 1 = {}", a, num);
            assert_eq!(num.to_string(), "0");
        },
        None => {
            println!("{} - 1 = overflow", a);
        }
    }

    let c = a.checked_sub_uint(2_u8);
    match c
    {
        Some(num) => {
            println!("{} - 2 = {}", a, num);
        },
        None => {
            println!("{} - 2 = overflow", a);
            assert_eq!(c, None);
        }
    }

    let d = a.checked_sub_uint(3_u8);
    match d
    {
        Some(num) => {
            println!("{} - 3 = {}", a, num);
        },
        None => {
            println!("{} - 3 = overflow", a);
            assert_eq!(d, None);
        }
    }
    println!("---------------------------");
}

fn BigUInt_unchecked_sub_uint()
{
    println!("BigUInt_unchecked_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a = U64::one();
    let b = a.unchecked_sub_uint(1_u8);
    println!("{} - 1 = {}", a, b);
    assert_eq!(b.to_string(), "0");

    // It will panic.
    // let c = a.unchecked_add_uint(2_u8);
    println!("---------------------------");
}

fn BigUInt_saturating_sub_uint()
{
    println!("BigUInt_saturating_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u512::zero().wrapping_add_uint(2_u8);
    let b = a.saturating_sub_uint(1_u8);
    let c = a.saturating_sub_uint(2_u8);
    let d = a.saturating_sub_uint(3_u8);
    
    println!("{} - 1 = {}", a, b);
    assert_eq!(b.to_string(), "1");

    println!("{} - 2 = {}", a, c);
    assert_eq!(c.to_string(), "0");

    println!("{} - 3 = {}", a, d);
    assert_eq!(d.to_string(), "0");
    println!("---------------------------");
}

fn BigUInt_saturating_sub_assign_uint()
{
    println!("BigUInt_saturating_sub_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = U64::zero().wrapping_add_uint(2_u8);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "2");
    
    a.saturating_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "1");
    
    a.saturating_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "0");
    
    a.saturating_sub_assign_uint(1_u8);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "0");
    println!("---------------------------");
}

fn BigUInt_modular_sub_uint()
{
    println!("BigUInt_modular_sub_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let m = u256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a = u256::from_uint(2_u8);
    let b = a.modular_sub_uint(1_u8, &m);
    let c = a.modular_sub_uint(2_u8, &m);
    let d = a.modular_sub_uint(3_u8, &m);
    
    println!("{} - 1 = {}", a, b);
    assert_eq!(b.to_string(), "1");

    println!("{} - 2 = {}", a, c);
    assert_eq!(c.to_string(), "0");

    println!("{} - 3 = {}", a, d);
    assert_eq!(d.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    println!("---------------------------");
}

fn BigUInt_modular_sub_assign_uint()
{
    println!("BigUInt_modular_sub_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let m = U32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a = U32::from_uint(2_u8);

    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "2");
    
    a.modular_sub_assign_uint(1_u8, &m);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "1");
    
    a.modular_sub_assign_uint(1_u8, &m);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "0");
    
    a.modular_sub_assign_uint(1_u8, &m);
    println!("After a -= 1,\ta = {}", a);
    assert_eq!(a.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    println!("---------------------------");
}

fn BigUInt_abs_diff_uint()
{
    println!("BigUInt_abs_diff_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str1 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let num_str2 = "12345678_9ABCDEF0_12345678_9ABCDEF0";
    let num_str3 = "9900AABB_CCDDEEFF_9900AABB_CCDDEEFF";
    let num1 = u256::from_str_radix(num_str1, 16).unwrap();
    let num2 = u256::from_str_radix(num_str2, 16).unwrap();
    let num3 = u256::from_str_radix(num_str3, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;

    let a = num1.abs_diff_uint(num_uint);
    let b = num2.abs_diff_uint(num_uint);
    let c = num3.abs_diff_uint(num_uint);
    
    println!("| {} - {} | = {}", num1, num_uint, a);
    assert_eq!(a.to_string(), "115792089237316195423570985008687907853066609319396769656704041438214461985024");

    println!("| {} - {} | = {}", num2, num_uint, b);
    assert_eq!(b.to_string(), "179177489040527647888749252028162707471");

    println!("| {} - {} | = {}", num3, num_uint, c);
    assert_eq!(c.to_string(), "0");
    println!("---------------------------");
}


fn BigUInt_mul_uint()
{
    BigUInt_carrying_mul_uint();
    BigUInt_carrying_mul_assign_uint();
    BigUInt_widening_mul_uint();
    BigUInt_widening_mul_assign_uint();
    BigUInt_wrapping_mul_uint();
    BigUInt_wrapping_mul_assign_uint();
    BigUInt_overflowing_mul_uint();
    BigUInt_overflowing_mul_assign_uint();
    BigUInt_checked_mul_uint();
    BigUInt_unchecked_mul_uint();
    BigUInt_saturating_mul_uint();
    BigUInt_saturating_mul_assign_uint();
    BigUInt_modular_mul_uint();
    BigUInt_modular_mul_assign_uint();
}

fn BigUInt_carrying_mul_uint()
{
    println!("BigUInt_carrying_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_low = u256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_high = U32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    let b_uint = 225_u8;
    let (mut res_low, mut res_high) = a_low.carrying_mul_uint(b_uint, U32::zero());
    let (mut res_high, mut res_higher) = a_high.carrying_mul_uint(b_uint, res_high);

    println!("{}:{} X {} = {}:{}:{}", a_high, a_low, b_uint, res_higher, res_high, res_low);
    assert_eq!(res_higher.to_string(), "0");
    assert_eq!(res_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    assert_eq!(res_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    println!("---------------------------");
}

fn BigUInt_carrying_mul_assign_uint()
{
    println!("BigUInt_carrying_mul_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_low = U32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_high = u256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    let b_uint = 225_u8;

    println!("Originally,\ta_low = {}", a_low);
    assert_eq!(a_low.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084094");
    println!("Originally,\ta_high = {}\n", a_high);
    assert_eq!(a_high.to_string(), "75388281194656994643364900608409476801874298166903427690031858186486050853");
    
    let mut res_high = a_low.carrying_mul_assign_uint(b_uint, U32::zero());
    println!("After a_low.carrying_mul_assign_uint(225_u8, 0),\na_low = {}\n", a_low);
    assert_eq!(a_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");

    let mut res_higher = a_high.carrying_mul_assign_uint(b_uint, res_high);
    println!("After a_high.carrying_mul_assign_uint(225_u8, res_higher),\na_high = {}\nres_higher = {}", a_high, res_higher);
    assert_eq!(a_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    assert_eq!(res_higher.to_string(), "0");
    println!("---------------------------");
}

fn BigUInt_widening_mul_uint()
{
    println!("BigUInt_widening_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = u256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u8;
    let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);

    println!("{} X {} = {}:{}", a_biguint, b_uint, res_high, res_low);
    assert_eq!(res_high.to_string(), "1");
    assert_eq!(res_low.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    println!("---------------------------");
}

fn BigUInt_widening_mul_assign_uint()
{
    println!("BigUInt_widening_mul_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u8;

    println!("Originally,\ta_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    
    let mut res_high = a_biguint.widening_mul_assign_uint(b_uint);
    println!("After a_biguint.widening_mul_assign_uint(248_u8),\na_biguint = {}\nres_high = {}", a_biguint, res_high);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res_high.to_string(), "1");
    println!("---------------------------");
}

fn BigUInt_wrapping_mul_uint()
{
    println!("BigUInt_wrapping_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = u256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u16;
    let res = a_biguint.wrapping_mul_uint(b_uint);
    println!("{} X {} = {}", a_biguint, b_uint, res);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_wrapping_mul_assign_uint()
{
    println!("BigUInt_wrapping_mul_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u16;

    println!("Originally,\ta_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    
    a_biguint.wrapping_mul_assign_uint(b_uint);
    println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(a_biguint.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_overflowing_mul_uint()
{
    println!("BigUInt_overflowing_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = u256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u8;
    let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(overflow, true);
    println!("---------------------------");
}

fn BigUInt_overflowing_mul_assign_uint()
{
    println!("BigUInt_overflowing_mul_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u16;

    println!("Originally,\ta_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");

    let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(overflow, true);
    println!("---------------------------");
}

fn BigUInt_checked_mul_uint()
{
    println!("BigUInt_checked_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = u256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut b_uint = 248_u8;
    let mut res = a_biguint.checked_mul_uint(b_uint);
    match &res
    {
        Some(r) => { println!("{} X {} = {}", a_biguint, b_uint, r); },
        None => { println!("Overflow happend!"); },
    }
    assert_eq!(res, None);

    b_uint = 5_u8;
    res = a_biguint.checked_mul_uint(b_uint);
    match &res
    {
        Some(r) => { println!("{} X {} = {}", a_biguint, b_uint, r); },
        None => { println!("Overflow happend!"); },
    }
    assert_eq!(res.unwrap().to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    println!("---------------------------");
}

fn BigUInt_unchecked_mul_uint()
{
    println!("BigUInt_unchecked_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut res = a_biguint.unchecked_mul_uint(5_u8);
    println!("{} X {} = {}", a_biguint, 5_u8, res);
    assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");

    // It will panic.
    // res = a_biguint.unchecked_mul_uint(248_u8);
    println!("---------------------------");
}

fn BigUInt_saturating_mul_uint()
{
    println!("BigUInt_saturating_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = u256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut res = a_biguint.saturating_mul_uint(5_u8);
    println!("{} X {} = {}", a_biguint, 5_u8, res);
    assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");

    res = a_biguint.saturating_mul_uint(248_u8);
    println!("{} X {} = {}", a_biguint, 248_u8, res);
    assert_eq!(res.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(res, U32::max());
    println!("---------------------------");
}

fn BigUInt_saturating_mul_assign_uint()
{
    println!("BigUInt_saturating_mul_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = u256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut b_biguint = a_biguint.clone();

    println!("Originally, a_biguint = {}", a_biguint);
    a_biguint.saturating_mul_assign_uint(5_u8);
    println!("After a_biguint.saturating_mul_assign_uint(5_u8), a_biguint = {}", a_biguint);

    println!("Originally, b_biguint = {}", b_biguint);
    b_biguint.saturating_mul_assign_uint(248_u8);
    println!("After b_biguint.saturating_mul_assign_uint(248_u8), b_biguint = {}", b_biguint);
    assert_eq!(b_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(b_biguint, U32::max());
    println!("---------------------------");
}

fn BigUInt_modular_mul_uint()
{
    println!("BigUInt_modular_mul_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let m = U32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_biguint = u256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mut mul_uint = 5_u8;

    let mut res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {}", a_biguint, mul_uint, res);
    assert_eq!(res.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");

    mul_uint = 248_u8;
    res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {}", a_biguint, mul_uint, res);
    assert_eq!(res.to_string(), "7900830248540611730962937362798468648259453920500671345448848261116134680");
    println!("---------------------------");
}

fn BigUInt_modular_mul_assign_uint()
{
    println!("BigUInt_modular_mul_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let m = U32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_biguint = u256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mut b_biguint = a_biguint.clone();
    let mut mul_uint = 5_u8;

    println!("Originally, a_biguint = {}", a_biguint);
    a_biguint.modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    
    mul_uint = 248_u8;
    println!("Originally, b_biguint = {}", b_biguint);
    b_biguint.modular_mul_assign_uint(mul_uint, &m);
    println!("After b_biguint.modular_mul_assign_uint(mul_uint, &m), b_biguint = {}", b_biguint);
    assert_eq!(b_biguint.to_string(), "7900830248540611730962937362798468648259453920500671345448848261116134680");
    println!("---------------------------");
}

fn BigUInt_div_uint()
{
    BigUInt_divide_fully_uint();
    BigUInt_wrapping_div_uint();
    BigUInt_wrapping_div_assign_uint();
    BigUInt_overflowing_div_uint();
    BigUInt_overflowing_div_assign_uint();
    BigUInt_checked_div_uint();
    BigUInt_unchecked_div_uint();
    BigUInt_saturating_div_uint();
    BigUInt_saturating_div_assign_uint();
    BigUInt_modular_div_uint();
    BigUInt_modular_div_assign_uint();
}

fn BigUInt_divide_fully_uint()
{
    println!("BigUInt_divide_fully_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let (mut quotient, mut remainder) = dividend.divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_inifinity(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);

    divisor = 0_u8;
    (quotient, remainder) = dividend.divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient, u256::max());
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(quotient.is_overflow(), true);
    assert_eq!(quotient.is_inifinity(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_wrapping_div_uint()
{
    println!("BigUInt_wrapping_div_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut quotient = dividend.wrapping_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_inifinity(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);

    divisor = 0_u8;
    quotient = dividend.wrapping_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient, u256::max());
    assert_eq!(quotient.is_overflow(), true);
    assert_eq!(quotient.is_inifinity(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_wrapping_div_assign_uint()
{
    println!("BigUInt_wrapping_div_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    println!("Originally,\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 0_u8;
    a_biguint.wrapping_div_assign_uint(divisor);
    println!("After a_biguint.wrapping_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, U32::max());
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_inifinity(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_overflowing_div_uint()
{
    println!("BigUInt_overflowing_div_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let (mut quotient, mut overflow) = dividend.overflowing_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(overflow, false);
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_inifinity(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);

    divisor = 0_u8;
    (quotient, overflow) = dividend.overflowing_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient, u256::max());
    assert_eq!(overflow, true);
    assert_eq!(quotient.is_overflow(), true);
    assert_eq!(quotient.is_inifinity(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    println!("---------------------------");
}
fn BigUInt_overflowing_div_assign_uint()
{
    println!("BigUInt_overflowing_div_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u16;
    println!("Originally,\na_biguint = {}", a_biguint);
    let mut overflow = a_biguint.overflowing_div_assign_uint(divisor);
    println!("After a_biguint.overflowing_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 0_u16;
    overflow = a_biguint.overflowing_div_assign_uint(divisor);
    println!("After a_biguint.overflowing_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, U32::max());
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_inifinity(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_checked_div_uint()
{
    println!("BigUInt_checked_div_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut quotient = dividend.checked_div_uint(divisor);
    match quotient.clone()
    {
        Some(q) =>
            {
                println!("{} / {} = {}", dividend, divisor, q);
                assert_eq!(q.to_string(), "1419043551905275201680884938348044216837079832");
                assert_eq!(quotient.clone().unwrap().is_overflow(), false);
                assert_eq!(quotient.clone().unwrap().is_inifinity(), false);
                assert_eq!(quotient.clone().unwrap().is_divided_by_zero(), false);
            },
        None => { println!("Divided By Zero"); },
    }

    divisor = 0_u8;
    quotient = dividend.checked_div_uint(divisor);
    match quotient
    {
        Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
        None =>
            {
                println!("Divided By Zero");
                assert_eq!(quotient, None);
            },
    }
    println!("---------------------------");
}

fn BigUInt_unchecked_div_uint()
{
    println!("BigUInt_unchecked_div_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let dividend = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut quotient = dividend.unchecked_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_inifinity(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);

    divisor = 0_u8;
    // It will panic.
    // quotient = dividend.uchecked_div_uint(divisor);
    println!("---------------------------");
}

fn BigUInt_saturating_div_uint()
{
    println!("BigUInt_saturating_div_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut quotient = dividend.saturating_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_inifinity(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);

    divisor = 0_u8;
    quotient = dividend.saturating_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient, u256::max());
    assert_eq!(quotient.is_overflow(), true);
    assert_eq!(quotient.is_inifinity(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    println!("---------------------------")
}

fn BigUInt_saturating_div_assign_uint()
{
    println!("BigUInt_saturating_div_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u16;
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.saturating_div_assign_uint(divisor);
    println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 0_u16;
    a_biguint.saturating_div_assign_uint(divisor);
    println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, U32::max());
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_inifinity(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_modular_div_uint()
{
    println!("BigUInt_modular_div_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 128_u8;
    let modulo = u256::from_uint(100_u8);
    let mut quotient = dividend.modular_div_uint(divisor, &modulo);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "3");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_inifinity(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    
    divisor = 200_u8;
    quotient = dividend.modular_div_uint(divisor, &modulo);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient, u256::max());
    assert_eq!(quotient.is_overflow(), true);
    assert_eq!(quotient.is_inifinity(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_modular_div_assign_uint()
{
    println!("BigUInt_modular_div_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    
    let mut a_biguint = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 128_u8;
    let modulo = U32::from_uint(100_u8);
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "3");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 200_u8;
    a_biguint.modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, u256::max());
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_inifinity(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_rem_uint()
{
    BigUInt_wrapping_rem_uint();
    BigUInt_wrapping_rem_assign_uint();
    BigUInt_overflowing_rem_uint();
    BigUInt_overflowing_rem_assign_uint();
    BigUInt_checked_rem_uint();
    BigUInt_unchecked_rem_uint();
    BigUInt_saturating_rem_uint();
    BigUInt_saturating_rem_assign_uint();
    BigUInt_modular_rem_uint();
    BigUInt_modular_rem_assign_uint();
}

fn BigUInt_wrapping_rem_uint()
{
    println!("BigUInt_wrapping_rem_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut remainder = dividend.wrapping_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    divisor = 0_u8;
    remainder = dividend.wrapping_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder, 0);
    println!("---------------------------");
}

fn BigUInt_wrapping_rem_assign_uint()
{
    println!("BigUInt_wrapping_rem_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.wrapping_rem_assign_uint(divisor);
    println!("After a_biguint.wrapping_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 0_u8;
    a_biguint.wrapping_rem_assign_uint(divisor);
    println!("After a_biguint.wrapping_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, 0);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_overflowing_rem_uint()
{
    println!("BigUInt_overflowing_rem_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let (mut remainder, mut overflow) = dividend.overflowing_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder, 8);
    assert_eq!(overflow, false);

    divisor = 0_u8;
    (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder, 0);
    assert_eq!(overflow, false);
    println!("---------------------------");
}

fn BigUInt_overflowing_rem_assign_uint()
{
    println!("BigUInt_overflowing_rem_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u16;
    println!("Originally,\na_biguint = {}", a_biguint);
    let mut overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    println!("After a_biguint.overflowing_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 0_u16;
    overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    println!("After a_biguint.overflowing_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, u256::zero());
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_checked_rem_uint()
{
    println!("BigUInt_checked_rem_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let dividend = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut remainder = dividend.checked_rem_uint(divisor);
    match remainder
    {
        Some(r) =>
            {
                println!("{} % {} = {}", dividend, divisor, r);
                assert_eq!(r.to_string(), "8");
            },
        None => { println!("Divided By Zero"); },
    }

    divisor = 0_u8;
    remainder = dividend.checked_rem_uint(divisor);
    match remainder
    {
        Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
        None =>
            {
                println!("Divided By Zero");
                assert_eq!(remainder, None);
            },
    }
    println!("---------------------------");
}

fn BigUInt_unchecked_rem_uint()
{
    println!("BigUInt_unchecked_rem_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut remainder = dividend.unchecked_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    divisor = 0_u8;
    // It will panic.
    // remainder = dividend.unchecked_rem_uint(divisor);
    println!("---------------------------");
}

fn BigUInt_saturating_rem_uint()
{
    println!("BigUInt_saturating_rem_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u8;
    let mut remainder = dividend.saturating_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    divisor = 0_u8;
    remainder = dividend.saturating_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder, 0);
    println!("---------------------------");
}

fn BigUInt_saturating_rem_assign_uint()
{
    println!("BigUInt_saturating_rem_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 87_u16;
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.saturating_rem_assign_uint(divisor);
    println!("After a_biguint.saturating_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 0_u16;
    a_biguint.saturating_rem_assign_uint(divisor);
    println!("After a_biguint.saturating_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint, U32::zero());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_modular_rem_uint()
{
    println!("BigUInt_modular_rem_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = u256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 128_u8;
    let modulo = u256::from_uint(100_u8);
    let mut remainder = dividend.modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");
    
    divisor = 200_u8;
    remainder = dividend.modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    println!("---------------------------");
}

fn BigUInt_modular_rem_assign_uint()
{
    println!("BigUInt_modular_rem_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    
    let mut a_biguint = U32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let mut divisor = 128_u8;
    let modulo = U32::from_uint(100_u8);
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    divisor = 200_u8;
    a_biguint.modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_inifinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_next_multiple_uint()
{
    BigUInt_next_multiple_of_uint();
    BigUInt_next_multiple_of_assign_uint();
}

fn BigUInt_next_multiple_of_uint()
{
    println!("BigUInt_next_multiple_of_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = u256::from_str("123456789012345678901234567890123456789").unwrap();
    let mut num = 586478_u32;
    let mut multiple = a_biguint.next_multiple_of_uint(num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "123456789012345678901234567890123697594");
    assert_eq!(multiple.is_overflow(), false);

    a_biguint = u256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    multiple = a_biguint.next_multiple_of_uint(num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "448670");
    assert_eq!(multiple.is_overflow(), true);

    num = 0_u32;
    // It will panic.
    // multiple = a_biguint.next_multiple_of_uint(num);
    println!("---------------------------");
}

fn BigUInt_next_multiple_of_assign_uint()
{
    println!("BigUInt_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U32::from_str("123456789012345678901234567890123456789").unwrap();
    let mut num = 586478_u32;

    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}),\na_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123697594");
    assert_eq!(a_biguint.is_overflow(), false);

    a_biguint = U32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}),\na_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "448670");
    assert_eq!(a_biguint.is_overflow(), true);

    num = 0_u32;
    // It will panic.
    // a_biguint.next_multiple_of_assign_uint(num);
    println!("---------------------------");
}

fn BigUInt_exponentiation_logarithm_uint()
{
    BigUInt_pow_uint();
    BigUInt_pow_assign_uint();
    BigUInt_wrapping_pow_uint();
    BigUInt_wrapping_pow_assign_uint();
    BigUInt_overflowing_pow_uint();
    BigUInt_overflowing_pow_assign_uint();
    BigUInt_checked_pow_uint();
    BigUInt_unchecked_pow_uint();
    BigUInt_saturating_pow_uint();
    BigUInt_saturating_pow_assign_uint();
    BigUInt_modular_pow_uint();
    BigUInt_modular_pow_assign_uint();

    BigUInt_root_uint();
    BigUInt_root_assign_uint();
    BigUInt_wrapping_root_uint();
    BigUInt_wrapping_root_assign_uint();
    BigUInt_overflowing_root_uint();
    BigUInt_overflowing_root_assign_uint();
    BigUInt_checked_root_uint();
    BigUInt_unchecked_root_uint();
    BigUInt_saturating_root_uint();
    BigUInt_saturating_root_assign_uint();

    BigUInt_ilog_uint();
    BigUInt_ilog_assign_uint();
    BigUInt_wrapping_ilog_uint();
    BigUInt_wrapping_ilog_assign_uint();
    BigUInt_overflowing_ilog_uint();
    BigUInt_overflowing_ilog_assign_uint();
    BigUInt_checked_ilog_uint();
    BigUInt_unchecked_ilog_uint();
    BigUInt_saturating_ilog_uint();
    BigUInt_saturating_ilog_assign_uint();

    BigUInt_ilog2_uint();
    BigUInt_ilog2_assign_uint();
    BigUInt_wrapping_ilog2_uint();
    BigUInt_wrapping_ilog2_assign_uint();
    BigUInt_overflowing_ilog2_uint();
    BigUInt_overflowing_ilog2_assign_uint();
    BigUInt_checked_ilog2_uint();
    BigUInt_unchecked_ilog2_uint();
    BigUInt_saturating_ilog2_uint();
    BigUInt_saturating_ilog2_assign_uint();

    BigUInt_ilog10_uint();
    BigUInt_ilog10_assign_uint();
    BigUInt_wrapping_ilog10_uint();
    BigUInt_wrapping_ilog10_assign_uint();
    BigUInt_overflowing_ilog10_uint();
    BigUInt_overflowing_ilog10_assign_uint();
    BigUInt_checked_ilog10_uint();
    BigUInt_unchecked_ilog10_uint();
    BigUInt_saturating_ilog10_uint();
    BigUInt_saturating_ilog10_assign_uint();
}

fn BigUInt_pow_uint()
{
    println!("BigUInt_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U32::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut res = a_biguint.pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);

    exp = 100_u32;
    res = a_biguint.pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_pow_assign_uint()
{
    println!("BigUInt_pow_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = u256::from_uint(10_u8);
    let exp = 10_u8;

    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.pow_assign_uint(exp);
    println!("After a_biguint.pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "10000000000");
    assert_eq!(a_biguint.is_overflow(), false);

    a_biguint.pow_assign_uint(exp);
    println!("After a_biguint.pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_wrapping_pow_uint()
{
    println!("BigUInt_wrapping_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U32::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut res = a_biguint.wrapping_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);

    exp = 100_u32;
    res = a_biguint.wrapping_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_wrapping_pow_assign_uint()
{
    println!("BigUInt_wrapping_pow_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = u256::from_uint(10_u8);
    let exp = 10_u8;

    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.wrapping_pow_assign_uint(exp);
    println!("After a_biguint.wrapping_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "10000000000");
    assert_eq!(a_biguint.is_overflow(), false);

    a_biguint.wrapping_pow_assign_uint(exp);
    println!("After a_biguint.wrapping_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_overflowing_pow_uint()
{
    println!("BigUInt_overflowing_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U32::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut res = a_biguint.overflowing_pow_uint(exp);
    println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res.0, res.1);
    assert_eq!(res.0.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.1, false);
    assert_eq!(res.0.is_overflow(), false);

    exp = 100_u32;
    res = a_biguint.overflowing_pow_uint(exp);
    println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res.0, res.1);
    assert_eq!(res.0.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.1, true);
    assert_eq!(res.0.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_overflowing_pow_assign_uint()
{
    println!("BigUInt_overflowing_pow_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = u256::from_uint(10_u8);
    let mut exp = 30_u32;

    println!("Originally,\na_biguint = {}", a_biguint);
    let mut overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);

    exp = 3_u32;
    overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "51484102413631087777415798035541167055393351402420714880745735202410401366016");
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.is_overflow(), true);

    exp = 0_u32;
    overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_checked_pow_uint()
{
    println!("BigUInt_checked_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U32::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "1000000000000000000000000000000");
                assert_eq!(raised.is_overflow(), false);
            },
        None => { println!("Overflow"); }
    }

    exp = 100_u32;
    res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
        None => {
                println!("Overflow");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn BigUInt_unchecked_pow_uint()
{
    println!("BigUInt_unchecked_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U32::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut res = a_biguint.unchecked_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);

    exp = 100_u32;
    // It will panic.
    // res = a_biguint.unchecked_pow_uint(exp);
    println!("---------------------------");
}

fn BigUInt_saturating_pow_uint()
{
    println!("BigUInt_saturating_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    
    let a_biguint = u256::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut res = a_biguint.saturating_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    
    exp = 100_u32;
    res = a_biguint.saturating_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res, U32::max());
    assert_eq!(res.is_overflow(), false);
    println!("---------------------------");
}

fn BigUInt_saturating_pow_assign_uint()
{
    println!("BigUInt_saturating_pow_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let mut a_biguint = U32::from_uint(10_u8);
    let mut exp = 30_u32;
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.saturating_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);

    a_biguint.saturating_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint, U32::max());
    assert_eq!(a_biguint.is_overflow(), false);
    println!("---------------------------");
}

fn BigUInt_modular_pow_uint()
{
    println!("BigUInt_modular_pow_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    let a_biguint = u256::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut modulo = u256::halfmax();
    let mut res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    
    exp = 100_u32;
    res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    assert_eq!(res.is_overflow(), false);

    modulo.set_zero();
    res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    assert_eq!(res, u256::max());
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_inifinity(), true);
    assert_eq!(res.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_modular_pow_assign_uint()
{
    println!("BigUInt_modular_pow_assign_uint");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    
    let mut a_biguint = u256::from_uint(10_u8);
    let mut exp = 30_u32;
    let mut modulo = u256::halfmax();
    println!("Originally,\na_biguint = {}", a_biguint);
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);

    exp = 100_u32;
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    assert_eq!(a_biguint.is_overflow(), false);

    modulo.set_zero();
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint, u256::max());
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_inifinity(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    println!("---------------------------");
}

fn BigUInt_root_uint()
{
    println!("BigUInt_root_uint");
    println!("---------------------------");
}

fn BigUInt_root_assign_uint()
{
    println!("BigUInt_root_assign_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_root_uint()
{
    println!("BigUInt_wrapping_root_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_root_assign_uint()
{
    println!("BigUInt_wrapping_root_assign_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_root_uint()
{
    println!("BigUInt_overflowing_root_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_root_assign_uint()
{
    println!("BigUInt_overflowing_root_assign_uint");
    println!("---------------------------");
}

fn BigUInt_checked_root_uint()
{
    println!("BigUInt_checked_root_uint");
    println!("---------------------------");
}

fn BigUInt_unchecked_root_uint()
{
    println!("BigUInt_unchecked_root_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_root_uint()
{
    println!("BigUInt_saturating_root_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_root_assign_uint()
{
    println!("BigUInt_saturating_root_assign_uint");
    println!("---------------------------");
}

fn BigUInt_ilog_uint()
{
    println!("BigUInt_ilog_uint");
    println!("---------------------------");
}

fn BigUInt_ilog_assign_uint()
{
    println!("BigUInt_ilog_assign_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_ilog_uint()
{
    println!("BigUInt_wrapping_ilog_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_ilog_assign_uint()
{
    println!("BigUInt_wrapping_ilog_assign_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_ilog_uint()
{
    println!("BigUInt_overflowing_ilog_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_ilog_assign_uint()
{
    println!("BigUInt_overflowing_ilog_assign_uint");
    println!("---------------------------");
}

fn BigUInt_checked_ilog_uint()
{
    println!("BigUInt_checked_ilog_uint");
    println!("---------------------------");
}

fn BigUInt_unchecked_ilog_uint()
{
    println!("BigUInt_unchecked_ilog_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_ilog_uint()
{
    println!("BigUInt_saturating_ilog_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_ilog_assign_uint()
{
    println!("BigUInt_saturating_ilog_assign_uint");
    println!("---------------------------");
}


fn BigUInt_ilog2_uint()
{
    println!("BigUInt_ilog2_uint");
    println!("---------------------------");
}

fn BigUInt_ilog2_assign_uint()
{
    println!("BigUInt_ilog2_assign_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_ilog2_uint()
{
    println!("BigUInt_wrapping_ilog2_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_ilog2_assign_uint()
{
    println!("BigUInt_wrapping_ilog2_assign_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_ilog2_uint()
{
    println!("BigUInt_overflowing_ilog2_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_ilog2_assign_uint()
{
    println!("BigUInt_overflowing_ilog2_assign_uint");
    println!("---------------------------");
}

fn BigUInt_checked_ilog2_uint()
{
    println!("BigUInt_checked_ilog2_uint");
    println!("---------------------------");
}

fn BigUInt_unchecked_ilog2_uint()
{
    println!("BigUInt_unchecked_ilog2_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_ilog2_uint()
{
    println!("BigUInt_saturating_ilog2_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_ilog2_assign_uint()
{
    println!("BigUInt_saturating_ilog2_assign_uint");
    println!("---------------------------");
}

fn BigUInt_ilog10_uint()
{
    println!("BigUInt_ilog10_uint");
    println!("---------------------------");
}

fn BigUInt_ilog10_assign_uint()
{
    println!("BigUInt_ilog10_assign_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_ilog10_uint()
{
    println!("BigUInt_wrapping_ilog10_uint");
    println!("---------------------------");
}

fn BigUInt_wrapping_ilog10_assign_uint()
{
    println!("BigUInt_wrapping_ilog10_assign_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_ilog10_uint()
{
    println!("BigUInt_overflowing_ilog10_uint");
    println!("---------------------------");
}

fn BigUInt_overflowing_ilog10_assign_uint()
{
    println!("BigUInt_overflowing_ilog10_assign_uint");
    println!("---------------------------");
}

fn BigUInt_checked_ilog10_uint()
{
    println!("BigUInt_checked_ilog10_uint");
    println!("---------------------------");
}

fn BigUInt_unchecked_ilog10_uint()
{
    println!("BigUInt_unchecked_ilog10_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_ilog10_uint()
{
    println!("BigUInt_saturating_ilog10_uint");
    println!("---------------------------");
}

fn BigUInt_saturating_ilog10_assign_uint()
{
    println!("BigUInt_saturating_ilog10_assign_uint");
    println!("---------------------------");
}

fn BigUInt_arithmatic_operation_biguint()
{
    BigUInt_add();
    BigUInt_sub();
    BigUInt_mul();
    BigUInt_div();
    BigUInt_rem();
    BigUInt_next_multiple();
    BigUInt_exponentiation_logarithm();
}

fn BigUInt_add()
{
    BigUInt_carrying_add();
    BigUInt_carrying_add_assign();
    BigUInt_wrapping_add();
    BigUInt_wrapping_add_assign();
    BigUInt_overflowing_add();
    BigUInt_overflowing_add_assign();
    BigUInt_checked_add();
    BigUInt_unchecked_add();
    BigUInt_saturating_add();
    BigUInt_saturating_add_assign();
    BigUInt_modular_add();
    BigUInt_modular_add_assign();
}

fn BigUInt_carrying_add()
{
    println!("BigUInt_carrying_add");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_hi = u256::from_str_radix("15D5_ECE4_41DB_7709_BA44_8C40_0DCF_7160_3CD4_F7FF_F0CF_476F_33FD_438B_0E1D_2086", 16).unwrap();
    let a_lo = u256::from_str_radix("C9B4_EF7B_BBC9_F60E_45CB_EE41_B567_A641_7D69_A0EC_05F7_65A7_F81B_5C91_72DC_BAC0", 16).unwrap();
    let b_hi = u256::from_str_radix("274_DDD9_4DAA_9405_B621_6BCA_AF43_78E3_0FA6_1D7D_86F4_0D17_2C18_A01C_80F9_DB46", 16).unwrap();
    let b_lo = u256::from_str_radix("DF8A_DC5F_FDA5_6D18_0010_7A81_C337_17A1_BA3E_98EB_F6C6_AD17_2C18_A01C_80F9_DB46", 16).unwrap();

    let (c_lo, carry) = a_lo.carrying_add(&b_lo, false);
    let (c_hi, overflow) = a_hi.carrying_add(&b_hi, carry);

    println!("{}:{} + {}:{} = {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);

    assert_eq!(c_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "184A_CABD_8F86_0B0F_7065_F80A_BD12_EA43_4C7B_157D_77C3_5486_6015_E3A7_8F16_FBCD");
    assert_eq!(c_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "A93F_CBDB_B96F_6326_45DC_68C3_789E_BDE3_37A8_39D7_FCBE_12BF_2433_FCAD_F3D6_9606");
    assert_eq!(carry, true);
    assert_eq!(c_lo.is_overflow(), true);
    assert_eq!(overflow, false);
    assert_eq!(c_hi.is_overflow(), false);
    println!("---------------------------");
}

fn BigUInt_carrying_add_assign()
{
    println!("BigUInt_carrying_add_assign");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_hi = u256::from_str_radix("15D5_ECE4_41DB_7709_BA44_8C40_0DCF_7160_3CD4_F7FF_F0CF_476F_33FD_438B_0E1D_2086", 16).unwrap();
    let mut a_lo = u256::from_str_radix("C9B4_EF7B_BBC9_F60E_45CB_EE41_B567_A641_7D69_A0EC_05F7_65A7_F81B_5C91_72DC_BAC0", 16).unwrap();
    let b_hi = u256::from_str_radix("274_DDD9_4DAA_9405_B621_6BCA_AF43_78E3_0FA6_1D7D_86F4_0D17_2C18_A01C_80F9_DB46", 16).unwrap();
    let b_lo = u256::from_str_radix("DF8A_DC5F_FDA5_6D18_0010_7A81_C337_17A1_BA3E_98EB_F6C6_AD17_2C18_A01C_80F9_DB46", 16).unwrap();
    let c_hi = u256::from(1_u8);
    let c_lo = u256::from(1_u8);

    print!("{}:{} + {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    let mut carry = a_lo.carrying_add_assign(&b_lo, false);
    let mut overflow = a_hi.carrying_add_assign(&b_hi, carry);
    println!(" = {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);

    assert_eq!(a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "184A_CABD_8F86_0B0F_7065_F80A_BD12_EA43_4C7B_157D_77C3_5486_6015_E3A7_8F16_FBCD");
    assert_eq!(a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "A93F_CBDB_B96F_6326_45DC_68C3_789E_BDE3_37A8_39D7_FCBE_12BF_2433_FCAD_F3D6_9606");
    assert_eq!(carry, true);
    assert_eq!(a_lo.is_overflow(), true);
    assert_eq!(overflow, false);
    assert_eq!(a_hi.is_overflow(), false);

    print!("{}:{} + {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    carry = a_lo.carrying_add_assign(&c_lo, false);
    overflow = a_hi.carrying_add_assign(&c_hi, carry);
    println!(" = {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);
    assert_eq!(a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "184A_CABD_8F86_0B0F_7065_F80A_BD12_EA43_4C7B_157D_77C3_5486_6015_E3A7_8F16_FBCE");
    assert_eq!(a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "A93F_CBDB_B96F_6326_45DC_68C3_789E_BDE3_37A8_39D7_FCBE_12BF_2433_FCAD_F3D6_9607");
    assert_eq!(carry, false);
    assert_eq!(a_lo.is_overflow(), true);
    assert_eq!(overflow, false);
    assert_eq!(a_hi.is_overflow(), false);
    println!("---------------------------");
}

fn BigUInt_wrapping_add()
{
    println!("BigUInt_wrapping_add___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let zero = u512::zero();
    let one = u512::one();
    let two = u512::from(2_u8);
    let three = u512::from(3_u8);
    let a = u512::max().wrapping_sub(&one);
    let b = a.wrapping_add(&one);
    let c = a.wrapping_add(&two);
    let d = a.wrapping_add(&three);

    println!("{} + 1 = {}", a, b);
    assert_eq!(b, u512::max());

    println!("{} + 2 = {}", a, c);
    assert_eq!(c, zero);

    println!("{} + 3 = {}", a, d);
    assert_eq!(d, one);
    println!("---------------------------");
}

fn BigUInt_wrapping_add_assign()
{
    println!("BigUInt_wrapping_add_assign()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let zero = u512::zero();
    let one = u512::one();

    let mut a = u512::max().wrapping_sub(&one);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");

    a.wrapping_add_assign(&one);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, u512::max());

    a.wrapping_add_assign(&one);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, zero);

    a.wrapping_add_assign(&one);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, one);
    println!("---------------------------");
}

fn BigUInt_overflowing_add()
{
    println!("BigUInt_overflowing_add()");
    println!("---------------------------");
}

fn BigUInt_overflowing_add_assign()
{
    println!("BigUInt_overflowing_add_assign()");
    println!("---------------------------");
}

fn BigUInt_checked_add()
{
    println!("BigUInt_checked_add()");
    println!("---------------------------");
}

fn BigUInt_unchecked_add()
{
    println!("BigUInt_unchecked_add()");
    println!("---------------------------");
}

fn BigUInt_saturating_add()
{
    println!("BigUInt_saturating_add()");
    println!("---------------------------");
}

fn BigUInt_saturating_add_assign()
{
    println!("BigUInt_saturating_add_assign()");
    println!("---------------------------");
}

fn BigUInt_modular_add()
{
    println!("BigUInt_modular_add()");
    println!("---------------------------");
}

fn BigUInt_modular_add_assign()
{
    println!("BigUInt_modular_add_assign()");
    println!("---------------------------");
}

fn BigUInt_sub()
{
}

fn BigUInt_mul()
{
}

fn BigUInt_div()
{
}

fn BigUInt_rem()
{
}

fn BigUInt_next_multiple()
{
}

fn BigUInt_exponentiation_logarithm()
{
}


fn BigUInt_abs_diff___main()
{
    println!("BigUInt_abs_diff___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let a = u256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    let b = u256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    let c = a.abs_diff(&b);
    let d = b.abs_diff(&a);
    println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c);
    println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d);
    assert_eq!(c, u256::from_str("500000000500000000500000000500000000").unwrap());
    assert_eq!(d, u256::from_str("500000000500000000500000000500000000").unwrap());
    println!("---------------------------");
}

fn BigUInt_exponentiation_logarithm_biguint___main()
{
}

fn BigUInt_pow_uint___main()
{
    println!("BigUInt_pow_uint___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(123_u8);

    // normal exponentiation
    let b = a.pow_uint(36_u8);
    println!("123 ** 36 = {}", b);
    assert_eq!(b.to_string(), "1724185592748300222303045014791251528772289498837076631331177393773983461361");

    // wrapping (modular) exponentiation
    let c = a.pow_uint(37_u8);
    println!("123 ** 37 = {}", c);
    assert_eq!(c.to_string(), "96282738670724731919703551810636030185721623691319861614277235426286836107467");

    // evidence of wrapping (modular) exponentiation
    assert_eq!(b.is_overflow(), false);
    assert_eq!(c.is_overflow(), true);
    println!("---------------------------");
}


fn BigUInt_pow_assign_uint___main()
{
    println!("BigUInt_pow_assign_uint___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = u256::from_uint(123_u8);
    let mut b = a.clone();

    // normal exponentiation
    a.pow_assign_uint(36_u8);
    println!("123 ** 36 = {}", a);
    assert_eq!(a.to_string(), "1724185592748300222303045014791251528772289498837076631331177393773983461361");

    // wrapping (modular) exponentiation
    b.pow_assign_uint(37_u8);
    println!("123 ** 37 = {}", b);
    assert_eq!(b.to_string(), "96282738670724731919703551810636030185721623691319861614277235426286836107467");

    // evidence of wrapping (modular) exponentiation
    assert_eq!(a.is_overflow(), false);
    assert_eq!(b.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_pow___main()
{
    println!("BigUInt_pow___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(234_u8);
    let mut exp = u256::from_uint(34_u8);

    // normal exponentiation
    let b = a.pow(&exp);
    println!("234 ** 34 = {}", b);
    assert_eq!(b.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    println!("{}", b.is_overflow());
    // wrapping (modular) exponentiation
    exp += 1;
    let c = a.pow(&exp);
    println!("234 ** 35 = {}", c);
    assert_eq!(c.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");

    // evidence of wrapping (modular) exponentiation
    assert!(b > c);
    println!("---------------------------");
}

fn BigUInt_pow_assign___main()
{
    println!("BigUInt_pow_assign___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = u256::from_uint(234_u8);
    let mut exp = u256::from_uint(34_u8);

    // normal exponentiation
    a.pow_assign(&exp);
    println!("234 ** 34 = {}", a);
    assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    println!("{}", a.is_overflow());
    // wrapping (modular) exponentiation
    let old = a.clone();
    a = u256::from_uint(234_u8);
    exp += 1;
    a.pow_assign(&exp);
    println!("234 ** 35 = {}", a);
    assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");

    // evidence of wrapping (modular) exponentiation
    assert!(old > a);
    println!("---------------------------");
}

fn BigUInt_bit_operation___main()
{

}

fn BigUInt_conversion___main()
{

}

fn BigUInt_flag_manipulation___main()
{

}


pub fn find_maximum()
{
    println!("find_maximum()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(123_u8);
    let mut exp = u256::from_uint(1_u8);
    loop {
        let b = a.pow(&exp);
        if b.is_overflow()
        {
            println!("Maximum i is {}", exp);
            break;
        }
        exp.wrapping_add_assign_uint(1_u8);
    }
    println!("---------------------------");
}

pub fn Test()
{
    println!("Test()");
    use Cryptocol::number::*;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = 128_u8;
    let b = a << 1;
    println!("b = {}", b);
    let p = u256::from_uint(12345678901234567890123456789_u128);
    let q = u256::from_uint(12345678901234567890_u128);
    let r = p.gcd(&q);

    println!("{} , {} => {}", p, q, r);

    let a = u256::from_uint(254_u8);
    let b = u256::from_uint(123_u8);
    let c = a.divide_fully(&b);
    let d = a.divide_fully_uint(123_u8);
    let aa = LongerUnion::new_with(254_u128);
    let bb = LongerUnion::new_with(123_u128);

    let cc = aa % bb;

    println!("c: {}  {}", c.0, c.1);
    println!("d: {}  {}", d.0, d.1);
    println!("{}", cc);

    let e = a.divide_fully_uint(4_u8);
    println!("{:?} {:?}", e.0, e.1);

    println!("a == b {}", a == b);
    println!("a != b {}", a != b);
    println!("a > b {}", a > b);
    println!("a >= b {}", a >= b);
    println!("a < b {}", a < b);
    println!("a <= b {}", a <= b);
}




fn f()
{
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    let divisor = 87_u8;
    let dividend = 1234567890157589425462369896584689254_u128;
    let dd = u256::from_uint(dividend);
    let (quotient, remainder) = dd.divide_fully_uint(divisor);
    println!("{} - {}", quotient, remainder);
    let (quotient, remainder) = dd.divide_fully(&u256::from_uint(divisor));
    println!("{} - {}", quotient, remainder);

}

/*
fn t_1024()
{
    define_utypes_with!(u128);
    let a = u1024::random();
    println!("{} 비트짜리 난수: {}", 1024, a);
    let b = u1024::from(1_u128);
    println!("{} 비트짜리 1: {}", 1024, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}

fn t_2048()
{
    define_utypes_with!(u128);
    let a = u2048::random();
    println!("{} 비트짜리 난수: {}", 2048, a);
    let b = u2048::from(1_u128);
    println!("{} 비트짜리 1: {}", 2048, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}

fn t_4096()
{
    define_utypes_with!(u128);
    let a = u4096::random();
    println!("{} 비트짜리 난수: {}", 4096, a);
    let b = u4096::from(1_u128);
    println!("{} 비트짜리 1: {}", 4096, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}



fn func<T: Uint + Add<Output = T>>(lhs: T, rhs: T) -> T
{
    lhs + rhs
}
fn func2<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_add(rhs)
}

fn main()
{



    let a = 100;
    let b = a % -3;
    let c = "123456789012".parse::<u256>().unwrap();
    let e = c.to_string_with_radix_and_stride(10, 4);
    let d: u128 = c.into_u128();
    println!("a = {}, b = {}, c = {}, e = {}", a, b, c, e);
    let a = "123_4566".parse::<u256>().unwrap();
    println!("a = {}", a);
    let ss = UShort { byte: [101, 100] };
    unsafe { println!("ss.short = {}", ss.ushort ); }
    println!("{}", (25700_u16 + 25800_u16));

    // a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    let a_high = 100_u8;
    let a_low = 101_u8;
    // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    let b_high = 100_u8;
    let b_low = 200_u8;
    // c: u16 === (c_high, c_low)
    let c_high: u8;
    let c_low: u8;
    let mut carry: bool;
    // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    println!("{}-{}, {}", c_high, c_low, carry);
    assert_eq!(c_high, 201);
    assert_eq!(c_low, 45);
    assert_eq!(carry, false);

    let d_high: u128;
    let d_low: u128;
    let e = BigUInt::<u128, 2>::from_array(&[6789012345678919134, 12345678901234569124]);
    println!("big = {}", e);
    (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    println!("{}-{}, {}", d_high, d_low, carry);
    assert_eq!(d_high, 12345678901234569124);
    assert_eq!(d_low, 6789012345678919134);
    assert_eq!(carry, false);
}

fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
{
    let mut carry = false;
    let mut sum_high: T;
    let mut sum_low: T;
    (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    (sum_high, sum_low, carry)
}

fn main()
{
    let a = func(50_u128, 4_u128);
    println!("50 + 4 = {}", a);
    assert_eq!(a, 54_u128);

    let b = func2(u8::MAX, u8::MAX);
    println!("{} * 15_u64 = {}", u128::MAX, b);
    assert_eq!(b, 254_u8);
    
    // u256::new();
    // let a = 100_u8;
    // let b = 100_u8;
    // let c = func(a, b);
    // let d = func(c, 57);
    // println!("a + b = {}", c);
    // println!("c + 57 = {}", d);
    // assert_eq!(c, 200_u8);
    // assert_eq!(d, 1_u8);
    
    let mut a = u256::from_string_with_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let b = u256::from_string_with_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    let d = u256::max();
    let c = !a | a;
    println!("c = {}", c.to_string_with_radix(2));
    assert_eq!(c, u256::max());

    // let mut sum = u1024::new();
    // sum.set_max();
    // println!("sum = {}", sum);

    // let mut a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    // println!("{}", a);
    // a >>= 2;
    // println!("a = {}\n{}", a, a.is_underflow());
    // assert_eq!(a.is_underflow(), true);
}
*/