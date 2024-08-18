# Breaking Changes

## Breaking changes from ver. 0.8.1 to ver. 0.8.2

### Twenty-four methods of BigUInt

| Ver. 0.8.1                                                      | Ver. 0.8.2 |
|-----------------------------------------------------------------|------------|
| pub fn wrapping_root_uint(&self, exp: U) -> Self                | removed    |
| pub fn wrapping_root_assign_uint(&mut self, exp: U)             | removed    |
| pub fn overflowing_root_uint(&self, exp: U) -> (Self, bool)     | removed    |
| pub fn overflowing_root_assign_uint(&mut self, exp: U) -> bool  | removed    |
| pub fn saturating_root_uint(&self, exp: U) -> Self              | removed    |
| pub fn saturating_root_assign_uint(&mut self, exp: U)           | removed    |
| pub fn wrapping_ilog_uint(&self, base: U) -> Self               | removed    |
| pub fn wrapping_ilog_assign_uint(&mut self, base: U)            | removed    |
| pub fn overflowing_ilog_uint(&self, base: U) -> (Self, bool)    | removed    |
| pub fn overflowing_ilog_assign_uint(&mut self, base: U) -> bool | removed    |
| pub fn saturating_ilog_uint(&self, base: U) -> Self             | removed    |
| pub fn saturating_ilog_assign_uint(&mut self, base: U)          | removed    |
| pub fn wrapping_ilog2_uint(&self) -> Self                       | removed    |
| pub fn wrapping_ilog2_assign_uint(&mut self)                    | removed    |
| pub fn overflowing_ilog2_uint(&self) -> (Self, bool)            | removed    |
| pub fn overflowing_ilog2_assign_uint(&mut self) -> bool         | removed    |
| pub fn saturating_ilog2_uint(&self) -> Self                     | removed    |
| pub fn saturating_ilog2_assign_uint(&mut self)                  | removed    |
| pub fn wrapping_ilog10_uint(&self) -> Self                      | removed    |
| pub fn wrapping_ilog10_assign_uint(&mut self)                   | removed    |
| pub fn overflowing_ilog10_uint(&self) -> (Self, bool)           | removed    |
| pub fn overflowing_ilog10_assign_uint(&mut self) -> bool        | removed    |
| pub fn saturating_ilog10_uint(&self) -> Self                    | removed    |
| pub fn saturating_ilog10_assign_uint(&mut self)                 | removed    |

- The methods above have been removed because they never overflow.

### Seven methods of BigUInt

| Ver. 0.8.1                                                      | Ver. 0.8.2                                                       |
|-----------------------------------------------------------------|------------------------------------------------------------------|
| pub fn root_uint(&self, exp: U) -> Self                         | pub fn iroot_uint(&self, exp: U) -> Self                         |
| pub fn root_assign_uint(&mut self, exp: U)                      | pub fn iroot_assign_uint(&mut self, exp: U)                      |
| pub fn checked_root_uint(&self, exp: U) -> Option&lt;Self&gt;   | pub fn checked_iroot_uint(&self, exp: U) -> Option&lt;Self&gt;   |
| pub fn unchecked_root_uint(&self, exp: U) -> Option&lt;Self&gt; | pub fn unchecked_iroot_uint(&self, exp: U) -> Option&lt;Self&gt; |
| pub fn set_inifinity(&mut self)                                 | pub fn set_infinity(&mut self)                                   |
| pub fn reset_inifinity(&mut self)                               | pub fn reset_infinity(&mut self)                                 |
| pub fn is_inifinity(&self) -> bool                              | pub fn is_infinity(&self) -> bool                                |

- The names of the four upper methods above `*root*_uint()` have been changed into `*iroot*_uint()` in order to keep consistency with primitive data types such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.
- The names of the three lower methods above `*inifinity()` have been changed into `*infinity()` because `inifinity` is the typo mistake of `infinity`.

