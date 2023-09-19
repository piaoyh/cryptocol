// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]
#![allow(non_camel_case_types)]
/*

use std::ops::*;
use std::convert::*;
use std::mem::size_of;
use std::str::FromStr;

use Cryptocol::number::*;
use Cryptocol::define_utypes_with;
use Cryptocol::define_Utypes_with_utypes;
use Cryptocol::define_utypes_with_u128;
use Cryptocol::define_utypes_with_u64;
use Cryptocol::define_utypes_with_u32;
use Cryptocol::define_utypes_with_u16;
use Cryptocol::define_utypes_with_u8;
*/


pub fn test_main_BigUInt()
{
//    BigUInt_quick_start___main();
//    BigUInt_constructors___main();
//    BigUInt_random_number___main();   // Prime number related methods not yet finished
//    BigUInt_get_size___main();
    BigUInt_get_set_check___main();
    // BigUInt_carrying_add___main();
    // BigUInt_carrying_add_assign___main();
    //BigUInt_wrapping_add___main();
    //BigUInt_wrapping_add_assign___main();

    // BigUInt_abs_diff___main();

    // BigUInt_pow_uint___main();
    // BigUInt_pow_assign_uint___main();
    // BigUInt_pow___main();
    // BigUInt_pow_assign___main();
}

fn BigUInt_quick_start___main()
{
    BigUInt_quick_start1___main();
    BigUInt_quick_start2___main();
    BigUInt_performance_test();
}

fn BigUInt_quick_start1___main()
{
    println!("fn BigUInt_quick_start1___main()");
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
}

