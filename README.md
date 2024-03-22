# cryptocol crate provides libraries for cryptography

## Endianness

This crate is optimized for Little-endian CPUs because Little-Endian CPUs
are far more popular than Big-endian CPUs. For the information about
Endianness (including Little-endian and Big-endian)
[Read more](https://en.wikipedia.org/wiki/Endianness).

## Big-endian issue

This crate is just experimental for Big-endian CPUs. So, you are not
encouraged to use this crate for Big-endian CPUs for serious purpose.
Only use this crate for Big-endian CPUs with your own full responsibility.

## Road Map for Version 1.0

This crate Cryptocol is planned to provide the following functionalities.
The checked items have already been implemented including documentation
at least 80%. The unchecked items have not yet been implemented including
documentation more than 80% or have not yet even been started to implement.

### Small Numbers: meaningful itself, and also the foundations mainly for Big Numbers as well as for other modules

- [ ] Unions for primitive data types and their implementation, and the implementation
      of trait SmallUInt for the Unions --- `ShortUnion`, `IntUnion`, `LongUnion`,
      `LongerUnion`, `SizeUnion`, `SharedValues`, and `SharedArrays`
- [X] Trait SmallUInt, its implementation for primitive data types, and the implementation
      of it for the Unions --- `SmallUInt`
- [ ] Trait SmallSInt, its implementation for primitive data types, and the implementation
      of it for the Unions --- SmallSInt
      _--> Thinking about postponing to Roadmap for ver. 2.0_

### Big Numbers: meaningful as itself and also the foundation for Asymmetric-Key Algorithms

- [ ] Fixed Sized Big Unsigned Integer Operation --- `BigUInt`
- [ ] Fixed Sized Big Signed Integer Operation --- `BigSInt`
    _--> Thinking about postponing to Roadmap for ver. 2.0_
- [ ] Variable Sized Big Signed Integer Operation --- `LargeInt`
    _--> Thinking about postponing to Roadmap for ver. 2.0 or higher_

### Hash Algorithms

- [X] MD4 hash algorithms based on 128 bits
    --- Includes MD4 and its expanded versions. `MD4_Generic`
- [X] MD5 hash algorithms based on 128 bits
    --- Includes MD5 and its expanded versions. `MD5_Generic`
- [X] SHA-1 hash algorithms based on 160 bits
    --- Includes SHA-1, SHA-0, and their expanded versions. `SHA1_Generic`
- [X] SHA-2 hash algorithms based on 256 bits
    --- Includes SHA-256, SHA-224, and their expanded versions. `SHA2_256_Generic`
- [X] SHA-2 hash algorithms based on 512 bits
    --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions. `SHA2_512_Generic`
- [X] SHA-2 hash algorithms based on 512/t bits
    --- Includes 512/256, SHA-512/224, and their expanded versions. `SHA2_512_t_Generic`
- [ ] SHA-3 (SHA3-224)
- [ ] SHA-3 (SHA3-256)
- [ ] SHA-3 (SHA3-384)
- [ ] SHA-3 (SHA3-512)
- [ ] SHA-3 (SHAKE 128)
- [ ] SHA-3 (SHAKE 256)

### Symmetric-key Algorithms for the Encryption/Decryption of digital data

- [ ] DES symmetric-key encryption/decryption algorithm
    --- Includes DES and its expanded versions. `DES_Generic`
- [ ] NDES symmetric-key encryption/decryption algorithm
    --- Includes 2DES, 3DES, 4DES, etc., and their expanded versions. `NDES_Generic`
- [ ] AES symmetric-key encryption/decryption algorithm
    --- Includes AES and its expanded versions. `AES_Generic`
- [ ] NAES symmetric-key encryption/decryption algorithm
    --- Includes 2DES, 3DES, 4AES, etc., and their expanded versions. `NAES_Generic`

### Pseudo-Random Number Generator Algorithms

- [X] The Pseudo-random number generator wrappers
    --- struct `Random_Generic` and trait `Random_Engine`
- [X] The implementation of `Random_Engine` for hash algorithms such as
    `MD4_Generic`, `MD5_Generic`, `SHA1_Generic`, `SHA2_256_Generic`,
    and`SHA2_512_Generic`,
    and for pseudo-random number generation algorithm such as `AnyNumber`

### Asymmetric-key Algorithms for the Encryption/Decryption of digital data

- [ ] Diffie-Hellman _--> Thinking about postponing to Roadmap for ver. 2.0_
- [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
- [ ] ECC (Elliptic Curve Cryptosystem)

When the implementation of all the above functionalitis are completed,
the version number 1.0.0.0 will be given. After that whenever another
functionality is added to this crate, the version number will get higher
beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
number will be 0.27.x.x since there are all twenty-six functionalities
listed above. So, for example, even if the version number is 0.5.0.0,
it does not mean that 50% of all functionalities are implemented.

## Sorry for breaking changes from ver. 0.7.2 to ver. 0.7.3

I think that if breaking changes are inevitable and have to be made, the sooner, the better even before more people will be using my crate.

### methods of BigUInt

| Ver. 0.7.2                                      | Ver. 0.7.3                                     |
|-------------------------------------------------|------------------------------------------------|
| pub fn from_array(val: &[T; N]) -> Self         | pub fn from_array(val: [T; N]) -> Self         |
| pub fn from_be(be: &Self) -> Self               | pub fn from_be(be: Self) -> Self               |
| pub fn from_le(le: S&elf) -> Self               | pub fn from_le(le: Self) -> Self               |
| pub fn from_be_bytes(be_bytes: &[T; N]) -> Self | pub fn from_be_bytes(be_bytes: [T; N]) -> Self |
| pub fn from_le_bytes(le_bytes: &[T; N]) -> Self | pub fn from_le_bytes(le_bytes: [T; N]) -> Self |

## Sorry for breaking changes from ver. 0.7.1 to ver. 0.7.2

### union SharedValues

| Ver. 0.7.1                                          | Ver. 0.7.2 |
|-----------------------------------------------------|------------|
| pub fn into_des(&mut self, pos: usize) -> Option&lt;D&gt; | removed    |

### union SharedArray

| Ver. 0.7.1                                   | Ver. 0.7.2                                           |
|----------------------------------------------|------------------------------------------------------|
| pub fn into_des(&mut self, des: &mut [D; N]) | pub fn put_des_in_array(&self, des: &mut [D; N]) |

- The function name `into_des()` does not show its functionality very clearly. It is desirable that the function name `into_des()` is changed to put_des_in_array() for the name consistency with the methods such as put_hash_value_in_array() in hash modules. The argument `&mut self` does not have to be `&mut self` so that it was changed to `&self`

## Sorry for breaking changes from ver. 0.7.0 to ver. 0.7.1

### macros for BigUInt

| Ver. 0.7.0                | Ver. 0.7.1 |
|---------------------------|------------|
| define_Utypes_with_utypes | removed    |

- The macro name `define_Utypes_with_utypes` should not have been exposed to the outside of this crate so it was removed rather than recommending not to use it at documentation.

### fields of ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion

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

## Sorry for breaking changes from ver. 0.6.3 to ver. 0.7.0

### trait SmallUInt

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

### unions ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

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

## Sorry for a breaking change from ver. 0.6.2 to ver. 0.6.3

A breaking change has been made to change the function `number::BigUInt::copy_within<R>(&mut self, src: R, dest: usize)` from public to private since it should have been private from the beginning for security reason because it is high chance that this function will be missused or even abused.

### struct BigUInt

| Ver. 0.6.2                                                  | Ver. 0.6.3                                              |
|-------------------------------------------------------------|---------------------------------------------------------|
| pub fn copy_within&lt;R&gt;(&mut self, src: R, dest: usize) | fn copy_within&lt;R&gt;(&mut self, src: R, dest: usize) |

## Sorry for breaking changes from ver. 0.5.0 to ver. 0.6.0

Breaking changes have been made to change the source code according to Rust convention and in order to remove all warnings.

### trait SmallUInt and its implementation for u8, u16, u32, u64, u128, usize, ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion

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

### struct BigUInt functions

| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn test_Miller_Rabin(self, a: Self) -> bool                     | fn test_miller_rabin(self, a: Self) -> bool                     |
| fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool | fn is_prime_using_miller_rabin(self, repetition: usize) -> bool |
| fn set_MSB(&mut self)                                           | fn set_msb(&mut self)                                           |
| fn set_LSB(&mut self)                                           | fn set_lsb(&mut self)                                           |
| fn is_MSB_set(self) -> bool                                     | fn is_msb_set(self) -> bool                                     |

### struct BigUInt types

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

### trait SmallSInt and its implementation for i8, i16, i32, i64, i128, and isize

| Ver. 0.5.0       | Ver. 0.6.0       |
|------------------|------------------|
| fn Max() -> Self | fn max() -> Self |
| fn Min() -> Self | fn min() -> Self |

### struct MD4_Generic, MD5_Generic, SHA1_Generic, SHA2_256_Generic, SHA2_512_Generic, and SHA2_512_t_Generic

| Ver. 0.5.0                                                 | Ver. 0.6.0                                                   |
|------------------------------------------------------------|--------------------------------------------------------------|
| fn get_HashValue(&self, hashValue: *mut u8, length: usize) | fn get_hash_value(&self, hash_value: *mut u8, length: usize) |
| fn get_HashValue_in_string(&self) -> String                | fn get_hash_value_in_string(&self) -> String                 |
| fn get_HashValue_in_array(&self) -> [u32; N]               | fn get_hash_value_in_array(&self) -> [u32; N]                |
| fn get_HashValue_in_vec(&self) -> Vec                      | fn get_hash_value_in_vec(&self) -> Vec                       |
| fn put_HashValue_in_array(&self, out: &mut [T; M])         | fn put_hash_value_in_array(&self, out: &mut [T; M])          |
| fn digest_C(&mut self, ...)                                | fn digest_c(&mut self, ...)                                  |

### struct SHA2_512_t_Generic

| Ver. 0.5.0                                    | Ver. 0.6.0                                     |
|-----------------------------------------------|------------------------------------------------|
| fn get_HashValue_in_array_TM(&self) -> [T; M] | fn get_hash_value_in_array_tm(&self) -> [T; M] |
| fn new_with_seedText(seed_text: &str) -> Self | fn new_with_seed_text(seed_text: &str) -> Self |
| fn new_with_H(h: &[u64; 8]) -> Self           | fn new_with_h(h: &[u64; 8]) -> Self            |

### struct Random

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