| Methods                                                                       | Ver. 0.8.1                                                    | Ver. 0.8.2                                       |
|-------------------------------------------------------------------------------|---------------------------------------------------------------|--------------------------------------------------|
| pub fn modular_add_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_add_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_sub_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_sub_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_mul_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_mul_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_div_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_div_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn modular_div_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_div_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn modular_rem_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_rem_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self        | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn modular_rem_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_rem_assign_uint&lt;U&gt;(&self, rhs: U, modulo: &Self) -> Self | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_div_uint&lt;U&gt;(&self, rhs: U) -> Self                      | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_div_assign_uint&lt;U&gt;(&self, rhs: U) -> Self               | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_div_uint&lt;U&gt;(&self, rhs: U) -> Self                   | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_div_assign_uint&lt;U&gt;(&self, rhs: U) -> Self            | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn pub fn unchecked_div_uint&lt;U&gt;(&self, rhs: U) -> Self              | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_div_uint&lt;U&gt;(&self, rhs: U) -> Self                    | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_div_assign_uint&lt;U&gt;(&mut self, rhs: U) -> Self         | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_rem_uint&lt;U&gt;(&self, rhs: U) -> Self                      | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_rem_assign_uint&lt;U&gt;(&self, rhs: U) -> Self               | It gives zero to `self` if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_rem_uint&lt;U&gt;(&self, rhs: U) -> Self                   | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_rem_assign_uint&lt;U&gt;(&self, rhs: U) -> Self            | It gives zero to `self` if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn pub fn unchecked_rem_uint&lt;U&gt;(&self, rhs: U) -> Self              | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_rem_uint&lt;U&gt;(&self, rhs: U) -> Self                    | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_rem_assign_uint&lt;U&gt;(&mut self, rhs: U) -> Self         | It gives zero to `self` if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn divide_fully_uint&lt;U&gt;(&self, rhs: U) -> (Self, U)                 | It returns (maximum, 0) if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn modular_pow_uint&lt;U&gt;(&self, exp: U, modulo: &Self) -> Self        | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_pow_assign_uint&lt;U&gt;(&mut self, exp: U, modulo: &Self)     | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_add_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_sub_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_mul_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn modular_div_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_div_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `modulo` is either zero or one.         | It will panic if `modulo` is either zero or one. |
| pub fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self                  | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn modular_rem_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `modulo` is either zero or one. | It will panic if `modulo` is either zero or one. |
| pub fn modular_rem_assign(&self, rhs: &Self, modulo: &Self) -> Self           | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_div(&self, rhs: &Self) -> Self                                | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_div_assign(&self, rhs: &Self) -> Self                         | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_div(&self, rhs: &Self) -> Self                             | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_div_assign(&self, rhs: &Self) -> Self                      | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn pub fn unchecked_div(&self, rhs: &Self) -> Self                        | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_div(&self, rhs: &Self) -> Self                              | It returns maximum if `rhs` is zero.                          | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_div_assign(&mut self, rhs: &Self) -> Self                   | It gives maximum to `self` if `rhs` is zero.                  | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_rem(&self, rhs: &Self) -> Self                                | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn wrapping_rem_assign(&self, rhs: &Self) -> Self                         | It gives zero to `self` if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_rem(&self, rhs: &Self) -> Self                             | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn overflowing_rem_assign(&self, rhs: &Self) -> Self                      | It gives zero to `self` if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn pub fn unchecked_rem(&self, rhs: &Self) -> Self                        | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_rem(&self, rhs: &Self) -> Self                              | It returns zero if `rhs` is zero.                             | It will panic if `rhs` is either zero or one.    |
| pub fn saturating_rem_assign(&mut self, rhs: &Self) -> Self                   | It gives zero to `self` if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
| pub fn divide_fully(&self, rhs: &Self) -> (Self, U)                           | It returns (maximum, 0) if `rhs` is zero.                     | It will panic if `rhs` is either zero or one.    |
   
