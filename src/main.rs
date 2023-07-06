use std::time::SystemTime;
use std::fmt::{ Display, Debug };
use std::ops::*;
//use std::mem::size_of;

mod number;

//#[macro_export]

//use number::*;
//use Cryptocol::number::*; //{u256, BigInteger, HugeInteger};

//use Cryptocol::number::BigUInt;


// fn func<T: Uint>(lhs: T, rhs: T) -> T
// {
//     lhs.wrapping_add(rhs)
// }

fn main()
{
    use Cryptocol::number::{ HugeInteger, BigUInt};
    let a = BigUInt::<u8,32>::from_string_with_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16).unwrap();
    println!("a = {}", a);
    assert_eq!(a.to_string(), "8234104123542484900769178205574010627627573691361805720124810878238590820080");
    define_utypes_with!(u128);
    // u256::new();
    // let a = 100_u8;
    // let b = 100_u8;
    // let c = func(a, b);
    // let d = func(c, 57);
    // println!("a + b = {}", c);
    // println!("c + 57 = {}", d);
    // assert_eq!(c, 200_u8);
    // assert_eq!(d, 1_u8);
    
    /*
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
    // assert_eq!(a.is_underflow(), true);*/
}