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
The checked items have already been implemented including documentation at least 95%. The unchecked items have been implemented including documentation less than 95% or have not yet even been started to implement.

### Small Numbers: meaningful as itself, and also the foundations mainly for Big Numbers as well as for other modules

- [X] Unions for primitive data types and their implementation, and the implementation
      of trait SmallUInt for the Unions --- `ShortUnion`, `IntUnion`, `LongUnion`,
      `LongerUnion`, `SizeUnion`, `SharedValues`, and `SharedArrays`
- [X] Trait SmallUInt, its implementation for primitive data types, and the implementation
      of it for the Unions --- `SmallUInt`

### Big Numbers: meaningful as itself and also the foundation for Asymmetric-Key Algorithms

- [ ] Fixed Sized Big Unsigned Integer Operation --- `BigUInt`

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

- [ ] Pseudo-random number generator --- struct `Random_Generic` and trait `Random_Engine`
- [ ] Pseudo-random number generator engines using hash algorithms ---
    `Any_MD4`, `Any_MD5`, `Any_SHA0`, `Any_SHA1`, `Any_SHA2_256`, `Any_SHA2_512`, and `Random_SHA2_512`.
- [ ] Pseudo-random number generator engines using symmetric-key encryption algorithms ---
    `Any_DES`, `Any_NDES`, `Any_AES`, `Any_NAES`, `Random_AES`, and `Random_NAES`.
- [ ] Pseudo-random number generator engines using simple randomization algorithm ---
    `Any_Num_C`

### Asymmetric-key Algorithms for the Encryption/Decryption of digital data

- [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
- [ ] ECC (Elliptic Curve Cryptosystem)

When the implementation of all the above functionalitis are completed,
the version number 1.0.0.0 will be given. After that whenever another
functionality is added to this crate, the version number will get higher
beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
number will be 0.25.x.x since there are all twenty-five functionalities
listed above. So, for example, even if the version number is 0.5.0.0,
it does not mean that 50% of all functionalities are implemented.