- The above-methods have been changed to cause panic when the argument `modulo` is either zero or one. The author think that it is high chance that it is a mistake to give zero or one to the arguement `modulo` but this mistake won't be found or will be found only with a lot of efforts at test time if these methods do not cause panic. So, this change has been made for better security.
- The above-methods have been changed to cause panic when the argument `rhs` is zero. The author think that it is high chance that it is a mistake to give zero to the arguement `rhs` but this mistake won't be found or will be found only with a lot of efforts at test time if these methods do not cause panic. So, this change has been made for better security.

## Breaking changes from ver. 0.7.6 to ver. 0.8.0

### One method of trait SmallUInt

| Ver. 0.7.6                             | Ver. 0.8.0                            |
|----------------------------------------|---------------------------------------|
| pub fn root(self, exp: Self) -> Self   | pub fn iroot(self, exp: Self) -> Self |

- The method name root has been changed into iroot for the name consistancy with the methods isqrt, ilog, ilog10, ilog2, etc.

### Two methods of unions ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

| Ver. 0.7.6                             | Ver. 0.8.0 |
|----------------------------------------|------------|
| pub fn root(self, exp: Self) -> Self   | removed    |
| pub fn reverse_bits(self) -> Self      | removed    |

- The above methods has been removed since they were found redundant. If you import (use) SmallUInt, you can use the above methods.

## Breaking changes from ver. 0.7.2 to ver. 0.7.3

### Five methods of BigUInt

| Ver. 0.7.2                                      | Ver. 0.7.3                                     |
|-------------------------------------------------|------------------------------------------------|
| pub fn from_array(val: &[T; N]) -> Self         | pub fn from_array(val: [T; N]) -> Self         |
| pub fn from_be(be: &Self) -> Self               | pub fn from_be(be: Self) -> Self               |
| pub fn from_le(le: S&elf) -> Self               | pub fn from_le(le: Self) -> Self               |
| pub fn from_be_bytes(be_bytes: &[T; N]) -> Self | pub fn from_be_bytes(be_bytes: [T; N]) -> Self |
| pub fn from_le_bytes(le_bytes: &[T; N]) -> Self | pub fn from_le_bytes(le_bytes: [T; N]) -> Self |

- The arguments of all the constructors to be borrowed are changed to be moved except BigUInt::from_string(), BigUInt::from_str_radix(), BigUInt::from_biguint(), and SharedArrays::from_src().
- The reason of the exception to keep the arguments of BigUInt::from_string() and BigUInt::from_str_radix() to be borrowed is to keep the consistency with String::from().
- The reason of the exception to keep the arguments of BigUInt::from_biguint() to be borrowed is for the performance and the convenience.
- The reason of the exception to keep the arguments of SharedArrays::from_src() to be borrowed is for its purpose and the convenience.

### 2 methods of unions ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

| Ver. 0.7.2                           | Ver. 0.7.3                            |
|--------------------------------------|---------------------------------------|
| pub fn onoff(b: bool) -> Self        | pub fn new_with_bool(b: bool) -> Self |
| pub fn onoff_signed(b: bool) -> Self | removed                               |

- The method name `onoff` is not intuitive so that it was changed into more intuitive name `new_with_bool`.
- The method `onoff_signed()` is redundant because it is the same as the method `onoff()` or `new_with_bool()`.

## Breaking changes from ver. 0.7.1 to ver. 0.7.2

### A method of union SharedValue

| Ver. 0.7.1                                          | Ver. 0.7.2 |
|-----------------------------------------------------|------------|
| pub fn into_des(&mut self, pos: usize) -> Option&lt;D&gt; | removed    |

### A method of union SharedArray

| Ver. 0.7.1                                   | Ver. 0.7.2                                           |
|----------------------------------------------|------------------------------------------------------|
| pub fn into_des(&mut self, des: &mut [D; N]) | pub fn put_des_in_array(&self, des: &mut [D; N]) |

- The function name `into_des()` does not show its functionality very clearly. It is desirable that the function name `into_des()` is changed to put_des_in_array() for the name consistency with the methods such as put_hash_value_in_array() in hash modules. The argument `&mut self` does not have to be `&mut self` so that it was changed to `&self`

