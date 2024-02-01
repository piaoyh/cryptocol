# cryptocol crate provides libraries for cryptography.

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

### Foundations mainly for Big Numbers and also for other modules
- [ ] Unions for primitive data types and their implementation
    --- `ShortUnion`, `IntUnion`, `LongUnion`, `LongerUnion`, and `SizeUnion`
- [ ] Trait SmallUInt and its implementation for primitive data types --- SmallUInt
- [ ] Trait SmallSInt and its implementation for primitive data types --- SmallSInt
    _--> Thinking about postponing to Roadmap for ver. 2.0_

### Big Numbers
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

### Symmetric-Key Cryptographic Algorithms
- [ ] DES
- [ ] 3DES
- [ ] AES

### Pseudo-Random Number Generator Algorithms
- [X] The Pseudo-random number generator wrappers
    --- struct `Random_Generic` and trait `Random_Engine`
- [X] The implementation of `Random_Engine` for hash algorithms such as
    `MD4_Generic`, `MD5_Generic`, `SHA1_Generic`, `SHA2_256_Generic`,
    and`SHA2_512_Generic`,
    and for pseudo-random number generation algorithm such as `AnyNumber`

### Asymmetric-Key Cryptographic Algorithms
- [ ] Diffie-Hellman _--> Thinking about postponing to Roadmap for ver. 2.0_
- [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
- [ ] ECC (Elliptic Curve Cryptosystem)

When the implementation of all the above functionalitis are completed,
the version number 1.0.0.0 will be given. After that whenever another
functionality is added to this crate, the version number will get higher
beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
number will be 0.26.x.x since there are all twenty-six functionalities
listed above. So, for example, even if the version number is 0.5.0.0,
it does not mean that 50% of all functionalities are implemented.

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

### struct BigUInt
| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn test_Miller_Rabin(self, a: Self) -> bool                     | fn test_miller_rabin(self, a: Self) -> bool                     |
| fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool | fn is_prime_using_miller_rabin(self, repetition: usize) -> bool |
| fn set_MSB(&mut self)                                           | fn set_msb(&mut self)                                           |
| fn set_LSB(&mut self)                                           | fn set_lsb(&mut self)                                           |
| fn is_MSB_set(self) -> bool                                     | fn is_msb_set(self) -> bool                                     |
| type U32                                                        | type UU32                                                       |
| type U64                                                        | type UU64                                                       |
| type U128                                                       | type UU128                                                      |
| type U256                                                       | type UU256                                                      |
| type U384                                                       | type UU384                                                      |
| type U512                                                       | type UU512                                                      |
| type U640                                                       | type UU640                                                      |
| type U768                                                       | type UU768                                                      |
| type U896                                                       | type UU896                                                      |
| type U1024                                                      | type UU1024                                                     |
| type U2048                                                      | type UU2048                                                     |
| type u256                                                       | type U256                                                       |
| type u512                                                       | type U512                                                       |
| type u1024                                                      | type U1024                                                      |
| type u2048                                                      | type U2048                                                      |
| type u3072                                                      | type U3072                                                      |
| type u4096                                                      | type U4096                                                      |
| type u5120                                                      | type U5120                                                      |
| type u6144                                                      | type U6144                                                      |
| type u7168                                                      | type U7168                                                      |
| type u8192                                                      | type U8192                                                      |
| type u16384                                                     | type U16384                                                     |
| type u256_with_u8                                               | type U256_with_u8                                               |
| type u512_with_u8                                               | type U512_with_u8                                               |
| type u1024_with_u8                                              | type U1024_with_u8                                              |
| type u2048_with_u8                                              | type U2048_with_u8                                              |
| type u3072_with_u8                                              | type U3072_with_u8                                              |
| type u4096_with_u8                                              | type U4096_with_u8                                              |
| type u5120_with_u8                                              | type U5120_with_u8                                              |
| type u6144_with_u8                                              | type U6144_with_u8                                              |
| type u7168_with_u8                                              | type U7168_with_u8                                              |
| type u8192_with_u8                                              | type U8192_with_u8                                              |
| type u16384_with_u8                                             | type U16384_with_u8                                             |
| type u256_with_u16                                              | type U256_with_u16                                              |
| type u512_with_u16                                              | type U512_with_u16                                              |
| type u1024_with_u16                                             | type U1024_with_u16                                             |
| type u2048_with_u16                                             | type U2048_with_u16                                             |
| type u3072_with_u16                                             | type U3072_with_u16                                             |
| type u4096_with_u16                                             | type U4096_with_u16                                             |
| type u5120_with_u16                                             | type U5120_with_u16                                             |
| type u6144_with_u16                                             | type U6144_with_u16                                             |
| type u7168_with_u16                                             | type U7168_with_u16                                             |
| type u8192_with_u16                                             | type U8192_with_u16                                             |
| type u16384_with_u16                                            | type U16384_with_u16                                            |
| type u256_with_u32                                              | type U256_with_u32                                              |
| type u512_with_u32                                              | type U512_with_u32                                              |
| type u1024_with_u32                                             | type U1024_with_u32                                             |
| type u2048_with_u32                                             | type U2048_with_u32                                             |
| type u3072_with_u32                                             | type U3072_with_u32                                             |
| type u4096_with_u32                                             | type U4096_with_u32                                             |
| type u5120_with_u32                                             | type U5120_with_u32                                             |
| type u6144_with_u32                                             | type U6144_with_u32                                             |
| type u7168_with_u32                                             | type U7168_with_u32                                             |
| type u8192_with_u32                                             | type U8192_with_u32                                             |
| type u16384_with_u32                                            | type U16384_with_u32                                            |
| type u256_with_u64                                              | type U256_with_u64                                              |
| type u512_with_u64                                              | type U512_with_u64                                              |
| type u1024_with_u64                                             | type U1024_with_u64                                             |
| type u2048_with_u64                                             | type U2048_with_u64                                             |
| type u3072_with_u64                                             | type U3072_with_u64                                             |
| type u4096_with_u64                                             | type U4096_with_u64                                             |
| type u5120_with_u64                                             | type U5120_with_u64                                             |
| type u6144_with_u64                                             | type U6144_with_u64                                             |
| type u7168_with_u64                                             | type U7168_with_u64                                             |
| type u8192_with_u64                                             | type U8192_with_u64                                             |
| type u16384_with_u64                                            | type U16384_with_u64                                            |
| type u256_with_u128                                             | type U256_with_u128                                             |
| type u512_with_u128                                             | type U512_with_u128                                             |
| type u1024_with_u128                                            | type U1024_with_u128                                            |
| type u2048_with_u128                                            | type U2048_with_u128                                            |
| type u3072_with_u128                                            | type U3072_with_u128                                            |
| type u4096_with_u128                                            | type U4096_with_u128                                            |
| type u5120_with_u128                                            | type U5120_with_u128                                            |
| type u6144_with_u128                                            | type U6144_with_u128                                            |
| type u7168_with_u128                                            | type U7168_with_u128                                            |
| type u8192_with_u128                                            | type U8192_with_u128                                            |
| type u16384_with_u128                                           | type U16384_with_u128                                           |

## trait SmallSInt and its implementation for i8, i16, i32, i64, i128, and isize
| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn Max() -> Self                                                | fn max() -> Self                                                |
| fn Min() -> Self                                                | fn min() -> Self                                                |

### struct MD4_Generic, MD5_Generic, SHA1_Generic, SHA2_256_Generic, SHA2_512_Generic, and SHA2_512_t_Generic
| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn get_HashValue(&self, hashValue: *mut u8, length: usize)      | fn get_hash_value(&self, hash_value: *mut u8, length: usize)    |
| fn get_HashValue_in_string(&self) -> String                     | fn get_hash_value_in_string(&self) -> String                    |
| fn get_HashValue_in_array(&self) -> [u32; N]                    | fn get_hash_value_in_array(&self) -> [u32; N]                   |
| fn get_HashValue_in_vec(&self) -> Vec                           | fn get_hash_value_in_vec(&self) -> Vec                          |
| fn put_HashValue_in_array(&self, out: &mut [T; M])              | fn put_hash_value_in_array(&self, out: &mut [T; M])             |
| fn digest_C(&mut self, ...)                                     | fn digest_c(&mut self, ...)                                     |

### struct SHA2_512_t_Generic
| Ver. 0.5.0                                                      | Ver. 0.6.0                                                      |
|-----------------------------------------------------------------|-----------------------------------------------------------------|
| fn get_HashValue_in_array_TM(&self) -> [T; M]                   | fn get_hash_value_in_array_tm(&self) -> [T; M]                  |
| fn new_with_seedText(seed_text: &str) -> Self                   | fn new_with_seed_text(seed_text: &str) -> Self                   |
| fn new_with_H(h: &[u64; 8]) -> Self                             | fn new_with_h(h: &[u64; 8]) -> Self                             |

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





