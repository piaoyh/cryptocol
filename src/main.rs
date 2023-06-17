use std::time::{ SystemTime, Duration };
use std::fmt::{ Display, Debug };
use std::ops::*;

mod number;
use number::*;


fn main()
{
    let num_txt = "1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_";
    let a_128 = u1024_with_u128::from_string(num_txt).unwrap();
    let a_64 = u1024_with_u64::from_string(num_txt).unwrap();
    let a_32 = u1024_with_u32::from_string(num_txt).unwrap();
    let a_16 = u1024_with_u16::from_string(num_txt).unwrap();
    let a_8 = u1024_with_u8::from_string(num_txt).unwrap();
    let mut sum_128 = u1024_with_u128::zero();
    let mut sum_64 = u1024_with_u64::zero();
    let mut sum_32 = u1024_with_u32::zero();
    let mut sum_16 = u1024_with_u16::zero();
    let mut sum_8 = u1024_with_u8::zero();

    calcAdd(&mut sum_128, &a_128, 128);
    calcAdd(&mut sum_64, &a_64, 64);
    calcAdd(&mut sum_32, &a_32, 32);
    calcAdd(&mut sum_16, &a_16, 16);
    calcAdd(&mut sum_8, &a_8, 8);

    calcMul(&mut sum_128, &a_128, 128);
    calcMul(&mut sum_64, &a_64, 64);
    calcMul(&mut sum_32, &a_32, 32);
    calcMul(&mut sum_16, &a_16, 16);
    calcMul(&mut sum_8, &a_8, 8);
}

fn calcAdd<T, const N: usize, S>(sum: &mut S, a: &S, bits: u8)
where T: Uint + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    S: Sized + Clone + Copy + Display + ToString
        + Add + AddAssign + Sub + SubAssign
        + Mul + MulAssign + Div + DivAssign
        + Rem + RemAssign
        + Shl<i32> + ShlAssign<i32>
        + Shr<i32> + ShrAssign<i32>
        + BitAnd + BitAndAssign + BitOr + BitOrAssign
        + BitXorAssign + Not
        + BigNumber<T, N> + BigUnsignedInt<T, N>
{
    let mut now = SystemTime::now();
    for _ in 0..1000
    {
        *sum += *a;
    }
    let mut elapsed = now.elapsed().unwrap();
    println!("{}-bit addition operation takes\t{}", bits, elapsed.as_nanos());
}

fn calcMul<T, const N: usize, S>(sum: &mut S, a: &S, bits: u8)
where T: Uint + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    S: Sized + Clone + Copy + Display + ToString
        + Add + AddAssign + Sub + SubAssign
        + Mul + MulAssign + Div + DivAssign
        + Rem + RemAssign
        + Shl<i32> + ShlAssign<i32>
        + Shr<i32> + ShrAssign<i32>
        + BitAnd + BitAndAssign + BitOr + BitOrAssign
        + BitXorAssign + Not
        + BigNumber<T, N> + BigUnsignedInt<T, N>
{
    sum.set_one();
    let mut now = SystemTime::now();
    for _ in 0..1000
    {
        *sum *= *a;
    }
    let mut elapsed = now.elapsed().unwrap();
    println!("{}-bit multiplication operation takes\t{}", bits, elapsed.as_nanos());
}