## Breaking changes from ver. 0.7.0 to ver. 0.7.1

### A macro for BigUInt

| Ver. 0.7.0                | Ver. 0.7.1 |
|---------------------------|------------|
| define_Utypes_with_utypes | removed    |

- The macro name `define_Utypes_with_utypes` should not have been exposed to the outside of this crate so it was removed rather than recommending not to use it at documentation.

### fields of ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

| Ver. 0.7.0  | Ver. 0.7.1 |
|-------------|------------|
| pub this    | this       |
| pub that    | that       |
| pub ubyte   | ubyte      |
| pub sbyte   | sbyte      |
| pub ushort  | ushort     |
| pub sshort  | sshort     |
| pub uint    | uint       |
| pub sint    | sint       |
| pub ulong   | ulong      |
| pub slong   | slong      |
| pub ulonger | ulonger    |
| pub slonger | slonger    |
| pub u_size  | u_size     |
| pub s_size  | s_size     |
| pub u_size  | u_size     |
| pub s_size  | s_size     |

- All the fields of all the unions such as ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion are changed from public into private in order that users cannot access them directly instead of warning users not to access them directly in documentation.

### Names of struct `Share` and `Common`

| Ver. 0.7.0 | Ver. 0.7.1   |
|------------|--------------|
| Share      | SharedValues |
| Common     | SharedArrays |

- The new names `SharedValues` and `SharedArrays` are more intuitive and more explanary than old names `Share` and `Common`. Actually, even the author is often confused with `Share` and `Common` in terms of their goals and roles.

## Breaking changes from ver. 0.6.3 to ver. 0.7.0

### Five methods of trait SmallUInt

| Ver. 0.6.3                        | Ver. 0.7.0               |
|-----------------------------------|--------------------------|
| fn sqrt(self) -> Self             | fn isqrt(self) -> Self   |
| fn is_max(&self) -> bool          | fn is_max(self) -> bool  |
| fn is_zero(&self) -> bool         | fn is_zero(self) -> bool |
| fn is_one(&self) -> bool          | fn is_one(self) -> bool  |
| fn reverse_bits_assign(&mut self) | removed                  |

- A breaking change has been made to change the function name from `number::SmallUInt::sqrt(self) -> Self;` to `number::SmallUInt::isqrt(self) -> Self;` in order to keep consistency with primitive data types such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.
- Breaking changes has been made to change the function arguement from `&self` into `self` in order to keep consistency with other functions.
- reverse_bits_assign(&mut self) has been removed to keep consistency with primitive data types such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

### Methods of unions ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

| Ver. 0.6.3                                       | Ver. 0.7.0 |
|--------------------------------------------------|------------|
| pub fn num(n: u128) -> Self                      | removed    |
| pub fn zero() -> Self                            | removed    |
| pub fn one() -> Self                             | removed    |
| pub fn max() -> Self                             | removed    |
| pub fn min() -> Self                             | removed    |
| pub fn reverse_bits_assign(&mut self)            | removed    |
| pub fn wrapping_add_assign(&mut self, rhs: Self) | removed    |
| pub fn wrapping_sub_assign(&mut self, rhs: Self) | removed    |
| pub fn wrapping_mul_assign(&mut self, rhs: Self) | removed    |
| pub fn wrapping_div_assign(&mut self, rhs: Self) | removed    |
| pub fn wrapping_rem_assign(&mut self, rhs: Self) | removed    |
| pub fn root(self, exp: Self) -> Self             | removed    |
| pub fn into_f64(self) -> f64                     | removed    |
| pub fn into_f32(self) -> f32                     | removed    |
| pub fn into_u128(self) -> u128                   | removed    |
| pub fn into_u64(self) -> u64                     | removed    |
| pub fn into_u32(self) -> u32                     | removed    |
| pub fn into_u16(self) -> u16                     | removed    |
| pub fn into_u8(self) -> u8                       | removed    |
| pub fn into_usize(self) -> usize                 | removed    |
| pub fn into_bool(self) -> bool                   | removed    |
| pub fn size_in_bytes() -> usize                  | removed    |
| pub fn size_in_bits() -> usize                   | removed    |
| pub fn length_in_bytes(self) -> usize            | removed    |
| pub fn length_in_bits(self) -> usize             | removed    |
| pub fn is_odd(self) -> bool                      | removed    |

