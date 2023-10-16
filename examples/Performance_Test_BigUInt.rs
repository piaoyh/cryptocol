fn main()
{
    t();
    // test_main_BigUInt();
}


use std::fmt::{Debug, Display};
use std::ops::*;
use std::convert::*;
use std::mem::size_of;
use std::str::FromStr;
use Cryptocol::number::*;
use Cryptocol::number::small_uint::*;
use Cryptocol::number::BigUInt;

trait PerformanceTestForBigUint
    : Sized + Clone + Display + Debug + ToString
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
    + Rem<Output = Self> + RemAssign
    + Shl<i32, Output = Self> + ShlAssign<i32>
    + Shr<i32, Output = Self> + ShrAssign<i32>
    + BitAnd<Self, Output = Self> + BitAndAssign
    + BitOr<Self, Output = Self> + BitOrAssign
    + BitXor<Self, Output = Self> + BitXorAssign
    + Not<Output = Self>
{
    fn wrapping_mul_assign2(&mut self, rhs: &Self);
    fn wrapping_mul_assign3(&mut self, rhs: &Self);
}

impl<T, const N: usize> PerformanceTestForBigUint for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn wrapping_mul_assign2(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE_BITS = T::size_in_bits();;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::usize_as_SmallUInt(TSIZE_BITS - 1);
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { self.wrapping_add_assign(&adder); }
                bit_check >>= one;
            }
        };

        let mut n = N - 1;
        while rhs.get_num_(n) == zero
            { n -= 1; }
        multiply_first(rhs.get_num_(n));
        if n == 0
            { return; }
        n -= 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BITS as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::usize_as_SmallUInt(TSIZE_BITS - 1);
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { self.wrapping_add_assign(&adder); }
                bit_check >>= one;
            }
        };

        loop
        {
            multiply(rhs.get_num_(n));
            if n == 0
                { break; }
            n = n.wrapping_sub(1);
        }
    }

    fn wrapping_mul_assign3(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let mut adder = self.clone();
        let mut mrhs = rhs.clone();
        self.set_zero();
        loop
        {
            if mrhs.is_odd()
                { self.wrapping_add_assign(&adder); }
            mrhs.shift_right_assign(1_u8);
            if mrhs.is_zero()
                { break; }
            adder.shift_left_assign(1_u8);
        }
    }
}


macro_rules! performance
{
    ($t:ty, $b:expr, $ti:expr, $f:expr) => {
        let mut sum = <$t>::one();
        let a = <$t>::from(3_u8);
        let now: SystemTime;
        match $f
        {
            0 => {
                now = SystemTime::now();
                for _ in 0..100000
                    { sum.wrapping_mul_assign(&a); }
                println!("sum = {}", sum);
            },
            1 => {
                now = SystemTime::now();
                for _ in 0..100000
                    { sum.wrapping_mul_assign2(&a); }
                println!("sum = {}", sum);
            },
            _ => {
                now = SystemTime::now();
                for _ in 0..100000
                    { sum.wrapping_mul_assign3(&a); }
                println!("sum = {}", sum);
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

pub fn t()
{
    use std::time::SystemTime;
    use Cryptocol::number::*;

    let mut ti = [[0_u128; 5]; 3];   // How many microseconds
    let dt = ["u128", "u64", "u32", "u16", "u8"];
    let op = ["multplication1", "multplication2" , "multplication3"];

    for operator in 0..3
    {
        println!("=== {} ===", op[operator]);
        performance!(u2048_with_u128, dt[0], ti[operator][0], operator);
        performance!(u2048_with_u64, dt[1], ti[operator][1], operator);
        performance!(u2048_with_u32, dt[2], ti[operator][2], operator);
        performance!(u2048_with_u16, dt[3], ti[operator][3], operator);
        performance!(u2048_with_u8, dt[4], ti[operator][4], operator);
    
        let mut fastest = 0;
        for i in 1..5
        {
            if ti[operator][i] < ti[operator][fastest]
                { fastest = i; }
        }
        println!("The fastest one is {}.", dt[fastest]);
    }

    for d in 0..5
    {
        let mut fastest = 0;
        for i in 1..3
        {
            if ti[i][d] < ti[fastest][d]
                { fastest = i; }
        }
        println!("{}: The fastest one is {}: {} - {} - {}", dt[d], op[fastest], ti[0][d], ti[1][d], ti[2][d]);
    }
}