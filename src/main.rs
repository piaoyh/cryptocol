use std::time::SystemTime;
use std::fmt::{ Display, Debug };
use std::ops::*;
//use std::mem::size_of;

mod number;

//#[macro_export]

//use number::*;
use Cryptocol::number::*; //{u256, BigInteger, HugeInteger};

//use Cryptocol::number::BigUInt;


// fn func<T: Uint>(lhs: T, rhs: T) -> T
// {
//     lhs.wrapping_add(rhs)
// }

fn main()
{
    // define_utypes_with!(u128);
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
    // assert_eq!(a.is_underflow(), true);
*/
    let num_txt = "1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_";
    let a_128 = u1024_with_u128::from_string(num_txt).unwrap();
    let a_64 = u1024_with_u64::from_string(num_txt).unwrap();
    let a_32 = u1024_with_u32::from_string(num_txt).unwrap();
    let a_16 = u1024_with_u16::from_string(num_txt).unwrap();
    let a_8 = u1024_with_u8::from_string(num_txt).unwrap();

    calcAdd(&a_128);
    calcAdd(&a_64);
    calcAdd(&a_32);
    calcAdd(&a_16);
    calcAdd(&a_8);

    calcMul(&a_128);
    calcMul(&a_64);
    calcMul(&a_32);
    calcMul(&a_16);
    calcMul(&a_8);


    // use Cryptocol::number::BigInteger;
   // use Cryptocol::define_utypes_with;
    //define_utypes_with!(u128);
    // let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    // let b = a << 1;
    // println!("b = {}", b);

    // let bi = u256::from_string_with_radix("A16F", 16).unwrap();
    // assert_eq!(bi.length_in_bytes(), 256 / 8);
}

fn calcAdd<T, const N: usize>(a: &BigUInt<T, N>)
where T: Uint + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let mut sum = BigUInt::<T, N>::zero();
    let now = SystemTime::now();
    for _ in 0..1000
    {
        sum += *a;
    }
    let elapsed = now.elapsed().unwrap();
    println!("{}-bit addition operation takes\t{}", T::size_in_bits(), elapsed.as_nanos());
}

fn calcMul<T, const N: usize>(a: &BigUInt<T, N>)
where T: Uint + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let mut sum = BigUInt::<T, N>::one();
    let now = SystemTime::now();
    for _ in 0..1000
    {
        sum *= *a;
    }
    let elapsed = now.elapsed().unwrap();
    println!("{}-bit multiplication operation takes\t{}", T::size_in_bits(), elapsed.as_nanos());
}