- Breaking changes have been made to remove the redundant methods from `ShortUnion`, `IntUnion`, `LongUnion`, `LongerUnion`, and `SizeUnions` since there are the same methods in the trait `number::SmallUInt` and its implementation, and/or in order to keep consistency with primitive data types such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

## Breaking change from ver. 0.6.2 to ver. 0.6.3

A breaking change has been made to change the function `number::BigUInt::copy_within<R>(&mut self, src: R, dest: usize)` from public to private since it should have been private from the beginning for security reason because it is high chance that this function will be missused or even abused.

### A method of struct BigUInt

| Ver. 0.6.2                                         | Ver. 0.6.3                                     |
|----------------------------------------------------|------------------------------------------------|
| pub fn copy_within(&mut self, src: R, dest: usize) | fn copy_within(&mut self, src: R, dest: usize) |

## Breaking changes from ver. 0.5.0 to ver. 0.6.0

Breaking changes have been made to change the source code according to Rust convention and in order to remove all warnings.

### Methods of trait SmallUInt and its implementation for u8, u16, u32, u64, u128, usize, ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn test_Miller_Rabin(self, a: Self) -> bool                     | fn test_miller_rabin(self, a: Self) -> bool                     |
| fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool | fn is_prime_using_miller_rabin(self, repetition: usize) -> bool |
| fn u128_as_SmallUInt(n: u128) -> Self                           | fn u128_as_smalluint(n: u128) -> Self                           |
| fn u64_as_SmallUInt(n: u64) -> Self                             | fn u64_as_smalluint(n: u64) -> Self                             |
| fn u32_as_SmallUInt(n: u32) -> Self                             | fn u32_as_smalluint(n: u32) -> Self                             |
| fn u16_as_SmallUInt(n: u16) -> Self                             | fn u16_as_smalluint(n: u16) -> Self                             |
| fn u8_as_SmallUInt(n: u8) -> Self                               | fn u8_as_smalluint(n: u8) -> Self                               |
| fn usize_as_SmallUInt(n: usize) -> Self                         | fn usize_as_smalluint(n: usize) -> Self                         |
| fn bool_as_SmallUInt(n: u8) -> Self                             | fn bool_as_smalluint(n: bool) -> Self                           |
| fn set_MSB(&mut self)                                           | fn set_msb(&mut self)                                           |
| fn set_LSB(&mut self)                                           | fn set_lsb(&mut self)                                           |
| fn is_MSB_set(self) -> bool                                     | fn is_msb_set(self) -> bool                                     |

### Five methods of struct BigUInt functions

| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn test_Miller_Rabin(self, a: Self) -> bool                     | fn test_miller_rabin(self, a: Self) -> bool                     |
| fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool | fn is_prime_using_miller_rabin(self, repetition: usize) -> bool |
| fn set_MSB(&mut self)                                           | fn set_msb(&mut self)                                           |
| fn set_LSB(&mut self)                                           | fn set_lsb(&mut self)                                           |
| fn is_MSB_set(self) -> bool                                     | fn is_msb_set(self) -> bool                                     |

### types of struct BigUInt

