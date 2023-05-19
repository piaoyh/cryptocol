//
// huge_numbers: the traits for BigUInt, BigInt, LargeUInt and LargeInt
// Version:				0.1.0.0
// Author:				PARK Youngho
//

/*
pub trait Large_Integer<T, const N: usize>
where T: Uint,
    Self: Sized + Clone
        + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
{
    const OVERFLOW: u8  = 0b0000_0001;
    const UNDERFLOW: u8 = 0b0000_0010;
    const INFINITY: u8  = 0b0000_0100;
    const DIVIDED_BY_ZERO: u8 = Self::INFINITY;

    fn new() -> Self;
    fn from_array(val: &[T; N]) -> Self;
    fn from_uint<V: Copy>(val: V) -> Self;
    fn from_string_with_radix(txt: &str, radix: usize) -> Option<Self>;
    fn to_string_with_radix(&self, radix: usize) -> String;

    fn times(&mut self, rhs: T);
    fn quotient(&mut self, rhs: T);
    fn remainder(&mut self, rhs: T);
    fn add_uint(&self, rhs: T) -> Self;
    fn sub_uint(&self, rhs: T) -> Self;
    fn mul_uint(&self, rhs: T) -> Self;
    fn div_uint(&self, rhs: T) -> Self;
    fn rem_uint(&self, rhs: T) -> Self;
//    fn div_assign_fully(&self, quotient: &mut Self, remainder: &mut Self, dividend: &Self, divisor: &Self);
    fn get_num(&self, i: usize) -> T;
    fn set_num(&mut self, i: usize, val: T);
    fn set_number(&mut self, val: &[T; N]);
    fn set_zero(&mut self);
    fn is_zero(&self) -> bool;
    fn set_max(&mut self);
    fn is_max(&self) -> bool;
    fn set_uint(&mut self, val: T);
    fn is_uint(&self, val: T) -> bool;
    fn set_flag_bit(&mut self, flag_bits: u8);
    fn reset_flag_bit(&mut self, flag_bits: u8);
    fn is_flag_bit_on(&self, flag_bits: u8) -> bool;

    fn accumulate(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self += bi;
    }

    fn dissipate(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self -= bi;
    }

    fn to_string(&self) -> String   { self.to_string_with_radix(10) }
    fn set_overflow(&mut self)      { self.set_flag_bit(Self::OVERFLOW); }
    fn reset_overflow(&mut self)    { self.reset_flag_bit(Self::OVERFLOW); }
    fn is_overflow(&self) -> bool   { self.is_flag_bit_on(Self::OVERFLOW) }
    fn set_underflow(&mut self)     { self.set_flag_bit(Self::UNDERFLOW) }
    fn reset_underflow(&mut self)   { self.reset_flag_bit(Self::UNDERFLOW) }
    fn is_underflow(&self) -> bool  { self.is_flag_bit_on(Self::UNDERFLOW) }
    fn set_inifinity(&mut self)     { self.set_flag_bit(Self::INFINITY); }
    fn reset_inifinity(&mut self)   { self.reset_flag_bit(Self::INFINITY); }
    fn is_inifinity(&self) -> bool  { self.is_flag_bit_on(Self::INFINITY) }
    fn set_untrustable(&mut self)   { self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn reset_untrustable(&mut self) { self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn is_untrustable(&self) -> bool { self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW)}
    fn set_divided_by_zero(&mut self)   { self.set_inifinity(); }
    fn reset_divided_by_zero(&mut self) { self.reset_inifinity(); }
    fn is_divided_by_zero(&self) -> bool { self.is_inifinity() }
}
*/