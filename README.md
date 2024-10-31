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
The checked items have already been implemented including documentation __at least 95%__. The unchecked items have been implemented including documentation __less than 95%__ or have __not__ yet even been started to implement.

### Small Numbers: meaningful as itself, and also the foundations mainly for Big Numbers as well as for other modules

- [X] Unions for primitive data types and their implementation, and the implementation
      of trait SmallUInt for the Unions --- 
      [`ShortUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/short_union/union.ShortUnion.html#union.ShortUnion),
      [`IntUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/int_union/union.IntUnion.html#union.IntUnion),
      [`LongUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/long_union/union.LongUnion.html#union.LongUnion),
      [`LongerUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/longer_union/union.LongerUnion.html#union.LongerUnion),
      [`SizeUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/size_union/union.SizeUnion.html#union.SizeUnion),
      [`SharedValues`](https://docs.rs/cryptocol/latest/cryptocol/number/shared_values/union.SharedValues.html#union.SharedValues), and
      [`SharedArrays`](https://docs.rs/cryptocol/latest/cryptocol/number/shared_arrays/union.SharedArrays.html#union.SharedArrays)
- [X] Trait SmallUInt, its implementation for primitive data types, and the implementation
      of it for the Unions --- [`SmallUInt`](https://docs.rs/cryptocol/latest/cryptocol/number/small_uint/trait.SmallUInt.html#trait.SmallUInt)

<!--
- [ ] Trait SmallUInt and its implementation for primitive data types --- SmallUInt
    ===> Moved to Roadmap for ver. 2.0
-->

### Big Numbers: meaningful as itself and also the foundation for Asymmetric-Key Algorithms

- [ ] Fixed Sized Big Unsigned Integer Operation --- [`BigUInt`](https://docs.rs/cryptocol/latest/cryptocol/number/big_uint/struct.BigUInt.html#struct.BigUInt)

<!--
- [ ] Fixed Sized Big Signed Integer Operation --- BigSInt
    ===> Moved to Roadmap for ver. 2.0
- [ ] Variable Sized Big Signed Integer Operation --- LargeInt
    ===> Moved to Roadmap for ver. 2.0 or higher
-->

### Hash Algorithms

<!--
- [ ] MD2 hash algorithms based on 128 bits
    --- Includes MD4 and its expanded versions.
    ===> Moved to Roadmap for ver. 2.0
-->

- [X] MD4 hash algorithms based on 128 bits
    --- Includes MD4 and its expanded versions.
    [`MD4_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/md4/struct.MD4_Generic.html#struct.MD4_Generic)
- [X] MD5 hash algorithms based on 128 bits
    --- Includes MD5 and its expanded versions.
    [`MD5_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/md5/struct.MD5_Generic.html#struct.MD5_Generic)

<!--
- [ ] MD6 hash algorithms based on 256 bits
    --- Includes MD4 and its expanded versions.
    ===> Moved to Roadmap for ver. 2.0
-->

- [X] SHA-1 hash algorithms based on 160 bits
    --- Includes SHA-1, SHA-0, and their expanded versions.
    [`SHA1_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha1/struct.SHA1_Generic.html#struct.SHA1_Generic)
- [X] SHA-2 hash algorithms based on 256 bits
    --- Includes SHA-256, SHA-224, and their expanded versions.
    [`SHA2_256_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_256/struct.SHA2_256_Generic.html#struct.SHA2_256_Generic)
- [X] SHA-2 hash algorithms based on 512 bits
    --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions. [`SHA2_512_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_512/struct.SHA2_512_Generic.html#struct.SHA2_512_Generic)
- [X] SHA-2 hash algorithms based on 512/t bits
    --- Includes 512/256, SHA-512/224, and their expanded versions.
    [`SHA2_512_t_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_512_t/struct.SHA2_512_t_Generic.html#struct.SHA2_512_t_Generic)
- [ ] SHA-3 (SHA3-224)
- [ ] SHA-3 (SHA3-256)
- [ ] SHA-3 (SHA3-384)
- [ ] SHA-3 (SHA3-512)
- [ ] SHA-3 (SHAKE 128)
- [ ] SHA-3 (SHAKE 256)

<!--
- [ ] RIPEMD hash algorithms based on 256 bits
    --- Includes RIPEMD and its expanded versions.
    ===> Moved to Roadmap for ver. 2.0
- [ ] BLAKE2 hash algorithms based on 256 bits
    --- Includes BLAKE2 and its expanded versions.
    ===> Moved to Roadmap for ver. 2.0
- [ ] BLAKE3 hash algorithms based on 256 bits
    --- Includes BLAKE3 and its expanded versions.
    ===> Moved to Roadmap for ver. 2.0
-->

### Symmetric-key Algorithms for the Encryption/Decryption of digital data

<!--
- [ ] Lucifer symmetric-key encryption/decryption algorithm
    --- Includes Lucifer and its expanded versions. `Lucifer_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] Bluefish symmetric-key encryption/decryption algorithm
    --- Includes Bluefish and its expanded versions. `Bluefish_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] Twofish symmetric-key encryption/decryption algorithm
    --- Includes Twofish and its expanded versions. `Twofish_Generic`
    ===> Moved to Roadmap for ver. 2.0
-->

- [ ] DES symmetric-key encryption/decryption algorithm
    --- Includes DES and its expanded versions. `DES_Generic`
- [ ] NDES symmetric-key encryption/decryption algorithm
    --- Includes 2DES, 3DES, 4DES, etc., and their expanded versions. `NDES_Generic`
- [ ] AES symmetric-key encryption/decryption algorithm
    --- Includes AES and its expanded versions. `AES_Generic`
- [ ] NAES symmetric-key encryption/decryption algorithm
    --- Includes 2DES, 3DES, 4AES, etc., and their expanded versions. `NAES_Generic`

<!--
- [ ] SEED symmetric-key encryption/decryption algorithm
    --- Includes SEED and its expanded versions. `SEED_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] HIGHT symmetric-key encryption/decryption algorithm
    --- Includes HIGHT and its expanded versions. `HIGHT_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] ARIA symmetric-key encryption/decryption algorithm
    --- Includes ARIA and its expanded versions. `ARIA_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] LEA symmetric-key encryption/decryption algorithm
    --- Includes LEA and its expanded versions. `LEA_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] RC2 symmetric-key encryption/decryption algorithm
    --- Includes RC2 and its expanded versions. `RC2_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] RC4 symmetric-key encryption/decryption algorithm
    --- Includes RC4 and its expanded versions. `RC4_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] RC5 symmetric-key encryption/decryption algorithm
    --- Includes RC5 and its expanded versions. `RC5_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] RC6 symmetric-key encryption/decryption algorithm
    --- Includes RC6 and its expanded versions. `RC6_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] Salsa20 symmetric-key encryption/decryption algorithm
    --- Includes Salsa20 and its expanded versions. `Salsa20_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] Chacha20 symmetric-key encryption/decryption algorithm
    --- Includes Chacha20 and its expanded versions. `Chacha20_Generic`
    ===> Moved to Roadmap for ver. 2.0