| Ver. 0.5.0            | Ver. 0.6.0            |
|-----------------------|-----------------------|
| type U32              | type UU32             |
| type U64              | type UU64             |
| type U128             | type UU128            |
| type U256             | type UU256            |
| type U384             | type UU384            |
| type U512             | type UU512            |
| type U640             | type UU640            |
| type U768             | type UU768            |
| type U896             | type UU896            |
| type U1024            | type UU1024           |
| type U2048            | type UU2048           |
| type u256             | type U256             |
| type u512             | type U512             |
| type u1024            | type U1024            |
| type u2048            | type U2048            |
| type u3072            | type U3072            |
| type u4096            | type U4096            |
| type u5120            | type U5120            |
| type u6144            | type U6144            |
| type u7168            | type U7168            |
| type u8192            | type U8192            |
| type u16384           | type U16384           |
| type u256_with_u8     | type U256_with_u8     |
| type u512_with_u8     | type U512_with_u8     |
| type u1024_with_u8    | type U1024_with_u8    |
| type u2048_with_u8    | type U2048_with_u8    |
| type u3072_with_u8    | type U3072_with_u8    |
| type u4096_with_u8    | type U4096_with_u8    |
| type u5120_with_u8    | type U5120_with_u8    |
| type u6144_with_u8    | type U6144_with_u8    |
| type u7168_with_u8    | type U7168_with_u8    |
| type u8192_with_u8    | type U8192_with_u8    |
| type u16384_with_u8   | type U16384_with_u8   |
| type u256_with_u16    | type U256_with_u16    |
| type u512_with_u16    | type U512_with_u16    |
| type u1024_with_u16   | type U1024_with_u16   |
| type u2048_with_u16   | type U2048_with_u16   |
| type u3072_with_u16   | type U3072_with_u16   |
| type u4096_with_u16   | type U4096_with_u16   |
| type u5120_with_u16   | type U5120_with_u16   |
| type u6144_with_u16   | type U6144_with_u16   |
| type u7168_with_u16   | type U7168_with_u16   |
| type u8192_with_u16   | type U8192_with_u16   |
| type u16384_with_u16  | type U16384_with_u16  |
| type u256_with_u32    | type U256_with_u32    |
| type u512_with_u32    | type U512_with_u32    |
| type u1024_with_u32   | type U1024_with_u32   |
| type u2048_with_u32   | type U2048_with_u32   |
| type u3072_with_u32   | type U3072_with_u32   |
| type u4096_with_u32   | type U4096_with_u32   |
| type u5120_with_u32   | type U5120_with_u32   |
| type u6144_with_u32   | type U6144_with_u32   |
| type u7168_with_u32   | type U7168_with_u32   |
| type u8192_with_u32   | type U8192_with_u32   |
| type u16384_with_u32  | type U16384_with_u32  |
| type u256_with_u64    | type U256_with_u64    |
| type u512_with_u64    | type U512_with_u64    |
| type u1024_with_u64   | type U1024_with_u64   |
| type u2048_with_u64   | type U2048_with_u64   |
| type u3072_with_u64   | type U3072_with_u64   |
| type u4096_with_u64   | type U4096_with_u64   |
| type u5120_with_u64   | type U5120_with_u64   |
| type u6144_with_u64   | type U6144_with_u64   |
| type u7168_with_u64   | type U7168_with_u64   |
| type u8192_with_u64   | type U8192_with_u64   |
| type u16384_with_u64  | type U16384_with_u64  |
| type u256_with_u128   | type U256_with_u128   |
| type u512_with_u128   | type U512_with_u128   |
| type u1024_with_u128  | type U1024_with_u128  |
| type u2048_with_u128  | type U2048_with_u128  |
| type u3072_with_u128  | type U3072_with_u128  |
| type u4096_with_u128  | type U4096_with_u128  |
| type u5120_with_u128  | type U5120_with_u128  |
| type u6144_with_u128  | type U6144_with_u128  |
| type u7168_with_u128  | type U7168_with_u128  |
| type u8192_with_u128  | type U8192_with_u128  |
| type u16384_with_u128 | type U16384_with_u128 |

### Two methods of trait SmallSInt and its implementation for i8, i16, i32, i64, i128, and isize

| Ver. 0.5.0       | Ver. 0.6.0       |
|------------------|------------------|
| fn Max() -> Self | fn max() -> Self |
| fn Min() -> Self | fn min() -> Self |

### Methods of struct MD4_Generic, MD5_Generic, SHA1_Generic, SHA2_256_Generic, SHA2_512_Generic, and SHA2_512_t_Generic