fn BigUInt_quick_start2___main()
{
    println!("fn BigUInt_quick_start2___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    
    define_utypes_with!(u128);

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
}

macro_rules! performance
{
    ($t:ty, $b:expr, $ti:expr, $f:expr) => {
        let mut sum = <$t>::zero();
        let a = <$t>::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
        let now: SystemTime;
        match $f
        {
            0 => {
                now = SystemTime::now();
                for _ in 0..100_0000
                    { sum.wrapping_add_assign(&a); }
            },
            1 => {
                now = SystemTime::now();
                for _ in 0..100_0000
                    { sum.wrapping_sub_assign(&a); }
            },
            2 => {
                now = SystemTime::now();
                for _ in 0..100_0000
                    { sum.wrapping_mul_assign(&a); }
            },
            _ => {
                now = SystemTime::now();
                for _ in 0..100_0000
                    { sum.wrapping_div_assign(&a); }
            },
        }
        match now.elapsed()
        {
            Ok(elapsed) => {
                $ti = elapsed.as_micros();
                println!("{}-based: {} usec", $b, $ti);
            },
            Err(e) => { println!("{}", e); },
        }
    }
}

fn BigUInt_performance_test()
{
    println!("BigUInt_performance_test");
    use std::time::SystemTime;
    use Cryptocol::number::*;

    let mut ti = [0_u128; 5];   // How many microseconds
    let dt = ["u128", "u64", "u32", "u16", "u8"];
    let op = ["addition", "subtraction", "multplication", "division" ];

    for operator in 0..4
    {
        println!("=== {} ===", op[operator]);
        performance!(u1024_with_u128, dt[0], ti[0], operator);
        performance!(u1024_with_u64, dt[1], ti[1], operator);
        performance!(u1024_with_u32, dt[2], ti[2], operator);
        performance!(u1024_with_u16, dt[3], ti[3], operator);
        performance!(u1024_with_u8, dt[4], ti[4], operator);
    
        let mut fastest = 0;
        for i in 1..5
        {
            if ti[fastest] > ti[i]
                { fastest = i; }
        }
        println!("The fastest one is {}.\n", dt[fastest]);

        #[cfg(debug_assertions)]
        assert_eq!(fastest, 0); // It means u128 shows the best performance most of the time.

        #[cfg(not(debug_assertions))]
        if operator < 2
            { assert_eq!(fastest, 0); } // It means u128 shows the best performance.
        else
            { assert_eq!(fastest, 1); } // It means u64 shows the best performance most of the time.
    }
}

fn BigUInt_constructors___main()
{
    BigUInt_new___main();
    BigUInt_zero___main();
    BigUInt_one___main();
    BigUInt_max___main();
    BigUInt_submax___main();
    BigUInt_halfmax___main();
    BigUInt_from_uint___main();
    BigUInt_from_array___main();
    BigUInt_from_biguint___main();
    BigUInt_from_be___main();
    BigUInt_from_be_bytes___main();
    BigUInt_from_le___main();
    BigUInt_from_le_bytes___main();
    BigUInt_from_str_radix___main();
    BigUInt_generate_check_bits___main();
}

fn BigUInt_new___main()
{
    println!("fn BigUInt_new___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let obj = u256::new();
    let zero = u256::zero();
    println!("obj = {}", obj);
    assert_eq!(obj, zero);
}

fn BigUInt_zero___main()
{
    println!("fn BigUInt_zero___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let zero = u256::zero();
    let obj = u256::new();
    println!("zero = {}", zero);
    assert_eq!(zero, obj);
}

fn BigUInt_one___main()
{
    println!("fn BigUInt_one___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let one = u256::one();
    let mut obj = u256::new();
    obj.set_uint(1_u32);
    println!("one = {}", one);
    assert_eq!(one, obj);
}

fn BigUInt_max___main()
{
    println!("BigUInt_max___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let maximum = u256::max();
    println!("maximum =\t{}", maximum);
    assert_eq!(maximum, u256::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap());
    println!("---------------------------");
}

fn BigUInt_submax___main()
{
    println!("BigUInt_submax___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let half = u256::submax(128_usize);
    println!("half maximum = \t{}", half);
    assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    println!("---------------------------");
}

fn BigUInt_halfmax___main()
{
    println!("BigUInt_halfmax___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let half = u256::halfmax();
    println!("half maximum = \t{}", half);
    assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    println!("---------------------------");
}

fn BigUInt_from_uint___main()
{
    BigUInt_from_uint_u8___main();
    BigUInt_from_uint_u16___main();
    BigUInt_from_uint_u32___main();
    BigUInt_from_uint_u64___main();
    BigUInt_from_uint_u128___main();
    BigUInt_from_uint_usize___main();
}

fn BigUInt_from_uint_u8___main()
{
    println!("BigUInt_from_uint_u8___main");
    use Cryptocol::define_utypes_with_u16;
    define_utypes_with_u16!();
     
    let aa = u512::from_uint(123_u8);
    println!("aa = {}", aa);
    assert_eq!(aa.into_u8(), 123_u8);
    println!("---------------------------");
}

fn BigUInt_from_uint_u16___main()
{
    println!("BigUInt_from_uint_u16___main");
    use Cryptocol::define_utypes_with_u64;
    define_utypes_with_u64!();
     
    let bb = u512::from_uint(12345_u16);
    println!("bb = {}", bb);
    assert_eq!(bb.into_u16(), 12345_u16);
    println!("---------------------------");
}

fn BigUInt_from_uint_u32___main()
{
    println!("BigUInt_from_uint_u32___main");
    use Cryptocol::define_utypes_with_u8;
    define_utypes_with_u8!();
     
    let cc = u512::from_uint(1234567890_u32);
    println!("cc = {}", cc);
    assert_eq!(cc.into_u32(), 1234567890_u32);
    println!("---------------------------");
}

fn BigUInt_from_uint_u64___main()
{
    println!("BigUInt_from_uint_u64___main");
    use Cryptocol::define_utypes_with_u128;
    define_utypes_with_u128!();
     
    let dd = u512::from_uint(12345678901234567890_u64);
    println!("dd = {}", dd);
    assert_eq!(dd.into_u64(), 12345678901234567890_u64);
    println!("---------------------------");
}

fn BigUInt_from_uint_u128___main()
{
    println!("BigUInt_from_uint_u128___main");
    use Cryptocol::define_utypes_with_u128;
    define_utypes_with_u128!();
     
    let ee = u512::from_uint(123456789012345678901234567890123456789_u128);
    println!("ee = {}", ee);
    assert_eq!(ee.into_u128(), 123456789012345678901234567890123456789_u128);
    println!("---------------------------");
}

fn BigUInt_from_uint_usize___main()
{
    println!("BigUInt_from_uint_usize___main");
    use Cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let ff = u512::from_uint(123_usize);
    println!("ff = {}", ff);

    assert_eq!(ff.into_usize(), 123_usize);
    println!("---------------------------");
}

fn BigUInt_from_array___main()
{
    println!("BigUInt_from_array___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let big_num = u256::from_array(&[1_u8;32]);
    println!("big_num = {}", big_num.to_string_with_radix(2).unwrap());
    assert_eq!(big_num, u256::from_str_radix("00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001", 2).unwrap());
    println!("---------------------------");
}

fn BigUInt_from_biguint___main()
{
    BigUInt_from_biguint_same_length___main();
    BigUInt_from_biguint_shorter_length___main();
    BigUInt_from_biguint_longer_length___main();
}

fn BigUInt_from_biguint_same_length___main()
{
    println!("BigUInt_from_biguint_same_length___main");
    use std::str::FromStr;
    use Cryptocol::number::*;

    let a = u256_with_u8::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = u256_with_u16::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a.to_string(), b.to_string());
    println!("---------------------------");
}

fn BigUInt_from_biguint_shorter_length___main()
{
    println!("BigUInt_from_biguint_shorter_length___main");
    use std::str::FromStr;
    use Cryptocol::number::*;

    let a = u256_with_u8::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = u512_with_u16::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a.to_string(), b.to_string());
    println!("---------------------------");
}

fn BigUInt_from_biguint_longer_length___main()
{
    println!("BigUInt_from_biguint_longer_length___main");
    use std::str::FromStr;
    use Cryptocol::number::*;

    let a = u512_with_u8::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = u256_with_u16::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a.to_string(), b.to_string());
    println!("---------------------------");
}

fn BigUInt_from_be___main()
{
    println!("BigUInt_from_be___main");
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

fn BigUInt_from_be_bytes___main()
{
    println!("BigUInt_from_be___main");
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

fn BigUInt_from_le___main()
{
    println!("BigUInt_from_le___main");
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

fn BigUInt_from_le_bytes___main()
{
    println!("BigUInt_from_le_bytes___main");
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

fn BigUInt_from_str_radix___main()
{
    println!("BigUInt_from_str_radix___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    let a = u512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16).unwrap();
    println!("a = {}", a);
    assert_eq!(a.to_string(), "953444119462584670231660883005169236350945453535049253076624239367818227875140724454335257332337691463184490358643394140772086144551847644877923949534960");
    println!("---------------------------");
}

fn BigUInt_generate_check_bits___main()
{
    println!("BigUInt_generate_check_bits___main");
    use Cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let a_0 = u256::generate_check_bits(0);
    println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    
    let a_12 = u256::generate_check_bits(12);
    println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");

    let a_255 = u256::generate_check_bits(255);
    println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");

    // It will panic!
    // let a_256 = u256::generate_check_bits(256);
    println!("---------------------------");
}

fn BigUInt_random_number___main()
{
    BigUInt_random___main();
    BigUInt_random_odd___main();
    BigUInt_random_less_than___main();
    BigUInt_random_odd_less_than___main();
    BigUInt_random_with_MSB_set___main();
    BigUInt_random_odd_with_MSB_set___main();
    BigUInt_random_prime_using_Miller_Rabin___main();
    BigUInt_randomize___main();
    BigUInt_is_prime_using_Miller_Rabin___main();
}

fn BigUInt_random___main()
{
    println!("BigUInt_random___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    println!("Random Number: {}", u1024::random());
    println!("---------------------------");
}

fn BigUInt_random_odd___main()
{
    println!("BigUInt_random_odd___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let r = u1024::random_odd();
    println!("Random Odd Number: {}", r);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn BigUInt_random_less_than___main()
{
    println!("BigUInt_random_less_than___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    let r = u1024::random_less_than(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("---------------------------");
}

fn BigUInt_random_odd_less_than___main()
{
    println!("BigUInt_random_odd_less_than___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    let r = u1024::random_odd_less_than(&ceiling);
    println!("Random Odd Number less than {} is\n{}", ceiling, u1024::random_odd_less_than(&ceiling));
    assert!(r < ceiling);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn BigUInt_random_with_MSB_set___main()
{
    println!("BigUInt_random_with_MSB_set___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let num = u1024::random_with_MSB_set();
    println!("Random Number = {}", u1024::random());
    println!("1024-bit Random Number = {}", num);
    assert!(num > u1024::submax(1023));
    println!("---------------------------");
}

fn BigUInt_random_odd_with_MSB_set___main()
{
    println!("BigUInt_random_with_MSB_set___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num = u1024::random_odd_with_MSB_set();
    println!("Random Number = {}", u1024::random());
    println!("1024-bit Random Odd Number = {}", num);
    assert!(num > u1024::submax(1023));
    assert!(num.is_odd());
    println!("---------------------------");
}

fn BigUInt_random_prime_using_Miller_Rabin___main()
{
    println!("BigUInt_random_prime_using_Miller_Rabin___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let num = u256::random_prime_using_Miller_Rabin(5);
    println!("Random Prime Number = {}", num);
    assert!(num.is_prime_using_Miller_Rabin(5));
    println!("---------------------------");
}

fn BigUInt_randomize___main()
{
    println!("BigUInt_randomize___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut r = u256::new();
    println!("original number = {}", r);
    assert_eq!(r, u256::zero());
    r.randomize();
    println!("random number = {}", r);
    assert_ne!(r, u256::zero());
    println!("---------------------------");
}

fn BigUInt_is_prime_using_Miller_Rabin___main()
{
    println!("BigUInt_is_prime_using_Miller_Rabin___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let num = u1024::random();
    let yes = num.is_prime_using_Miller_Rabin(5);
    println!("Is {} a prime number? => {}", num, yes);
    if yes  { assert!(yes); }
    else    { assert!(!yes); }
    println!("---------------------------");
}

fn BigUInt_get_size___main()
{
    BigUInt_size_in_bytes___main();
    BigUInt_size_in_bits___main();
    BigUInt_length_in_bytes___main();
    BigUInt_length_in_bits___main();
}

fn BigUInt_size_in_bytes___main()
{
    println!("BigUInt_size_in_bytes___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    println!("u256 is {}-byte integer.", u256::size_in_bytes());
    assert_eq!(u256::size_in_bytes(), 32);
    println!("---------------------------");
}

fn BigUInt_size_in_bits___main()
{
    println!("BigUInt_size_in_bits___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    println!("u256 is {}-bit integer.", u256::size_in_bits());
    assert_eq!(u256::size_in_bits(), 256);
    println!("---------------------------");
}

fn BigUInt_length_in_bytes___main()
{
    println!("BigUInt_length_in_bytes___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a = u256::from_str_radix("A16F", 16).unwrap();
    println!("a is {}-byte integer.", a.length_in_bytes());
    assert_eq!(a.length_in_bytes(), 32);
    println!("---------------------------");
}

fn BigUInt_length_in_bits___main()
{
    println!("BigUInt_length_in_bits___main");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a = u256::from_str_radix("A16F", 16).unwrap();
    println!("a is {}-bit integer.", a.length_in_bits());
    assert_eq!(a.length_in_bits(), 256);
    println!("---------------------------");
}

fn BigUInt_get_set_check___main()
{
    BigUInt_turn_check_bits___main();
    BigUInt_get_num___main();
    BigUInt_get_num____main();
    BigUInt_set_num___main();
    BigUInt_set_num____main();
    BigUInt_get_number___main();
    BigUInt_set_number___main();
    BigUInt_copy_within___main();
    BigUInt_set_zero___main();
    BigUInt_is_zero___main();
    BigUInt_set_one___main();
    BigUInt_is_one___main();
    BigUInt_is_zero_or_one___main();
    BigUInt_set_max___main();
    BigUInt_set_submax___main();
    BigUInt_set_halfmax___main();
    BigUInt_is_max___main();
    BigUInt_set_msb___main();
    BigUInt_set_lsb___main();
    BigUInt_set_uint___main();
    BigUInt_is_uint___main();
    BigUInt_is_odd___main();
    BigUInt_is_even___main();

}

fn BigUInt_turn_check_bits___main()
{
    println!("BigUInt_length_in_bits___main");
    use Cryptocol::define_utypes_with;
    
    define_utypes_with!(u128);
    let mut a = u256::random();
    println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    a.turn_check_bits(102);
    println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a, u256::from_str_radix("1000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000", 2).unwrap());
    println!("---------------------------");
}
/////THIS/////

fn BigUInt_get_num___main()
{
    println!("BigUInt_get_num___main");

    println!("---------------------------");
}

fn BigUInt_get_num____main()
{
    println!("BigUInt_get_num____main");

    println!("---------------------------");
}

fn BigUInt_set_num___main()
{
    println!("BigUInt_set_num___main");

    println!("---------------------------");
}

fn BigUInt_set_num____main()
{
    println!("BigUInt_set_num____main");

    println!("---------------------------");
}

fn BigUInt_get_number___main()
{
    println!("BigUInt_get_number___main");

    println!("---------------------------");
}

fn BigUInt_set_number___main()
{
    println!("BigUInt_set_number___main");

    println!("---------------------------");
}

fn BigUInt_copy_within___main()
{
    println!("BigUInt_copy_within___main");

    println!("---------------------------");
}

fn BigUInt_set_zero___main()
{
    println!("BigUInt_set_zero___main");

    println!("---------------------------");
}

fn BigUInt_is_zero___main()
{
    println!("BigUInt_is_zero___main");

    println!("---------------------------");
}

fn BigUInt_set_one___main()
{
    println!("BigUInt_set_one___main");

    println!("---------------------------");
}

fn BigUInt_is_one___main()
{
    println!("BigUInt_is_one___main");

    println!("---------------------------");
}

fn BigUInt_is_zero_or_one___main()
{
    println!("BigUInt_is_zero_or_one___main");

    println!("---------------------------");
}

fn BigUInt_set_max___main()
{
    println!("BigUInt_set_max___main");

    println!("---------------------------");
}

fn BigUInt_set_submax___main()
{
    println!("BigUInt_set_submax___main");

    println!("---------------------------");
}

fn BigUInt_set_halfmax___main()
{
    println!("BigUInt_set_halfmax___main");

    println!("---------------------------");
}

fn BigUInt_is_max___main()
{
    println!("BigUInt_is_max___main");

    println!("---------------------------");
}

fn BigUInt_set_msb___main()
{
    println!("BigUInt_set_msb___main");

    println!("---------------------------");
}

fn BigUInt_set_lsb___main()
{
    println!("BigUInt_set_lsb___main");

    println!("---------------------------");
}

fn BigUInt_set_uint___main()
{
    println!("BigUInt_set_uint___main");

    println!("---------------------------");
}

fn BigUInt_is_uint___main()
{
    println!("BigUInt_is_uint___main");

    println!("---------------------------");
}

fn BigUInt_is_odd___main()
{
    println!("BigUInt_is_odd___main");

    println!("---------------------------");
}

fn BigUInt_is_even___main()
{
    println!("BigUInt_is_even___main");

    println!("---------------------------");
}






fn BigUInt_carrying_add___main()
{
    println!("BigUInt_carrying_add___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;

    define_utypes_with!(u128);

    let a_hi = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    let a_lo = u256::from_str("91234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    let b_hi = u256::from_str("1111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    let b_lo = u256::from_str("101111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();

    let (c_lo, carry) = a_lo.carrying_add(&b_lo, false);
    let (c_hi, overflow) = a_hi.carrying_add(&b_hi, carry);

    println!("{}:{} + {}:{} = {}:{}", a_hi, a_lo, b_hi, b_lo, c_hi, c_lo);
    println!("carry = {}, overflow = {}", carry, overflow);

    assert_eq!(c_hi.to_string(), "10987654312098765431209876543120987654312098765431209876543120987654312098765");
    assert_eq!(c_lo.to_string(), "76553589753918372475552471781224437825721249902258559417332328337765861594630");
    assert_eq!(carry, true);
    assert_eq!(overflow, false);
    println!("---------------------------");
}

fn BigUInt_carrying_add_assign___main()
{
    println!("BigUInt_carrying_add_assign___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;

    define_utypes_with!(u128);

    let mut a_hi = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    let mut a_lo = u256::from_str("91234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    let b_hi = u256::from_str("1111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    let b_lo = u256::from_str("101111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();

    let carry = a_lo.carrying_add_assign(&b_lo, false);
    let overflow = a_hi.carrying_add_assign(&b_hi, carry);

    println!("9876543210987654321098765432109876543210987654321098765432109876543210987654:91234567890123456789012345678901234567890123456789012345678901234567890123456 + {}:{} = {}:{}", b_hi, b_lo, a_hi, a_lo);
    println!("carry = {}, overflow = {}", carry, overflow);

    assert_eq!(a_hi.to_string(), "10987654312098765431209876543120987654312098765431209876543120987654312098765");
    assert_eq!(a_lo.to_string(), "76553589753918372475552471781224437825721249902258559417332328337765861594630");
    assert_eq!(carry, true);
    assert_eq!(overflow, false);
    println!("---------------------------");
}

fn BigUInt_wrapping_add___main()
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

fn BigUInt_wrapping_add_assign___main()
{
    println!("BigUInt_wrapping_add_assign___main()");
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