- [ ] IDEA symmetric-key encryption/decryption algorithm
    --- Includes IDEA and its expanded versions. `IDEA_Generic`
    ===> Moved to Roadmap for ver. 2.0
-->

### Pseudo-Random Number Generator Algorithms

- [ ] Pseudo-random number generator ---
    struct [`Random_Generic`](https://docs.rs/cryptocol/latest/cryptocol/random/random/struct.Random_Generic.html#struct.Random_Generic) and
    trait [`Random_Engine`](https://docs.rs/cryptocol/latest/cryptocol/random/trait_random_engine/trait.Random_Engine.html#trait.Random_Engine)
- [ ] Pseudo-random number generator engines using hash algorithms ---
    [`Any_MD4`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_MD4.html#type.Any_MD4),
    [`Any_MD5`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_MD5.html#type.Any_MD5),
    [`Any_SHA0`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_SHA0.html#type.Any_SHA0),
    [`Any_SHA1`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_SHA1.html#type.Any_SHA1),
    [`Any_SHA2_256`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_SHA2_256.html#type.Any_SHA2_256),
    [`Any_SHA2_512`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_SHA2_512.html#type.Any_SHA2_512), and
    [`Random_SHA2_512`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Random_SHA2_512.html#type.Random_SHA2_512).
- [ ] Pseudo-random number generator engines using symmetric-key encryption algorithms ---
    `Any_DES`, `Any_NDES`, `Any_AES`, `Any_NAES`, `Random_AES`, and `Random_NAES`.
- [ ] Pseudo-random number generator engines using simple randomization algorithm ---
    [`Any_Num_C`](https://docs.rs/cryptocol/latest/cryptocol/random/random/type.Any_Num_C.html#type.Any_Num_C)

### Asymmetric-key Algorithms for the Encryption/Decryption of digital data

<!--
- [ ] Diffie-Hellman
    ===> Moved to Roadmap for ver. 2.0
- [ ] ElGamal
    ===> Moved to Roadmap for ver. 2.0
-->

- [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
- [ ] ECC (Elliptic Curve Cryptosystem)

<!--
- [ ] Rabin
    ===> Moved to Roadmap for ver. 2.0
-->

When the implementation of all the above functionalitis are completed,
the version number 1.0.0.0 will be given. After that whenever another
functionality is added to this crate, the version number will get higher
beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
number will be 0.25.x.x since there are all twenty-five functionalities
listed above. So, for example, even if the version number is 0.5.0.0,
it does not mean that 50% of all functionalities are implemented.

## Breaking Changes

Refer to the file `BreakingChanges.md` in this crate.