| Ver. 0.5.0                                                 | Ver. 0.6.0                                                   |
|------------------------------------------------------------|--------------------------------------------------------------|
| fn get_HashValue(&self, hashValue: *mut u8, length: usize) | fn get_hash_value(&self, hash_value: *mut u8, length: usize) |
| fn get_HashValue_in_string(&self) -> String                | fn get_hash_value_in_string(&self) -> String                 |
| fn get_HashValue_in_array(&self) -> [u32; N]               | fn get_hash_value_in_array(&self) -> [u32; N]                |
| fn get_HashValue_in_vec(&self) -> Vec                      | fn get_hash_value_in_vec(&self) -> Vec                       |
| fn put_HashValue_in_array(&self, out: &mut [T; M])         | fn put_hash_value_in_array(&self, out: &mut [T; M])          |
| fn digest_C(&mut self, ...)                                | fn digest_c(&mut self, ...)                                  |

### Three methods of struct SHA2_512_t_Generic

| Ver. 0.5.0                                    | Ver. 0.6.0                                     |
|-----------------------------------------------|------------------------------------------------|
| fn get_HashValue_in_array_TM(&self) -> [T; M] | fn get_hash_value_in_array_tm(&self) -> [T; M] |
| fn new_with_seedText(seed_text: &str) -> Self | fn new_with_seed_text(seed_text: &str) -> Self |
| fn new_with_H(h: &[u64; 8]) -> Self           | fn new_with_h(h: &[u64; 8]) -> Self            |

### Methods of struct Random

| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn random_with_MSB_set_uint(&mut self) -> T                     | fn random_with_msb_set_uint(&mut self) -> T                     |
| fn random_odd_with_MSB_set_uint(&mut self) -> T                 | fn random_odd_with_msb_set_uint(&mut self) -> T                 |
| fn random_prime_using_Miller_Rabin_uint(&mut self, ... ) -> T   | fn random_prime_using_miller_rabin_uint(&mut self, ... ) -> T   |
| fn random_prime_with_MSB_set_using_Miller_Rabin_uint(...) -> T  | fn random_prime_with_msb_set_using_miller_rabin_uint(...) -> T  |
| fn random_BigUInt(&mut self) -> BigUInt<T, N>                   | fn random_biguint(&mut self) -> BigUInt<T, N>                   |
| fn random_under_BigUInt( ... ) -> Option<BigUInt<T, N>>         | fn random_under_biguint( ... ) -> Option<BigUInt<T, N>>         |
| fn random_under_BigUInt_( ... ) -> Option<BigUInt<T, N>>        | fn random_under_biguint_( ... ) -> Option<BigUInt<T, N>>        |
| fn random_odd_BigUInt(&mut self) -> BigUInt<T, N>               | fn random_odd_biguint(&mut self) -> BigUInt<T, N>               |
| fn random_odd_under_BigUInt( ... ) -> Option<BigUInt<T, N>>     | fn random_odd_under_biguint( ... ) -> Option<BigUInt<T, N>>     |
| fn random_odd_under_BigUInt_( ... ) -> Option<BigUInt<T, N>>    | fn random_odd_under_biguint_( ... ) -> Option<BigUInt<T, N>>    |
| fn random_with_MSB_set_BigUInt(&mut self) -> BigUInt<T, N>      | fn random_with_msb_set_biguint(&mut self) -> BigUInt<T, N>      |
| fn random_odd_with_MSB_set_BigUInt(&mut self) -> BigUInt<T, N>  | fn random_odd_with_msb_set_biguint(&mut self) -> BigUInt<T, N>  |
| fn random_prime_using_Miller_Rabin_BigUInt(..) -> BigUInt<T, N> | fn random_prime_using_miller_rabin_biguint(..) -> BigUInt<T, N> |
| fn random_prime_with_MSB_set_using_Miller_Rabin_BigUInt(.)-> .. | fn random_prime_with_msb_set_using_miller_rabin_biguint(.)-> .. |
