fn main()
{
    // test_wrapping_mul();test_wrapping_mul();
    test_widening_mul_assign_uint();
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
    fn widening_mul_assign_uint1<U>(&mut self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    fn widening_mul_assign_uint2<U>(&mut self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    fn widening_mul_assign1<U>(&mut self, rhs: U) -> Self;
    fn widening_mul_assign2<U>(&mut self, rhs: U) -> Self;
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

    fn widening_mul_assign_uint1<U>(&mut self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { return self.widening_mul_assign(&Self::from_uint(rhs)); }
        let mut trhs = T::num::<U>(rhs);
        let zero = T::zero();
        let mut high = Self::zero();
        if trhs == zero
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let mut lower = zero;
        let mut higher = zero;
        for i in 0..N
        {
            (lower, higher) = self.get_num_(i).carrying_mul(trhs, higher);
            self.set_num_(i, lower);
        }
        high.set_num_(0, higher);
        high
    }

    fn widening_mul_assign_uint2<U>(&mut self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { return self.widening_mul_assign(&Self::from_uint(rhs)); }
        // if U::size_in_bytes() <= T::size_in_bytes()
        let mut trhs = T::num::<U>(rhs);
        let zero = T::zero();
        let mut high = Self::zero();
        if trhs == zero
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let mut adder = self.clone();
        self.set_zero();
        loop
        {
            if trhs.is_odd()
            {
                if self.overflowing_add_assign(&adder)
                    { high.wrapping_add_assign_uint(1_u8); }
            }
            trhs >>= T::one();
            if trhs == T::zero()
                { break; }
                adder.shift_left_assign(1_u8);
        }
        high
    }

    fn widening_mul_assign1(&mut self, rhs: &Self) -> Self
    {
        let mut high = Self::zero();
        if rhs.is_zero()
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let zero = T::zero();
        let mut lower = zero;
        let mut higher = zero;
        let mut operand = self.clone();
        for i in 0..N
        {
            for j in 0..N
            {
                (lower, higher) = rhs.get_num_(i).carrying_mul(operand.get_num_(j), higher);
                if i + j < N
                    { self.set_num_(i + j, lower); }
                else
                    { high.set_num_(i + j - N, lower); }
            }
            high.set_num_(i, higher);
        }
        high
    }

    fn widening_mul_assign2(&mut self, rhs: &Self) -> Self
    {
        let mut high = Self::zero();
        if rhs.is_zero()
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let adder = self.clone();
        let TSIZE_BITS = T::size_in_bits();
        let mut chunk = N - 1 - rhs.leading_zero_elements() as usize;
        let mut piece = T::size_in_bits() - 1 - rhs.get_num_(chunk).leading_zeros() as usize;
        self.set_zero();
        loop
        {
            let num = rhs.get_num_(chunk);
            if num.is_zero()
            {
                high.shift_left_assign(TSIZE_BITS);
                let MSByte = self.get_num_(N-1);
                if !MSByte.is_zero()
                    { high.set_num_(0, MSByte); }
                self.shift_left_assign(TSIZE_BITS);
            }
            else
            {
                loop
                {
                    if num.is_bit_set_(piece)
                    {
                        if self.overflowing_add_assign(&adder)
                            { high.wrapping_add_assign_uint(1_u8); }
                    }
                    if piece == 0
                        { break; }
                    piece -= 1;
                    high.shift_left_assign(1_u8);
                    if self.is_MSB_set()
                        { high.set_LSB(); }
                    self.shift_left_assign(1_u8);
                }
            }
            if chunk == 0
                { break; }
            chunk -= 1;
            piece = T::size_in_bits() - 1;
        }
        high
    }
}


macro_rules! performance_wrapping_mul
{
    ($t:ty, $b:expr, $ti:expr, $f:expr) => {
        let mut sum = <$t>::one();
        let a = <$t>::from(3_u8);
        let now: SystemTime;
        let elapsed;
        match $f
        {
            0 => {
                now = SystemTime::now();
                for _ in 0..100000
                    { sum.wrapping_mul_assign(&a); }
                elapsed = now.elapsed();
                println!("sum = {}", sum);
            },
            1 => {
                now = SystemTime::now();
                for _ in 0..100000
                    { sum.wrapping_mul_assign2(&a); }
                elapsed = now.elapsed();
                println!("sum = {}", sum);
            },
            2 => {
                now = SystemTime::now();
                for _ in 0..100000
                    { sum.wrapping_mul_assign3(&a); }
                elapsed = now.elapsed();
                println!("sum = {}", sum);
            },
            _ => { panic!("Wrong_test_wrapping_mul"); }
        }
        match elapsed
        {
            Ok(e) => {
                $ti = e.as_micros();
                println!("{}-based: {} usec", $b, $ti);
            },
            Err(e) => { println!("{}", e); },
        }
    }
}

pub fn test_wrapping_mul()
{
    use std::time::SystemTime;
    use Cryptocol::number::*;

    let dt = ["u128", "u64", "u32", "u16", "u8"];
    let op = ["multplication1", "multplication2" , "multplication3"];
    let mut ti = [[0_u128; 5]; 3];   // How many microseconds

    performance_wrapping_mul!(u16384_with_u128, dt[0], ti[0][0], 0); // for warmming up
    for operator in 0..op.len()
    {
        println!("=== {} ===", op[operator]);
        performance_wrapping_mul!(u16384_with_u128, dt[0], ti[operator][0], operator);
        performance_wrapping_mul!(u16384_with_u64, dt[1], ti[operator][1], operator);
        performance_wrapping_mul!(u16384_with_u32, dt[2], ti[operator][2], operator);
        performance_wrapping_mul!(u16384_with_u16, dt[3], ti[operator][3], operator);
        performance_wrapping_mul!(u16384_with_u8, dt[4], ti[operator][4], operator);
    
        let mut fastest = 0;
        for i in 1..dt.len()
        {
            if ti[operator][i] < ti[operator][fastest]
                { fastest = i; }
        }
        println!("The fastest one is {}.", dt[fastest]);
    }

    for d in 0..dt.len()
    {
        let mut fastest = 0;
        for i in 1..ti.len()
        {
            if ti[i][d] < ti[fastest][d]
                { fastest = i; }
        }
        println!("{}: The fastest one is {}: {} - {} - {}", dt[d], op[fastest], ti[0][d], ti[1][d], ti[2][d]);
    }
}


macro_rules! performance_widening_mul_assign_uint
{
    ($t:ty, $b:expr, $ti:expr, $f:expr) => {
        let mut sum = <$t>::one();
        let now: SystemTime;
        let elapsed;
        match $f
        {
            0 => {
                now = SystemTime::now();
                for _ in 0..1_000
                    { sum.widening_mul_assign_uint(3_u8); }
                elapsed = now.elapsed();
                println!("sum = {}", sum);
            },
            1 => {
                now = SystemTime::now();
                for _ in 0..1_000
                    { sum.widening_mul_assign_uint1(3_u8); }
                elapsed = now.elapsed();
                println!("sum = {}", sum);
            },
            2 => {
                now = SystemTime::now();
                for _ in 0..1_000
                    { sum.widening_mul_assign_uint2(3_u8); }
                elapsed = now.elapsed();
                println!("sum = {}", sum);
            },
            _ => { panic!("Wrong_test_widening_mul_assign_uint"); }
        }
        match elapsed
        {
            Ok(e) => {
                $ti = e.as_micros();
                println!("{}-based: {} usec", $b, $ti);
            },
            Err(e) => { println!("{}", e); },
        }
    }
}

pub fn test_widening_mul_assign_uint()
{
    use std::time::SystemTime;
    use Cryptocol::number::*;
/*
    let mut sum = 0_u128;
    let mut high = 10_u128;
    let rhs = 3_u128;
    
    for _ in 0..1
    {
        print!("high = {}, ", high);
        (sum, high) = sum.carrying_mul(rhs, high);
        println!("sum = {}, high = {}", sum, high);
    }
    
return;
*/
    let dt = ["u128", "u64", "u32", "u16", "u8"];
    let op = ["multplication0", "multplication1", "multplication2"];
    let mut ti = [[0_u128; 5]; 3];   // How many microseconds

    performance_widening_mul_assign_uint!(u16384_with_u128  , dt[0], ti[0][0], 0); // for warmming up
    for operator in 0..op.len()
    {
        println!("=== {} ===", op[operator]);
        performance_widening_mul_assign_uint!(u16384_with_u128, dt[0], ti[operator][0], operator);
        performance_widening_mul_assign_uint!(u16384_with_u64, dt[1], ti[operator][1], operator);
        performance_widening_mul_assign_uint!(u16384_with_u32, dt[2], ti[operator][2], operator);
        performance_widening_mul_assign_uint!(u16384_with_u16, dt[3], ti[operator][3], operator);
        performance_widening_mul_assign_uint!(u16384_with_u8, dt[4], ti[operator][4], operator);
    
        let mut fastest = 0;
        for i in 1..dt.len()
        {
            if ti[operator][i] < ti[operator][fastest]
                { fastest = i; }
        }
        println!("The fastest one is {}.", dt[fastest]);
    }

    for d in 0..dt.len()
    {
        let mut fastest = 0;
        for i in 1..ti.len()
        {
            if ti[i][d] < ti[fastest][d]
                { fastest = i; }
        }
        println!("{}: The fastest one is {}: {} - {} - {} : {} - {}", dt[d], op[fastest], ti[0][d], ti[1][d], ti[2][d], ti[1][d] as f64 / ti[0][d] as f64, ti[2][d] as f64 / ti[0][d] as f64);
    }
}
