// Copyright 2023, 2024 PARK Youngho.hort_union_
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]
#![allow(non_camel_case_types)]


pub fn main()
{
    short_union_main();
    int_union_main();
    long_union_main();
    longer_union_main();
    size_union_main();
    shared_values_main();
    shared_arrays_main();
    unions_get_set_byte_main();
    unions_get_set_short_main();
    unions_get_set_int_main();
    unions_get_set_long_main();
    unions_get_set_size_main();
    unions_add_main();
    unions_sub_main();
    unions_mul_main();
    unions_div_main();
    unions_rem_main();
    unions_neg_main();
    unions_pow_main();
    unions_log_main();
    unions_root_main();
    unions_prime_main();
    unions_bits_operation();
    unions_bytes_operation();
    unions_find_power();
    unions_conversion();
    unions_constants();
    unions_size();
    end();
}

fn short_union_main()
{
    short_union_quick_start1();
    short_union_quick_start2();
    short_union_new();
    short_union_new_with();
    short_union_new_with_signed();
    short_union_new_with_ubytes();
    short_union_new_with_u128();
    short_union_new_with_bool();
    short_union_zero();
    short_union_one();
    short_union_get();
    short_union_get_signed();
    short_union_set();
    short_union_set_signed();
    short_union_get_ushort();
    short_union_get_sshort();
    short_union_set_ushort();
    short_union_set_sshort();
}

fn short_union_quick_start1()
{
    println!("short_union_quick_start1()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::new_with(55468_u16);
    println!("a.get() = {}", a.get());
    println!("a.get_signed() = {}", a.get_signed());
    println!("a.get_ushort() = {}", a.get_ushort());
    println!("a.get_sshort() = {}", a.get_sshort());
    assert_eq!(a.get(), 55468_u16);
    assert_eq!(a.get_signed(), -10068_i16);
    assert_eq!(a.get_ushort(), 55468_u16);
    assert_eq!(a.get_sshort(), -10068_i16);

    for i in 0..2
        { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
    for i in 0..2
        { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
    assert_eq!(a.get_ubyte_(0), 172_u8);
    assert_eq!(a.get_ubyte_(1), 216_u8);
    assert_eq!(a.get_sbyte_(0), -84_i8);
    assert_eq!(a.get_sbyte_(1), -40_i8);
    
    #[cfg(target_pointer_width = "8")]
    {
        const N: usize = 2;
        for i in 0..N
            { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
        for i in 0..N
            { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
        assert_eq!(a.get_usize_(0), 172_usize);
        assert_eq!(a.get_usize_(1), 216_usize);
        assert_eq!(a.get_usize_(0), -84_isize);
        assert_eq!(a.get_usize_(1), -40_isize);
    }

    #[cfg(target_pointer_width = "16")]
    {
        println!("a.get_usize() = {}", a.get_usize());
        println!("a.get_ssize() = {}", a.get_ssize());
        assert_eq!(a.get_usize(), 55468_usize);
        assert_eq!(a.get_ssize(), -10068_isize);
    }
    println!("--------------------------------------");
}

fn short_union_quick_start2()
{
    println!("short_union_quick_start2()");
    use cryptocol::number::SmallUInt;

    let a_shortunion = 1234_u16.into_shortunion();
    let b_shortunion = 4321_u16.into_shortunion();
    let c_shortunion = a_shortunion.wrapping_add(b_shortunion);
    println!("{} + {} = {}", a_shortunion, b_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), 5555_u16);
    for i in 0..2
        { println!("c_shortunion.get_ubyte_({}) = {}", i, c_shortunion.get_ubyte_(i)); }
    assert_eq!(c_shortunion.get_ubyte_(0), 179_u8);
    assert_eq!(c_shortunion.get_ubyte_(1), 21_u8);

    let d_shortunion = b_shortunion - a_shortunion;
    println!("{} - {} = {}", b_shortunion, a_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), 3087_u16);
    for i in 0..2
        { println!("d_shortunion.get_ubyte_({}) = {}", i, d_shortunion.get_ubyte_(i)); }
    assert_eq!(d_shortunion.get_ubyte_(0), 15_u8);
    assert_eq!(d_shortunion.get_ubyte_(1), 12_u8);

    let e_shortunion = d_shortunion * 3_u16.into_shortunion();
    println!("{} * {} = {}", d_shortunion, 3_u16.into_shortunion(), e_shortunion);
    assert_eq!(e_shortunion.get(), 9261_u16);

    let f_shortunion = c_shortunion / 10_u16.into_shortunion();
    println!("{} / {} = {}", c_shortunion, 10_u16.into_shortunion(), f_shortunion);
    assert_eq!(f_shortunion.get(), 555_u16);

    let g_shortunion = c_shortunion % 10_u16.into_shortunion();
    println!("{} % {} = {}", c_shortunion, 10_u16.into_shortunion(), g_shortunion);
    assert_eq!(g_shortunion.get(), 5_u16);
    println!("--------------------------------------");
}

fn short_union_new()
{
    println!("short_union_new()");
    use cryptocol::number::ShortUnion;    
    let a = ShortUnion::new();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u16);
    println!("--------------------------------------");
}

fn short_union_new_with()
{
    println!("short_union_new_with()");
    use cryptocol::number::ShortUnion;    
    let a = ShortUnion::new_with(1234_u16);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1234_u16);
    println!("--------------------------------------");
}

fn short_union_new_with_signed()
{
    println!("short_union_new_with_signed()");
    use cryptocol::number::ShortUnion;    
    let a = ShortUnion::new_with_signed(-1234_i16);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -1234_i16);
    println!("--------------------------------------");
}

fn short_union_new_with_ubytes()
{
    println!("short_union_new_with_ubytes()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::new_with_ubytes([172_u8, 216_u8]);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 55468_u16);
    println!("--------------------------------------");
}

fn short_union_new_with_u128()
{
    println!("short_union_new_with_u128()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::new_with_u128(55468_u128);
    let b = ShortUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 55468_u16);
    assert_eq!(b.get(), 33045_u16);
    println!("--------------------------------------");
}

fn short_union_new_with_bool()
{
    println!("short_union_new_with_bool()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::new_with_bool(true);
    let b = ShortUnion::new_with_bool(false);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 1_u16);
    assert_eq!(b.get(), 0_u16);
    println!("--------------------------------------");
}

fn short_union_zero()
{
    println!("short_union_zero()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::zero();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u16);
    println!("--------------------------------------");
}

fn short_union_one()
{
    println!("short_union_one()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::one();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1_u16);
    println!("--------------------------------------");
}

fn short_union_get()
{
    println!("short_union_get()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::new_with(55468_u16);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 55468_u16);
    println!("--------------------------------------");
}

fn short_union_get_signed()
{
    println!("short_union_get_signed()");
    use cryptocol::number::ShortUnion;    
    let a = ShortUnion::new_with(54321_u16);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -11215_i16);
    println!("--------------------------------------");
}

fn short_union_set()
{
    println!("short_union_set()");
    use cryptocol::number::ShortUnion;    
    let mut a = ShortUnion::new();
    a.set(54321_u16);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 54321_u16);
    println!("--------------------------------------");
}

fn short_union_set_signed()
{
    println!("short_union_set_signed()");
    use cryptocol::number::ShortUnion;    
    let mut a = ShortUnion::new();
    a.set_signed(-11215_i16);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -11215_i16);
    println!("--------------------------------------");
}

fn short_union_get_ushort()
{
    println!("short_union_get_ushort()");
    use cryptocol::number::ShortUnion;
    let a = ShortUnion::new_with(55468_u16);
    println!("a = {}", a.get_ushort());
    assert_eq!(a.get_ushort(), 55468_u16);
    println!("--------------------------------------");
}

fn short_union_get_sshort()
{
    println!("short_union_get_sshort()");
    use cryptocol::number::ShortUnion;    
    let a = ShortUnion::new_with(54321_u16);
    println!("a = {}", a.get_sshort());
    assert_eq!(a.get_sshort(), -11215_i16);
    println!("--------------------------------------");
}

fn short_union_set_ushort()
{
    println!("short_union_set_ushort()");
    use cryptocol::number::ShortUnion;    
    let mut a = ShortUnion::new();
    a.set_ushort(54321_u16);
    println!("a = {}", a.get_ushort());
    assert_eq!(a.get_ushort(), 54321_u16);
    println!("--------------------------------------");
}

fn short_union_set_sshort()
{
    println!("short_union_set_sshort()");
    use cryptocol::number::ShortUnion;    
    let mut a = ShortUnion::new();
    a.set_sshort(-11215_i16);
    println!("a = {}", a.get_sshort());
    assert_eq!(a.get_sshort(), -11215_i16);
    println!("--------------------------------------");
}


fn int_union_main()
{
    int_union_quick_start1();
    int_union_quick_start2();
    int_union_new();
    int_union_new_with();
    int_union_new_with_signed();
    int_union_new_with_ubytes();
    int_union_new_with_ushorts();
    int_union_new_with_u128();
    int_union_new_with_bool();
    int_union_zero();
    int_union_one();
    int_union_get();
    int_union_get_signed();
    int_union_set();
    int_union_set_signed();
    int_union_get_uint();
    int_union_get_sint();
    int_union_set_uint();
    int_union_set_sint();
}

fn int_union_quick_start1()
{
    println!("int_union_quick_start1()");
    use cryptocol::number::IntUnion;

    let a = IntUnion::new_with_signed(-454688546_i32);
    println!("a.get() = {}", a.get());
    println!("a.get_signed() = {}", a.get_signed());
    println!("a.get_uint() = {}", a.get_uint());
    println!("a.get_sint() = {}", a.get_sint());
    assert_eq!(a.get(), 3840278750_u32);
    assert_eq!(a.get_signed(), -454688546_i32);
    assert_eq!(a.get_uint(), 3840278750_u32);
    assert_eq!(a.get_sint(), -454688546_i32);

    for i in 0..2
        { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
    for i in 0..2
        { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
    for i in 0..4
        { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
    for i in 0..4
        { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
    assert_eq!(a.get_ushort_(0), 222_u16);
    assert_eq!(a.get_ushort_(1), 58598_u16);
    assert_eq!(a.get_sshort_(0), 222_i16);
    assert_eq!(a.get_sshort_(1), -6938_i16);
    assert_eq!(a.get_ubyte_(0), 222_u8);
    assert_eq!(a.get_ubyte_(1), 0_u8);
    assert_eq!(a.get_ubyte_(2), 230_u8);
    assert_eq!(a.get_ubyte_(3), 228_u8);
    assert_eq!(a.get_sbyte_(0), -34_i8);
    assert_eq!(a.get_sbyte_(1), 0_i8);
    assert_eq!(a.get_sbyte_(2), -26_i8);
    assert_eq!(a.get_sbyte_(3), -28_i8);
    #[cfg(target_pointer_width = "16")]
    {
        const N: usize = 2;
        for i in 0..N
            { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
        for i in 0..N
            { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
        assert_eq!(a.get_usize_(0), 222_usize);
        assert_eq!(a.get_usize_(1), 58598_usize);
        assert_eq!(a.get_ssize_(0), 222_isize);
        assert_eq!(a.get_ssize_(1), -6938_isize);
    }
    #[cfg(target_pointer_width = "8")]
    {
        const N: usize = 4;
        for i in 0..N
            { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
        for i in 0..N
            { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
        assert_eq!(a.get_usize_(0), 222_usize);
        assert_eq!(a.get_usize_(1), 0_usize);
        assert_eq!(a.get_usize_(2), 230_usize);
        assert_eq!(a.get_usize_(3), 228_usize);
        assert_eq!(a.get_ssize_(0), -34_isize);
        assert_eq!(a.get_ssize_(1), 0_isize);
        assert_eq!(a.get_ssize_(2), -26_isize);
        assert_eq!(a.get_ssize_(3), -28_isize);
    }
    #[cfg(target_pointer_width = "32")]
    {
        println!("a.get_usize() = {}", a.get_usize());
        println!("a.get_ssize() = {}", a.get_ssize());
        assert_eq!(a.get_usize(), 3840278750_usize);
        assert_eq!(a.get_ssize(), -454688546_isize);
    }
    println!("--------------------------------------");
}

fn int_union_quick_start2()
{
    println!("int_union_quick_start2()");
    use cryptocol::number::SmallUInt;

    let a_intunion = 12345678_u32.into_intunion();
    let b_intunion = 87654321_u32.into_intunion();
    let c_intunion = a_intunion.wrapping_add(b_intunion);
    println!("{} + {} = {}", a_intunion, b_intunion, c_intunion);
    assert_eq!(c_intunion.get(), 99999999_u32);
    for i in 0..2
        { println!("c_intunion.get_ushort_({}) = {}", i, c_intunion.get_ushort_(i)); }
    assert_eq!(c_intunion.get_ushort_(0), 57599_u16);
    assert_eq!(c_intunion.get_ushort_(1), 1525_u16);
    for i in 0..4
        { println!("c_intunion.get_ubyte_({}) = {}", i, c_intunion.get_ubyte_(i)); }
    assert_eq!(c_intunion.get_ubyte_(0), 255_u8);
    assert_eq!(c_intunion.get_ubyte_(1), 224_u8);
    assert_eq!(c_intunion.get_ubyte_(2), 245_u8);
    assert_eq!(c_intunion.get_ubyte_(3), 5_u8);

    let d_intunion = b_intunion - a_intunion;
    println!("{} - {} = {}", b_intunion, a_intunion, d_intunion);
    assert_eq!(d_intunion.get(), 75308643_u32);
    for i in 0..2
        { println!("d_shortunion.get_ushort_({}) = {}", i, d_intunion.get_ushort_(i)); }
    assert_eq!(d_intunion.get_ushort_(0), 7779_u16);
    assert_eq!(d_intunion.get_ushort_(1), 1149_u16);
    for i in 0..4
        { println!("d_shortunion.get_ubyte_({}) = {}", i, d_intunion.get_ubyte_(i)); }
    assert_eq!(d_intunion.get_ubyte_(0), 99_u8);
    assert_eq!(d_intunion.get_ubyte_(1), 30_u8);
    assert_eq!(d_intunion.get_ubyte_(2), 125_u8);
    assert_eq!(d_intunion.get_ubyte_(3), 4_u8);

    let e_intunion = d_intunion * 3_u32.into_intunion();
    println!("{} * {} = {}", d_intunion, 3_u32.into_intunion(), e_intunion);
    assert_eq!(e_intunion.get(), 225925929_u32);

    let f_intunion = c_intunion / 10_u32.into_intunion();
    println!("{} / {} = {}", c_intunion, 10_u16.into_intunion(), f_intunion);
    assert_eq!(f_intunion.get(), 9999999_u32);

    let g_intunion = c_intunion % 10_u32.into_intunion();
    println!("{} % {} = {}", c_intunion, 10_u16.into_intunion(), g_intunion);
    assert_eq!(g_intunion.get(), 9_u32);
    println!("--------------------------------------");
}

fn int_union_new()
{
    println!("int_union_new()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u32);
    println!("--------------------------------------");
}

fn int_union_new_with()
{
    println!("int_union_new_with()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new_with(1234567890_u32);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1234567890_u32);
    println!("--------------------------------------");
}

fn int_union_new_with_signed()
{
    println!("int_union_new_with_signed()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new_with_signed(-1234567890_i32);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -1234567890_i32);
    println!("--------------------------------------");
}

fn int_union_new_with_ubytes()
{
    println!("int_union_new_with_ubytes()");
    use cryptocol::number::IntUnion;
    let a = IntUnion::new_with_ubytes([222_u8, 0_u8, 230_u8, 228_u8]);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 3840278750_u32);
    println!("--------------------------------------");
}

fn int_union_new_with_ushorts()
{
    println!("int_union_new_with_ushorts()");
    use cryptocol::number::IntUnion;
    let a = IntUnion::new_with_ushorts([222_u16, 58598_u16]);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 3840278750_u32);
    println!("--------------------------------------");
}

fn int_union_new_with_u128()
{
    println!("int_union_new_with_u128()");
    use cryptocol::number::IntUnion;
    let a = IntUnion::new_with_u128(3840278750_u128);
    let b = IntUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 3840278750_u32);
    assert_eq!(b.get(), 2923004181_u32);
    println!("--------------------------------------");
}

fn int_union_new_with_bool()
{
    println!("int_union_new_with_bool()");
    use cryptocol::number::IntUnion;
    let a = IntUnion::new_with_bool(true);
    let b = IntUnion::new_with_bool(false);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 1_u32);
    assert_eq!(b.get(), 0_u32);
    println!("--------------------------------------");
}

fn int_union_zero()
{
    println!("int_union_zero()");
    use cryptocol::number::IntUnion;
    let a = IntUnion::zero();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u32);
    println!("--------------------------------------");
}

fn int_union_one()
{
    println!("int_union_one()");
    use cryptocol::number::IntUnion;
    let a = IntUnion::one();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1_u32);
    println!("--------------------------------------");
}

fn int_union_get()
{
    println!("int_union_get()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new_with(987654321_u32);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 987654321_u32);
    println!("--------------------------------------");
}

fn int_union_get_signed()
{
    println!("int_union_get_signed()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new_with(2345678901_u32);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -1949288395_i32);
    println!("--------------------------------------");
}

fn int_union_set()
{
    println!("int_union_set()");
    use cryptocol::number::IntUnion;    
    let mut a = IntUnion::new();
    a.set(987654321_u32);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 987654321_u32);
    println!("--------------------------------------");
}

fn int_union_set_signed()
{
    println!("int_union_set_signed()");
    use cryptocol::number::IntUnion;    
    let mut a = IntUnion::new();
    a.set_signed(-1949288395_i32);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -1949288395_i32);
    println!("--------------------------------------");
}

fn int_union_get_uint()
{
    println!("int_union_get_uint()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new_with(987654321_u32);
    println!("a = {}", a.get_uint());
    assert_eq!(a.get_uint(), 987654321_u32);
    println!("--------------------------------------");
}

fn int_union_get_sint()
{
    println!("int_union_get_sint()");
    use cryptocol::number::IntUnion;    
    let a = IntUnion::new_with(2345678901_u32);
    println!("a = {}", a.get_sint());
    assert_eq!(a.get_sint(), -1949288395_i32);
    println!("--------------------------------------");
}

fn int_union_set_uint()
{
    println!("int_union_set_uint()");
    use cryptocol::number::IntUnion;    
    let mut a = IntUnion::new();
    a.set_uint(987654321_u32);
    println!("a = {}", a.get_uint());
    assert_eq!(a.get_uint(), 987654321_u32);
    println!("--------------------------------------");
}

fn int_union_set_sint()
{
    println!("int_union_set_sint()");
    use cryptocol::number::IntUnion;    
    let mut a = IntUnion::new();
    a.set_sint(-1949288395_i32);
    println!("a = {}", a.get_sint());
    assert_eq!(a.get_sint(), -1949288395_i32);
    println!("--------------------------------------");
}


fn long_union_main()
{
    long_union_quick_start1();
    long_union_quick_start2();
    long_union_new();
    long_union_new_with();
    long_union_new_with_signed();
    long_union_new_with_ubytes();
    long_union_new_with_ushorts();
    long_union_new_with_uints();
    long_union_new_with_u128();
    long_union_new_with_bool();
    long_union_zero();
    long_union_one();
    long_union_get();
    long_union_set();
    long_union_get_signed();
    long_union_set_signed();
    long_union_get_ulong();
    long_union_set_ulong();
    long_union_get_slong();
    long_union_set_slong();
}

fn long_union_quick_start1()
{
    println!("long_union_quick_start1()");
    use cryptocol::number::LongUnion;

    let a = LongUnion::new_with_signed(-1234567890987645_i64);
    println!("a.get() = {}", a.get());
    println!("a.get_signed() = {}", a.get_signed());
    println!("a.get_ulong() = {}", a.get_ulong());
    println!("a.get_slong() = {}", a.get_slong());
    assert_eq!(a.get(), 18445509505818563971_u64);
    assert_eq!(a.get_signed(), -1234567890987645_i64);
    assert_eq!(a.get_ulong(), 18445509505818563971_u64);
    assert_eq!(a.get_slong(), -1234567890987645_i64);
    for i in 0..2
        { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
    for i in 0..2
        { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
    for i in 0..4
        { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
    for i in 0..4
        { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
    for i in 0..8
        { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
    for i in 0..8
        { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
    assert_eq!(a.get_uint_(0), 3278378371_u32);
    assert_eq!(a.get_uint_(1), 4294679850_u32);
    assert_eq!(a.get_sint_(0), -1016588925_i32);
    assert_eq!(a.get_sint_(1), -287446_i32);
    assert_eq!(a.get_ushort_(0), 5507_u16);
    assert_eq!(a.get_ushort_(1), 50024_u16);
    assert_eq!(a.get_ushort_(2), 40234_u16);
    assert_eq!(a.get_ushort_(3), 65531_u16);
    assert_eq!(a.get_sshort_(0), 5507_i16);
    assert_eq!(a.get_sshort_(1), -15512_i16);
    assert_eq!(a.get_sshort_(2), -25302_i16);
    assert_eq!(a.get_sshort_(3), -5_i16);
    assert_eq!(a.get_ubyte_(0), 131_u8);
    assert_eq!(a.get_ubyte_(1), 21_u8);
    assert_eq!(a.get_ubyte_(2), 104_u8);
    assert_eq!(a.get_ubyte_(3), 195_u8);
    assert_eq!(a.get_ubyte_(4), 42_u8);
    assert_eq!(a.get_ubyte_(5), 157_u8);
    assert_eq!(a.get_ubyte_(6), 251_u8);
    assert_eq!(a.get_ubyte_(7), 255_u8);
    assert_eq!(a.get_sbyte_(0), -125_i8);
    assert_eq!(a.get_sbyte_(1), 21_i8);
    assert_eq!(a.get_sbyte_(2), 104_i8);
    assert_eq!(a.get_sbyte_(3), -61_i8);
    assert_eq!(a.get_sbyte_(4), 42_i8);
    assert_eq!(a.get_sbyte_(5), -99_i8);
    assert_eq!(a.get_sbyte_(6), -5_i8);
    assert_eq!(a.get_sbyte_(7), -1_i8);

    #[cfg(target_pointer_width = "64")]
    {
        println!("a.get_usize() = {}", a.get_usize());
        println!("a.get_ssize() = {}", a.get_ssize());
        assert_eq!(a.get_usize(), 18445509505818563971_usize);
        assert_eq!(a.get_ssize(), -1234567890987645_isize);
    }
    #[cfg(target_pointer_width = "32")]
    {
        const N: usize = 2;
        for i in 0..N
            { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
        for i in 0..N
            { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
        assert_eq!(a.get_usize_(0), 3278378371_usize);
        assert_eq!(a.get_usize_(1), 4294679850_usize);
        assert_eq!(a.get_ssize_(0), -1016588925_isize);
        assert_eq!(a.get_ssize_(1), -287446_isize);
    }
    #[cfg(target_pointer_width = "16")]
    {
        const N: usize = 4;
        for i in 0..N
            { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
        for i in 0..N
            { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
            assert_eq!(a.get_usize_(0), 5507_usize);
            assert_eq!(a.get_usize_(1), 50024_usize);
            assert_eq!(a.get_usize_(2), 40234_usize);
            assert_eq!(a.get_usize_(3), 65531_usize);
            assert_eq!(a.get_ssize_(0), 5507_isize);
            assert_eq!(a.get_ssize_(1), -15512_isize);
            assert_eq!(a.get_ssize_(2), -25302_isize);
            assert_eq!(a.get_ssize_(3), -5_isize);
    }
    #[cfg(target_pointer_width = "8")]
    {
        const N: usize = 8;
        for i in 0..N
            { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
        for i in 0..N
            { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
        assert_eq!(a.get_ubyte_(0), 131_usize);
        assert_eq!(a.get_ubyte_(1), 21_usize);
        assert_eq!(a.get_ubyte_(2), 104_usize);
        assert_eq!(a.get_ubyte_(3), 195_usize);
        assert_eq!(a.get_ubyte_(4), 42_usize);
        assert_eq!(a.get_ubyte_(5), 157_usize);
        assert_eq!(a.get_ubyte_(6), 251_usize);
        assert_eq!(a.get_ubyte_(7), 255_usize);
        assert_eq!(a.get_sbyte_(0), -125_isize);
        assert_eq!(a.get_sbyte_(1), 21_isize);
        assert_eq!(a.get_sbyte_(2), 104_isize);
        assert_eq!(a.get_sbyte_(3), -61_isize);
        assert_eq!(a.get_sbyte_(4), 42_isize);
        assert_eq!(a.get_sbyte_(5), -99_isize);
        assert_eq!(a.get_sbyte_(6), -5_isize);
        assert_eq!(a.get_sbyte_(7), -1_isize);
    }
    println!("--------------------------------------");
}

fn long_union_quick_start2()
{
    println!("long_union_quick_start2()");
    use cryptocol::number::SmallUInt;

    let a_longunion = 12345678987654321_u64.into_longunion();
    let b_longunion = 87654321012345678_u64.into_longunion();
    let c_longunion = a_longunion.wrapping_add(b_longunion);
    println!("{} + {} = {}", a_longunion, b_longunion, c_longunion);
    assert_eq!(c_longunion.get(), 99999999999999999_u64);
    for i in 0..2
        { println!("c_longunion.get_uint_({}) = {}", i, c_longunion.get_uint_(i)); }
    assert_eq!(c_longunion.get_uint_(0), 1569325055_u32);
    assert_eq!(c_longunion.get_uint_(1), 23283064_u32);
    for i in 0..4
        { println!("c_longunion.get_ushort_({}) = {}", i, c_longunion.get_ushort_(i)); }
    assert_eq!(c_longunion.get_ushort_(0), 65535_u16);
    assert_eq!(c_longunion.get_ushort_(1), 23945_u16);
    assert_eq!(c_longunion.get_ushort_(2), 17784_u16);
    assert_eq!(c_longunion.get_ushort_(3), 355_u16);
    for i in 0..8
        { println!("c_longunion.get_ubyte_({}) = {}", i, c_longunion.get_ubyte_(i)); }
    assert_eq!(c_longunion.get_ubyte_(0), 255_u8);
    assert_eq!(c_longunion.get_ubyte_(1), 255_u8);
    assert_eq!(c_longunion.get_ubyte_(2), 137_u8);
    assert_eq!(c_longunion.get_ubyte_(3), 93_u8);
    assert_eq!(c_longunion.get_ubyte_(4), 120_u8);
    assert_eq!(c_longunion.get_ubyte_(5), 69_u8);
    assert_eq!(c_longunion.get_ubyte_(6), 99_u8);
    assert_eq!(c_longunion.get_ubyte_(7), 1_u8);

    let d_longunion = b_longunion - a_longunion;
    println!("{} - {} = {}", b_longunion, a_longunion, d_longunion);
    assert_eq!(d_longunion.get(), 75308642024691357_u64);
    for i in 0..2
        { println!("d_longunion.get_uint_({}) = {}", i, d_longunion.get_uint_(i)); }
    assert_eq!(d_longunion.get_uint_(0), 2556827293_u32);
    assert_eq!(d_longunion.get_uint_(1), 17534159_u32);
    for i in 0..4
        { println!("d_longunion.get_ushort_({}) = {}", i, d_longunion.get_ushort_(i)); }
    assert_eq!(d_longunion.get_ushort_(0), 5789_u16);
    assert_eq!(d_longunion.get_ushort_(1), 39014_u16);
    assert_eq!(d_longunion.get_ushort_(2), 36047_u16);
    assert_eq!(d_longunion.get_ushort_(3), 267_u16);
    for i in 0..8
        { println!("d_longunion.get_ubyte_({}) = {}", i, d_longunion.get_ubyte_(i)); }
    assert_eq!(d_longunion.get_ubyte_(0), 157_u8);
    assert_eq!(d_longunion.get_ubyte_(1), 22_u8);
    assert_eq!(d_longunion.get_ubyte_(2), 102_u8);
    assert_eq!(d_longunion.get_ubyte_(3), 152_u8);
    assert_eq!(d_longunion.get_ubyte_(4), 207_u8);
    assert_eq!(d_longunion.get_ubyte_(5), 140_u8);
    assert_eq!(d_longunion.get_ubyte_(6), 11_u8);
    assert_eq!(d_longunion.get_ubyte_(7), 1_u8);

    let e_longunion = d_longunion * 3_u64.into_longunion();
    println!("{} * {} = {}", d_longunion, 3_u64.into_longunion(), e_longunion);
    assert_eq!(e_longunion.get(), 225925926074074071_u64);

    let f_longunion = c_longunion / 10_u64.into_longunion();
    println!("{} / {} = {}", c_longunion, 10_u64.into_longunion(), f_longunion);
    assert_eq!(f_longunion.get(), 9999999999999999_u64);

    let g_longunion = c_longunion % 10_u64.into_longunion();
    println!("{} % {} = {}", c_longunion, 10_u64.into_longunion(), g_longunion);
    assert_eq!(g_longunion.get(), 9_u64);
    println!("--------------------------------------");
}

fn long_union_new()
{
    println!("long_union_new()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u64);
    println!("--------------------------------------");
}

fn long_union_new_with()
{
    println!("long_union_new_with()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new_with(12345678909876456_u64);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 12345678909876456_u64);
    println!("--------------------------------------");
}

fn long_union_new_with_signed()
{
    println!("long_union_new_with_signed()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new_with_signed(-12345678909876456_i64);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -12345678909876456_i64);
    println!("--------------------------------------");
}

fn long_union_new_with_ubytes()
{
    println!("long_union_new_with_ubytes()");
    use cryptocol::number::LongUnion;
    let arr = [131_u8, 21_u8, 104_u8, 195_u8, 42_u8, 157_u8, 251_u8, 255_u8];
    let a = LongUnion::new_with_ubytes(arr);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 18445509505818563971_u64);
    println!("--------------------------------------");
}

fn long_union_new_with_ushorts()
{
    println!("long_union_new_with_ushorts()");
    use cryptocol::number::LongUnion;
    let a = LongUnion::new_with_ushorts([5507_u16, 50024_u16, 40234_u16, 65531_u16]);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 18445509505818563971_u64);
    println!("--------------------------------------");
}

fn long_union_new_with_uints()
{
    println!("long_union_new_with_uints()");
    use cryptocol::number::LongUnion;
    let a = LongUnion::new_with_uints([3278378371_u32, 4294679850_u32]);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 18445509505818563971_u64);
    println!("--------------------------------------");
}

fn long_union_new_with_u128()
{
    println!("long_union_new_with_u128()");
    use cryptocol::number::LongUnion;
    let a = LongUnion::new_with_u128(18445509505818563971_u128);
    let b = LongUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 18445509505818563971_u64);
    assert_eq!(b.get(), 12312739301371248917_u64);
    println!("--------------------------------------");
}

fn long_union_new_with_bool()
{
    println!("long_union_new_with_bool()");
    use cryptocol::number::LongUnion;
    let a = LongUnion::new_with_bool(true);
    let b = LongUnion::new_with_bool(false);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 1_u64);
    assert_eq!(b.get(), 0_u64);
    println!("--------------------------------------");
}

fn long_union_zero()
{
    println!("long_union_zero()");
    use cryptocol::number::LongUnion;
    let a = LongUnion::zero();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u64);
    println!("--------------------------------------");
}

fn long_union_one()
{
    println!("long_union_one()");
    use cryptocol::number::LongUnion;
    let a = LongUnion::one();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1_u64);
    println!("--------------------------------------");
}

fn long_union_get()
{
    println!("long_union_get()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new_with(654321987654321_u64);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 654321987654321_u64);
    println!("--------------------------------------");
}

fn long_union_set()
{
    println!("long_union_set()");
    use cryptocol::number::LongUnion;    
    let mut a = LongUnion::new();
    a.set(654321987654321_u64);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 654321987654321_u64);
    println!("--------------------------------------");
}

fn long_union_get_signed()
{
    println!("long_union_get_signed()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new_with(12345678909876456789_u64);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -6101065163833094827_i64);
    println!("--------------------------------------");
}

fn long_union_set_signed()
{
    println!("long_union_set_signed()");
    use cryptocol::number::LongUnion;    
    let mut a = LongUnion::new();
    a.set_signed(-6101065163833094827_i64);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -6101065163833094827_i64);
    println!("--------------------------------------");
}

fn long_union_get_ulong()
{
    println!("long_union_get_ulong()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new_with(654321987654321_u64);
    println!("a = {}", a.get_ulong());
    assert_eq!(a.get_ulong(), 654321987654321_u64);
    println!("--------------------------------------");
}

fn long_union_set_ulong()
{
    println!("long_union_set_ulong()");
    use cryptocol::number::LongUnion;    
    let mut a = LongUnion::new();
    a.set_ulong(654321987654321_u64);
    println!("a = {}", a.get_ulong());
    assert_eq!(a.get_ulong(), 654321987654321_u64);
    println!("--------------------------------------");
}

fn long_union_get_slong()
{
    println!("long_union_get_slong()");
    use cryptocol::number::LongUnion;    
    let a = LongUnion::new_with(12345678909876456789_u64);
    println!("a = {}", a.get_slong());
    assert_eq!(a.get_slong(), -6101065163833094827_i64);
    println!("--------------------------------------");
}

fn long_union_set_slong()
{
    println!("long_union_set_slong()");
    use cryptocol::number::LongUnion;    
    let mut a = LongUnion::new();
    a.set_signed(-6101065163833094827_i64);
    println!("a = {}", a.get_slong());
    assert_eq!(a.get_slong(), -6101065163833094827_i64);
    println!("--------------------------------------");
}


fn longer_union_main()
{
    longer_union_quick_start1();
    longer_union_quick_start2();
    longer_union_new();
    longer_union_new_with();
    longer_union_new_with_signed();
    longer_union_new_with_ubytes();
    longer_union_new_with_ushorts();
    longer_union_new_with_uints();
    longer_union_new_with_ulongs();
    longer_union_new_with_u128();
    longer_union_new_with_bool();
    longer_union_zero();
    longer_union_one();
    longer_union_get();
    longer_union_set();
    longer_union_get_signed();
    longer_union_set_signed();
    longer_union_get_ulonger();
    longer_union_set_ulonger();
    longer_union_get_slonger();
    longer_union_set_slonger();
}

fn longer_union_quick_start1()
{
    println!("longer_union_quick_start1()");
    use cryptocol::number::LongerUnion;

    let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);

    println!("a.get() = {}", a.get());
    println!("a.get_signed() = {}", a.get_signed());
    println!("a.get_ulonger() = {}", a.get_ulonger());
    println!("a.get_slonger() = {}", a.get_slonger());
    assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_i128);
    assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
    assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);

    #[cfg(target_endian = "little")]
    {
        for i in 0..2
            { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
        for i in 0..2
            { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
        for i in 0..4
            { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
        for i in 0..4
            { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
        for i in 0..8
            { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
        for i in 0..8
            { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
        for i in 0..16
            { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
        for i in 0..16
            { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
        assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
        assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
        assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
        assert_eq!(a.get_slong_(1), -66926059474483112_i64);
        assert_eq!(a.get_uint_(0), 4048161615_u32);
        assert_eq!(a.get_uint_(1), 3181603061_u32);
        assert_eq!(a.get_uint_(2), 2127464536_u32);
        assert_eq!(a.get_uint_(3), 4279384858_u32);
        assert_eq!(a.get_sint_(0), -246805681_i32);
        assert_eq!(a.get_sint_(1), -1113364235_i32);
        assert_eq!(a.get_sint_(2), 2127464536_i32);
        assert_eq!(a.get_sint_(3), -15582438_i32);
        assert_eq!(a.get_ushort_(0), 2895_u16);
        assert_eq!(a.get_ushort_(1), 61770_u16);
        assert_eq!(a.get_ushort_(2), 26869_u16);
        assert_eq!(a.get_ushort_(3), 48547_u16);
        assert_eq!(a.get_ushort_(4), 34904_u16);
        assert_eq!(a.get_ushort_(5), 32462_u16);
        assert_eq!(a.get_ushort_(6), 15130_u16);
        assert_eq!(a.get_ushort_(7), 65298_u16);
        assert_eq!(a.get_sshort_(0), 2895_i16);
        assert_eq!(a.get_sshort_(1), -3766_i16);
        assert_eq!(a.get_sshort_(2), 26869_i16);
        assert_eq!(a.get_sshort_(3), -16989_i16);
        assert_eq!(a.get_sshort_(4), -30632_i16);
        assert_eq!(a.get_sshort_(5), 32462_i16);
        assert_eq!(a.get_sshort_(6), 15130_i16);
        assert_eq!(a.get_sshort_(7), -238_i16);
        assert_eq!(a.get_ubyte_(0), 79_u8);
        assert_eq!(a.get_ubyte_(1), 11_u8);
        assert_eq!(a.get_ubyte_(2), 74_u8);
        assert_eq!(a.get_ubyte_(3), 241_u8);
        assert_eq!(a.get_ubyte_(4), 245_u8);
        assert_eq!(a.get_ubyte_(5), 104_u8);
        assert_eq!(a.get_ubyte_(6), 163_u8);
        assert_eq!(a.get_ubyte_(7), 189_u8);
        assert_eq!(a.get_ubyte_(8), 88_u8);
        assert_eq!(a.get_ubyte_(9), 136_u8);
        assert_eq!(a.get_ubyte_(10), 206_u8);
        assert_eq!(a.get_ubyte_(11), 126_u8);
        assert_eq!(a.get_ubyte_(12), 26_u8);
        assert_eq!(a.get_ubyte_(13), 59_u8);
        assert_eq!(a.get_ubyte_(14), 18_u8);
        assert_eq!(a.get_ubyte_(15), 255_u8);
        assert_eq!(a.get_sbyte_(0), 79_i8);
        assert_eq!(a.get_sbyte_(1), 11_i8);
        assert_eq!(a.get_sbyte_(2), 74_i8);
        assert_eq!(a.get_sbyte_(3), -15_i8);
        assert_eq!(a.get_sbyte_(4), -11_i8);
        assert_eq!(a.get_sbyte_(5), 104_i8);
        assert_eq!(a.get_sbyte_(6), -93_i8);
        assert_eq!(a.get_sbyte_(7), -67_i8);
        assert_eq!(a.get_sbyte_(8), 88_i8);
        assert_eq!(a.get_sbyte_(9), -120_i8);
        assert_eq!(a.get_sbyte_(10), -50_i8);
        assert_eq!(a.get_sbyte_(11), 126_i8);
        assert_eq!(a.get_sbyte_(12), 26_i8);
        assert_eq!(a.get_sbyte_(13), 59_i8);
        assert_eq!(a.get_sbyte_(14), 18_i8);
        assert_eq!(a.get_sbyte_(15), -1_i8);

        #[cfg(target_pointer_width = "128")]
        {
            println!("a.get_usize() = {}", a.get_usize());
            println!("a.get_ssize() = {}", a.get_ssize());
            assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
            assert_eq!(a.get_ssize(), 1234567890987654321012345678987654321_isize);
        }
        #[cfg(target_pointer_width = "64")]
        {
            const N: usize = 2;
            for i in 0..N
                { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
            assert_eq!(a.get_usize_(0), 13664881099896654671_usize);
            assert_eq!(a.get_usize_(1), 18379818014235068504_usize);
            assert_eq!(a.get_ssize_(0), -4781862973812896945_isize);
            assert_eq!(a.get_ssize_(1), -66926059474483112_isize);
        }
        #[cfg(target_pointer_width = "32")]
        {
            const N: usize = 4;
            for i in 0..N
                { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
            assert_eq!(a.get_usize_(0), 4048161615_usize);
            assert_eq!(a.get_usize_(1), 3181603061_usize);
            assert_eq!(a.get_usize_(2), 2127464536_usize);
            assert_eq!(a.get_usize_(3), 4279384858_usize);
            assert_eq!(a.get_ssize_(0), -246805681_isize);
            assert_eq!(a.get_ssize_(1), -1113364235_isize);
            assert_eq!(a.get_ssize_(2), 2127464536_isize);
            assert_eq!(a.get_ssize_(3), -15582438_isize);
        }
        #[cfg(target_pointer_width = "16")]
        {
            const N: usize = 8;
            for i in 0..N
                { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
            assert_eq!(a.get_usize_(0), 2895_usize);
            assert_eq!(a.get_usize_(1), 61770_usize);
            assert_eq!(a.get_usize_(2), 26869_usize);
            assert_eq!(a.get_usize_(3), 48547_usize);
            assert_eq!(a.get_usize_(4), 34904_usize);
            assert_eq!(a.get_usize_(5), 32462_usize);
            assert_eq!(a.get_usize_(6), 15130_usize);
            assert_eq!(a.get_usize_(7), 65298_usize);
            assert_eq!(a.get_ssize_(0), 2895_isize);
            assert_eq!(a.get_ssize_(1), -3766_isize);
            assert_eq!(a.get_ssize_(2), 26869_isize);
            assert_eq!(a.get_ssize_(3), -16989_isize);
            assert_eq!(a.get_ssize_(4), -30632_isize);
            assert_eq!(a.get_ssize_(5), 32462_isize);
            assert_eq!(a.get_ssize_(6), 15130_isize);
            assert_eq!(a.get_ssize_(7), -238_isize);
        }
        #[cfg(target_pointer_width = "8")]
        {
            const N: usize = 16;
            for i in 0..N
                { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
            assert_eq!(a.get_usize_(0), 79_usize);
            assert_eq!(a.get_usize_(1), 11_usize);
            assert_eq!(a.get_usize_(2), 74_usize);
            assert_eq!(a.get_usize_(3), 241_usize);
            assert_eq!(a.get_usize_(4), 245_usize);
            assert_eq!(a.get_usize_(5), 104_usize);
            assert_eq!(a.get_usize_(6), 163_usize);
            assert_eq!(a.get_usize_(7), 189_usize);
            assert_eq!(a.get_usize_(8), 88_usize);
            assert_eq!(a.get_usize_(9), 136_usize);
            assert_eq!(a.get_usize_(10), 206_usize);
            assert_eq!(a.get_usize_(11), 126_usize);
            assert_eq!(a.get_usize_(12), 26_usize);
            assert_eq!(a.get_usize_(13), 59_usize);
            assert_eq!(a.get_usize_(14), 18_usize);
            assert_eq!(a.get_usize_(15), 255_usize);
            assert_eq!(a.get_ssize_(0), 79_isize);
            assert_eq!(a.get_ssize_(1), 11_isize);
            assert_eq!(a.get_ssize_(2), 74_isize);
            assert_eq!(a.get_ssize_(3), -15_isize);
            assert_eq!(a.get_ssize_(4), -11_isize);
            assert_eq!(a.get_ssize_(5), 104_isize);
            assert_eq!(a.get_ssize_(6), -93_isize);
            assert_eq!(a.get_ssize_(7), -67_isize);
            assert_eq!(a.get_ssize_(8), 88_isize);
            assert_eq!(a.get_ssize_(9), -120_isize);
            assert_eq!(a.get_ssize_(10), -50_isize);
            assert_eq!(a.get_ssize_(11), 126_isize);
            assert_eq!(a.get_ssize_(12), 26_isize);
            assert_eq!(a.get_ssize_(13), 59_isize);
            assert_eq!(a.get_ssize_(14), 18_isize);
            assert_eq!(a.get_ssize_(15), -1_isize);
        }
    }
    println!("--------------------------------------");
}

fn longer_union_quick_start2()
{
    println!("longer_union_quick_start2()");
    use cryptocol::number::SmallUInt;

    let a_longerunion = 123456789876543212345678987654321_u128.into_longerunion();
    let b_longerunion = 876543210123456787654321012345678_u128.into_longerunion();
    let c_longerunion = a_longerunion.wrapping_add(b_longerunion);
    println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), 999999999999999999999999999999999_u128);
    for i in 0..2
        { println!("c_longerunion.get_ulong_({}) = {}", i, c_longerunion.get_ulong_(i)); }
    assert_eq!(c_longerunion.get_ulong_(0), 4089650035136921599_u64);
    assert_eq!(c_longerunion.get_ulong_(1), 54210108624275_u64);
    for i in 0..4
        { println!("c_longerunion.get_uint_({}) = {}", i, c_longerunion.get_uint_(i)); }
    assert_eq!(c_longerunion.get_uint_(0), 4294967295_u32);
    assert_eq!(c_longerunion.get_uint_(1), 952195849_u32);
    assert_eq!(c_longerunion.get_uint_(2), 3326381459_u32);
    assert_eq!(c_longerunion.get_uint_(3), 12621_u32);
    for i in 0..8
        { println!("c_longerunion.get_ushort_({}) = {}", i, c_longerunion.get_ushort_(i)); }
    assert_eq!(c_longerunion.get_ushort_(0), 65535_u16);
    assert_eq!(c_longerunion.get_ushort_(1), 65535_u16);
    assert_eq!(c_longerunion.get_ushort_(2), 23305_u16);
    assert_eq!(c_longerunion.get_ushort_(3), 14529_u16);
    assert_eq!(c_longerunion.get_ushort_(4), 36243_u16);
    assert_eq!(c_longerunion.get_ushort_(5), 50756_u16);
    assert_eq!(c_longerunion.get_ushort_(6), 12621_u16);
    assert_eq!(c_longerunion.get_ushort_(7), 0_u16);
    for i in 0..16
        { println!("c_longerunion.get_ubyte_({}) = {}", i, c_longerunion.get_ubyte_(i)); }
    assert_eq!(c_longerunion.get_ubyte_(0), 255_u8);
    assert_eq!(c_longerunion.get_ubyte_(1), 255_u8);
    assert_eq!(c_longerunion.get_ubyte_(2), 255_u8);
    assert_eq!(c_longerunion.get_ubyte_(3), 255_u8);
    assert_eq!(c_longerunion.get_ubyte_(4), 9_u8);
    assert_eq!(c_longerunion.get_ubyte_(5), 91_u8);
    assert_eq!(c_longerunion.get_ubyte_(6), 193_u8);
    assert_eq!(c_longerunion.get_ubyte_(7), 56_u8);
    assert_eq!(c_longerunion.get_ubyte_(8), 147_u8);
    assert_eq!(c_longerunion.get_ubyte_(9), 141_u8);
    assert_eq!(c_longerunion.get_ubyte_(10), 68_u8);
    assert_eq!(c_longerunion.get_ubyte_(11), 198_u8);
    assert_eq!(c_longerunion.get_ubyte_(12), 77_u8);
    assert_eq!(c_longerunion.get_ubyte_(13), 49_u8);
    assert_eq!(c_longerunion.get_ubyte_(14), 0_u8);
    assert_eq!(c_longerunion.get_ubyte_(15), 0_u8);

    let d_longerunion = b_longerunion - a_longerunion;
    println!("{} - {} = {}", b_longerunion, a_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), 753086420246913575308642024691357_u128);
    for i in 0..2
        { println!("d_longunion.get_ulong_({}) = {}", i, d_longerunion.get_ulong_(i)); }
    assert_eq!(d_longerunion.get_ulong_(0), 14084888390109238941_u64);
    assert_eq!(d_longerunion.get_ulong_(1), 40824896645051_u64);
    for i in 0..4
        { println!("d_longunion.get_uint_({}) = {}", i, d_longerunion.get_uint_(i)); }
    assert_eq!(d_longerunion.get_uint_(0), 2843481757_u32);
    assert_eq!(d_longerunion.get_uint_(1), 3279393629_u32);
    assert_eq!(d_longerunion.get_uint_(2), 1232496571_u32);
    assert_eq!(d_longerunion.get_uint_(3), 9505_u32);
    for i in 0..8
        { println!("d_longunion.get_ushort_({}) = {}", i, d_longerunion.get_ushort_(i)); }
    assert_eq!(d_longerunion.get_ushort_(0), 5789_u16);
    assert_eq!(d_longerunion.get_ushort_(1), 43388_u16);
    assert_eq!(d_longerunion.get_ushort_(2), 37725_u16);
    assert_eq!(d_longerunion.get_ushort_(3), 50039_u16);
    assert_eq!(d_longerunion.get_ushort_(4), 26555_u16);
    assert_eq!(d_longerunion.get_ushort_(5), 18806_u16);
    assert_eq!(d_longerunion.get_ushort_(6), 9505_u16);
    assert_eq!(d_longerunion.get_ushort_(7), 0_u16);
    for i in 0..16
        { println!("d_longunion.get_ubyte_({}) = {}", i, d_longerunion.get_ubyte_(i)); }
    assert_eq!(d_longerunion.get_ubyte_(0), 157_u8);
    assert_eq!(d_longerunion.get_ubyte_(1), 22_u8);
    assert_eq!(d_longerunion.get_ubyte_(2), 124_u8);
    assert_eq!(d_longerunion.get_ubyte_(3), 169_u8);
    assert_eq!(d_longerunion.get_ubyte_(4), 93_u8);
    assert_eq!(d_longerunion.get_ubyte_(5), 147_u8);
    assert_eq!(d_longerunion.get_ubyte_(6), 119_u8);
    assert_eq!(d_longerunion.get_ubyte_(7), 195_u8);
    assert_eq!(d_longerunion.get_ubyte_(8), 187_u8);
    assert_eq!(d_longerunion.get_ubyte_(9), 103_u8);
    assert_eq!(d_longerunion.get_ubyte_(10), 118_u8);
    assert_eq!(d_longerunion.get_ubyte_(11), 73_u8);
    assert_eq!(d_longerunion.get_ubyte_(12), 33_u8);
    assert_eq!(d_longerunion.get_ubyte_(13), 37_u8);
    assert_eq!(d_longerunion.get_ubyte_(14), 0_u8);
    assert_eq!(d_longerunion.get_ubyte_(15), 0_u8);

    let e_longerunion = d_longerunion * 3_u128.into_longerunion();
    println!("{} * {} = {}", d_longerunion, 3_u128.into_longerunion(), e_longerunion);
    assert_eq!(e_longerunion.get(), 2259259260740740725925926074074071_u128);

    let f_longerunion = c_longerunion / 10_u128.into_longerunion();
    println!("{} / {} = {}", c_longerunion, 10_u128.into_longerunion(), f_longerunion);
    assert_eq!(f_longerunion.get(), 99999999999999999999999999999999_u128);

    let g_longerunion = c_longerunion % 10_u128.into_longerunion();
    println!("{} % {} = {}", c_longerunion, 10_u128.into_longerunion(), g_longerunion);
    assert_eq!(g_longerunion.get(), 9_u128);
    println!("--------------------------------------");
}

fn longer_union_new()
{
    println!("longer_union_new()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with()
{
    println!("longer_union_new_with()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new_with(1234567890987654321012345678987654321_u128);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1234567890987654321012345678987654321_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with_signed()
{
    println!("longer_union_new_with_signed()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_i128);
    println!("--------------------------------------");
}

fn longer_union_new_with_ubytes()
{
    println!("longer_union_new_with_ubytes()");
    use cryptocol::number::LongerUnion;
    let arr = [79_u8, 11_u8, 74_u8, 241_u8, 245_u8, 104_u8, 163_u8, 189_u8, 88_u8, 136_u8, 206_u8, 126_u8, 26_u8, 59_u8, 18_u8, 255_u8];
    let a = LongerUnion::new_with_ubytes(arr);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with_ushorts()
{
    println!("longer_union_new_with_ushorts()");
    use cryptocol::number::LongerUnion;
    let arr = [2895_u16, 61770_u16, 26869_u16, 48547_u16, 34904_u16, 32462_u16, 15130_u16, 65298_u16];
    let a = LongerUnion::new_with_ushorts(arr);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with_uints()
{
    println!("longer_union_new_with_uints()");
    use cryptocol::number::LongerUnion;
    let arr = [4048161615_u32, 3181603061_u32, 2127464536_u32, 4279384858_u32];
    let a = LongerUnion::new_with_uints(arr);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with_ulongs()
{
    println!("longer_union_new_with_ulongs()");
    use cryptocol::number::LongerUnion;
    let a = LongerUnion::new_with_ulongs([13664881099896654671_u64, 18379818014235068504_u64]);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with_u128()
{
    println!("longer_union_new_with_u128()");
    use cryptocol::number::LongerUnion;
    let a = LongerUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 123456789012345678901234567890123456789_u128);
    println!("--------------------------------------");
}

fn longer_union_new_with_bool()
{
    println!("longer_union_new_with_bool()");
    use cryptocol::number::LongerUnion;
    let a = LongerUnion::new_with_bool(true);
    let b = LongerUnion::new_with_bool(false);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 1_u128);
    assert_eq!(b.get(), 0_u128);
    println!("--------------------------------------");
}

fn longer_union_zero()
{
    println!("longer_union_zero()");
    use cryptocol::number::LongerUnion;
    let a = LongerUnion::zero();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_u128);
    println!("--------------------------------------");
}

fn longer_union_one()
{
    println!("longer_union_one()");
    use cryptocol::number::LongerUnion;
    let a = LongerUnion::one();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1_u128);
    println!("--------------------------------------");
}

fn longer_union_get()
{
    println!("longer_union_get()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new_with(98765432101234567898765432101234546789_u128);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 98765432101234567898765432101234546789_u128);
    println!("--------------------------------------");
}

fn longer_union_set()
{
    println!("longer_union_set()");
    use cryptocol::number::LongerUnion;    
    let mut a = LongerUnion::new();
    a.set(98765432101234567898765432101234546789_u128);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 98765432101234567898765432101234546789_u128);
    println!("--------------------------------------");
}

fn longer_union_get_signed()
{
    println!("longer_union_get_signed()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new_with(234567890987654321012345678987654321234_u128);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -105714475933284142451028928444113890222_i128);
    println!("--------------------------------------");
}

fn longer_union_set_signed()
{
    println!("longer_union_set()");
    use cryptocol::number::LongerUnion;    
    let mut a = LongerUnion::new();
    a.set_signed(-105714475933284142451028928444113890222_i128);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -105714475933284142451028928444113890222_i128);
    println!("--------------------------------------");
}

fn longer_union_get_ulonger()
{
    println!("longer_union_get_ulonger()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new_with(98765432101234567898765432101234546789_u128);
    println!("a = {}", a.get_ulonger());
    assert_eq!(a.get_ulonger(), 98765432101234567898765432101234546789_u128);
    println!("--------------------------------------");
}

fn longer_union_set_ulonger()
{
    println!("longer_union_set_ulonger()");
    use cryptocol::number::LongerUnion;    
    let mut a = LongerUnion::new();
    a.set_ulonger(98765432101234567898765432101234546789_u128);
    println!("a = {}", a.get_ulonger());
    assert_eq!(a.get_ulonger(), 98765432101234567898765432101234546789_u128);
    println!("--------------------------------------");
}

fn longer_union_get_slonger()
{
    println!("longer_union_get_slonger()");
    use cryptocol::number::LongerUnion;    
    let a = LongerUnion::new_with(234567890987654321012345678987654321234_u128);
    println!("a = {}", a.get_slonger());
    assert_eq!(a.get_slonger(), -105714475933284142451028928444113890222_i128);
    println!("--------------------------------------");
}

fn longer_union_set_slonger()
{
    println!("longer_union_set_slong()");
    use cryptocol::number::LongerUnion;    
    let mut a = LongerUnion::new();
    a.set_slonger(-105714475933284142451028928444113890222_i128);
    println!("a = {}", a.get_slonger());
    assert_eq!(a.get_slonger(), -105714475933284142451028928444113890222_i128);
    println!("--------------------------------------");
}


fn size_union_main()
{
    size_union_quick_start1();
    size_union_quick_start2();
    size_union_new();
    size_union_new_with();
    size_union_new_with_signed();
    size_union_new_with_bytes();
    size_union_new_with_shorts();
    size_union_new_with_ints();
    size_union_new_with_longs();
    size_union_new_with_u128();
    size_union_new_with_bool();
    size_union_zero();
    size_union_one();
    size_union_get();
    size_union_set();
    size_union_get_signed();
    size_union_set_signed();
    size_union_get_usize();
    size_union_set_usize();
    size_union_get_ssize();
    size_union_set_ssize();
}

fn size_union_quick_start1()
{
    println!("size_union_quick_start1()");
    use cryptocol::number::SizeUnion;

    #[cfg(target_pointer_width = "128")]    let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
    #[cfg(target_pointer_width = "64")]     let a = SizeUnion::new_with_signed(-4781862973812896945_isize);
    #[cfg(target_pointer_width = "32")]     let a = SizeUnion::new_with_signed(-246805681_isize);
    #[cfg(target_pointer_width = "16")]     let a = SizeUnion::new_with_signed(2895_isize);
    #[cfg(target_pointer_width = "8")]      let a = SizeUnion::new_with_signed(79_isize);
    println!("a.get() = {}", a.get());
    println!("a.get_signed() = {}", a.get_signed());
    println!("a.get_usize() = {}", a.get_usize());
    println!("a.get_ssize() = {}", a.get_ssize());
    #[cfg(target_pointer_width = "128")]    println!("a.get_ulonger() = {}", a.get_ulonger());
    #[cfg(target_pointer_width = "128")]    println!("a.get_slonger() = {}", a.get_slonger());
    #[cfg(target_pointer_width = "64")]     println!("a.get_ulong() = {}", a.get_ulong());
    #[cfg(target_pointer_width = "64")]     println!("a.get_slong() = {}", a.get_slong());
    #[cfg(target_pointer_width = "32")]     println!("a.get_uint() = {}", a.get_uint());
    #[cfg(target_pointer_width = "32")]     println!("a.get_sint() = {}", a.get_sint());
    #[cfg(target_pointer_width = "16")]     println!("a.get_ushort() = {}", a.get_ushort());
    #[cfg(target_pointer_width = "16")]     println!("a.get_sshort() = {}", a.get_sshort());
    #[cfg(target_pointer_width = "128")]
    {
        assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
        assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_isize);
        assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
        assert_eq!(a.get_ssize(), -1234567890987654321012345678987654321_isize);
    }
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(a.get(), 13664881099896654671_usize);
        assert_eq!(a.get_signed(), -4781862973812896945_isize);
        assert_eq!(a.get_usize(), 13664881099896654671_usize);
        assert_eq!(a.get_ssize(), -4781862973812896945_isize);
    }
    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(a.get(), 4048161615_usize);
        assert_eq!(a.get_signed(), -246805681_isize);
        assert_eq!(a.get_usize(), 4048161615_usize);
        assert_eq!(a.get_ssize(), -246805681_isize);
    }
    #[cfg(target_pointer_width = "16")]
    {
        assert_eq!(a.get(), 2895_usize);
        assert_eq!(a.get_signed(), 2895_isize);
        assert_eq!(a.get_usize(), 2895_usize);
        assert_eq!(a.get_ssize(), 2895_isize);
    }
    #[cfg(target_pointer_width = "8")]
    {
        assert_eq!(a.get(), 79_usize);
        assert_eq!(a.get_signed(), 79_isize);
        assert_eq!(a.get_usize(), 79_usize);
        assert_eq!(a.get_ssize(), 79_isize);
    }

    #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
    #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
    #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_ulong(), 13664881099896654671_u64);
    #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_slong(), -4781862973812896945_i64);
    #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_uint(), 4048161615_u32);
    #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_sint(), -246805681_i32);
    #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_ushort(), 2895_u16);
    #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_sshort(), 2895_i16);
    #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_ubyte(), 79_u8);
    #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_sbyte(), 79_u8);

    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..2
            { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
        for i in 0..2
            { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
        for i in 0..4
            { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
        for i in 0..4
            { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
        for i in 0..8
            { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
        for i in 0..8
            { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
        for i in 0..16
            { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
        for i in 0..16
            { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
    }
    #[cfg(target_pointer_width = "64")]
    {
        for i in 0..2
            { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
        for i in 0..2
            { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
        for i in 0..4
            { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
        for i in 0..4
            { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
        for i in 0..8
            { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
        for i in 0..8
            { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
        }
    #[cfg(target_pointer_width = "32")]
    {
        for i in 0..2
            { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
        for i in 0..2
            { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
        for i in 0..4
            { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
        for i in 0..4
            { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
    }
    #[cfg(target_pointer_width = "16")]
    {
        for i in 0..2
            { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
        for i in 0..2
            { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
    }
    #[cfg(target_pointer_width = "128")]
    {
        assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
        assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
        assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
        assert_eq!(a.get_slong_(1), -66926059474483112_i64);
        assert_eq!(a.get_uint_(0), 4048161615_u32);
        assert_eq!(a.get_uint_(1), 3181603061_u32);
        assert_eq!(a.get_uint_(2), 2127464536_u32);
        assert_eq!(a.get_uint_(3), 4279384858_u32);
        assert_eq!(a.get_sint_(0), -246805681_i32);
        assert_eq!(a.get_sint_(1), -1113364235_i32);
        assert_eq!(a.get_sint_(2), 2127464536_i32);
        assert_eq!(a.get_sint_(3), -15582438_i32);
        assert_eq!(a.get_ushort_(0), 2895_u16);
        assert_eq!(a.get_ushort_(1), 61770_u16);
        assert_eq!(a.get_ushort_(2), 26869_u16);
        assert_eq!(a.get_ushort_(3), 48547_u16);
        assert_eq!(a.get_ushort_(4), 34904_u16);
        assert_eq!(a.get_ushort_(5), 32462_u16);
        assert_eq!(a.get_ushort_(6), 15130_u16);
        assert_eq!(a.get_ushort_(7), 65298_u16);
        assert_eq!(a.get_sshort_(0), 2895_i16);
        assert_eq!(a.get_sshort_(1), -3766_i16);
        assert_eq!(a.get_sshort_(2), 26869_i16);
        assert_eq!(a.get_sshort_(3), -16989_i16);
        assert_eq!(a.get_sshort_(4), -30632_i16);
        assert_eq!(a.get_sshort_(5), 32462_i16);
        assert_eq!(a.get_sshort_(6), 15130_i16);
        assert_eq!(a.get_sshort_(7), -238_i16);
        assert_eq!(a.get_ubyte_(0), 79_u8);
        assert_eq!(a.get_ubyte_(1), 11_u8);
        assert_eq!(a.get_ubyte_(2), 74_u8);
        assert_eq!(a.get_ubyte_(3), 241_u8);
        assert_eq!(a.get_ubyte_(4), 245_u8);
        assert_eq!(a.get_ubyte_(5), 104_u8);
        assert_eq!(a.get_ubyte_(6), 163_u8);
        assert_eq!(a.get_ubyte_(7), 189_u8);
        assert_eq!(a.get_ubyte_(8), 88_u8);
        assert_eq!(a.get_ubyte_(9), 136_u8);
        assert_eq!(a.get_ubyte_(10), 206_u8);
        assert_eq!(a.get_ubyte_(11), 126_u8);
        assert_eq!(a.get_ubyte_(12), 26_u8);
        assert_eq!(a.get_ubyte_(13), 59_u8);
        assert_eq!(a.get_ubyte_(14), 18_u8);
        assert_eq!(a.get_ubyte_(15), 255_u8);
        assert_eq!(a.get_sbyte_(0), 79_i8);
        assert_eq!(a.get_sbyte_(1), 11_i8);
        assert_eq!(a.get_sbyte_(2), 74_i8);
        assert_eq!(a.get_sbyte_(3), -15_i8);
        assert_eq!(a.get_sbyte_(4), -11_i8);
        assert_eq!(a.get_sbyte_(5), 104_i8);
        assert_eq!(a.get_sbyte_(6), -93_i8);
        assert_eq!(a.get_sbyte_(7), -67_i8);
        assert_eq!(a.get_sbyte_(8), 88_i8);
        assert_eq!(a.get_sbyte_(9), -120_i8);
        assert_eq!(a.get_sbyte_(10), -50_i8);
        assert_eq!(a.get_sbyte_(11), 126_i8);
        assert_eq!(a.get_sbyte_(12), 26_i8);
        assert_eq!(a.get_sbyte_(13), 59_i8);
        assert_eq!(a.get_sbyte_(14), 18_i8);
        assert_eq!(a.get_sbyte_(15), -1_i8);
    }
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(a.get_uint_(0), 4048161615_u32);
        assert_eq!(a.get_uint_(1), 3181603061_u32);
        assert_eq!(a.get_sint_(0), -246805681_i32);
        assert_eq!(a.get_sint_(1), -1113364235_i32);
        assert_eq!(a.get_ushort_(0), 2895_u16);
        assert_eq!(a.get_ushort_(1), 61770_u16);
        assert_eq!(a.get_ushort_(2), 26869_u16);
        assert_eq!(a.get_ushort_(3), 48547_u16);
        assert_eq!(a.get_sshort_(0), 2895_i16);
        assert_eq!(a.get_sshort_(1), -3766_i16);
        assert_eq!(a.get_sshort_(2), 26869_i16);
        assert_eq!(a.get_sshort_(3), -16989_i16);
        assert_eq!(a.get_ubyte_(0), 79_u8);
        assert_eq!(a.get_ubyte_(1), 11_u8);
        assert_eq!(a.get_ubyte_(2), 74_u8);
        assert_eq!(a.get_ubyte_(3), 241_u8);
        assert_eq!(a.get_ubyte_(4), 245_u8);
        assert_eq!(a.get_ubyte_(5), 104_u8);
        assert_eq!(a.get_ubyte_(6), 163_u8);
        assert_eq!(a.get_ubyte_(7), 189_u8);
        assert_eq!(a.get_sbyte_(0), 79_i8);
        assert_eq!(a.get_sbyte_(1), 11_i8);
        assert_eq!(a.get_sbyte_(2), 74_i8);
        assert_eq!(a.get_sbyte_(3), -15_i8);
        assert_eq!(a.get_sbyte_(4), -11_i8);
        assert_eq!(a.get_sbyte_(5), 104_i8);
        assert_eq!(a.get_sbyte_(6), -93_i8);
        assert_eq!(a.get_sbyte_(7), -67_i8);
    }
    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(a.get_ushort_(0), 2895_u16);
        assert_eq!(a.get_ushort_(1), 61770_u16);
        assert_eq!(a.get_sshort_(0), 2895_i16);
        assert_eq!(a.get_sshort_(1), -3766_i16);
        assert_eq!(a.get_ubyte_(0), 79_u8);
        assert_eq!(a.get_ubyte_(1), 11_u8);
        assert_eq!(a.get_ubyte_(2), 74_u8);
        assert_eq!(a.get_ubyte_(3), 241_u8);
        assert_eq!(a.get_sbyte_(0), 79_i8);
        assert_eq!(a.get_sbyte_(1), 11_i8);
        assert_eq!(a.get_sbyte_(2), 74_i8);
        assert_eq!(a.get_sbyte_(3), -15_i8);
    }
    #[cfg(target_pointer_width = "16")]
    {
        assert_eq!(a.get_ubyte_(0), 79_u8);
        assert_eq!(a.get_ubyte_(1), 11_u8);
        assert_eq!(a.get_sbyte_(0), 79_i8);
        assert_eq!(a.get_sbyte_(1), 11_i8);
    }
    println!("--------------------------------------");
}

fn size_union_quick_start2()
{
    println!("size_union_quick_start2()");
    use cryptocol::number::{ SmallUInt, SizeUnion };

    let a_sizeunion: SizeUnion;
    let b_sizeunion: SizeUnion;
    let c_sizeunion: SizeUnion;
    #[cfg(target_pointer_width = "128")]
    {
        a_sizeunion = 123456789876543212345678987654321_usize.into_sizeunion();
        b_sizeunion = 876543210123456787654321012345678_usize.into_sizeunion();
        c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
    }
    #[cfg(target_pointer_width = "64")]
    {
        a_sizeunion = 12345678987654321_usize.into_sizeunion();
        b_sizeunion = 87654321012345678_usize.into_sizeunion();
        c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
    }
    #[cfg(target_pointer_width = "32")]
    {
        a_sizeunion = 12345678_usize.into_sizeunion();
        b_sizeunion = 87654321_usize.into_sizeunion();
        c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
    }
    #[cfg(target_pointer_width = "16")]
    {
        a_sizeunion = 1234_usize.into_sizeunion();
        b_sizeunion = 4321_usize.into_sizeunion();
        c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
    }
    #[cfg(target_pointer_width = "8")]
    {
        a_sizeunion = 12_usize.into_sizeunion();
        b_sizeunion = 87_usize.into_sizeunion();
        c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
    }
    println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
    #[cfg(target_pointer_width = "128")]    assert_eq!(c_sizeunion.get(), 999999999999999999999999999999999_usize);
    #[cfg(target_pointer_width = "64")]     assert_eq!(c_sizeunion.get(), 99999999999999999_usize);
    #[cfg(target_pointer_width = "32")]     assert_eq!(c_sizeunion.get(), 99999999_usize);
    #[cfg(target_pointer_width = "16")]     assert_eq!(c_sizeunion.get(), 5555_usize);
    #[cfg(target_pointer_width = "8")]      assert_eq!(c_sizeunion.get(), 55_usize);

    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..2
            { println!("c_sizeunion.get_ulong_({}) = {}", i, c_sizeunion.get_ulong_(i)); }
        assert_eq!(c_sizeunion.get_ulong_(0), 4089650035136921599_u64);
        assert_eq!(c_sizeunion.get_ulong_(1), 54210108624275_u64);
    }
    #[cfg(target_pointer_width = "64")]
    {
        for i in 0..2
            { println!("c_sizeunion.get_uint_({}) = {}", i, c_sizeunion.get_uint_(i)); }
        assert_eq!(c_sizeunion.get_uint_(0), 1569325055_u32);
        assert_eq!(c_sizeunion.get_uint_(1), 23283064_u32);
    }
    #[cfg(target_pointer_width = "32")]
    {
        for i in 0..2
            { println!("c_sizeunion.get_ushort_({}) = {}", i, c_sizeunion.get_ushort_(i)); }
        assert_eq!(c_sizeunion.get_ushort_(0), 57599_u16);
        assert_eq!(c_sizeunion.get_ushort_(1), 1525_u16);
    }
    #[cfg(target_pointer_width = "16")]
    {
        for i in 0..2
            { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
        assert_eq!(c_sizeunion.get_ubyte_(0), 179_u8);
        assert_eq!(c_sizeunion.get_ubyte_(1), 21_u8);
    }

    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..4
            { println!("c_sizeunion.get_uint_({}) = {}", i, c_sizeunion.get_uint_(i)); }
        assert_eq!(c_sizeunion.get_uint_(0), 4294967295_u32);
        assert_eq!(c_sizeunion.get_uint_(1), 952195849_u32);
        assert_eq!(c_sizeunion.get_uint_(2), 3326381459_u32);
        assert_eq!(c_sizeunion.get_uint_(3), 12621_u32);
    }
    #[cfg(target_pointer_width = "64")]
    {
        for i in 0..4
            { println!("c_sizeunion.get_ushort_({}) = {}", i, c_sizeunion.get_ushort_(i)); }
        assert_eq!(c_sizeunion.get_ushort_(0), 65535_u16);
        assert_eq!(c_sizeunion.get_ushort_(1), 23945_u16);
        assert_eq!(c_sizeunion.get_ushort_(2), 17784_u16);
        assert_eq!(c_sizeunion.get_ushort_(3), 355_u16);
    }
    #[cfg(target_pointer_width = "32")]
    {
        for i in 0..4
            { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
        assert_eq!(c_sizeunion.get_ubyte_(0), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(1), 224_u8);
        assert_eq!(c_sizeunion.get_ubyte_(2), 245_u8);
        assert_eq!(c_sizeunion.get_ubyte_(3), 5_u8);
    }
    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..8
            { println!("c_sizeunion.get_ushort_({}) = {}", i, c_sizeunion.get_ushort_(i)); }
        assert_eq!(c_sizeunion.get_ushort_(0), 65535_u16);
        assert_eq!(c_sizeunion.get_ushort_(1), 65535_u16);
        assert_eq!(c_sizeunion.get_ushort_(2), 23305_u16);
        assert_eq!(c_sizeunion.get_ushort_(3), 14529_u16);
        assert_eq!(c_sizeunion.get_ushort_(4), 36243_u16);
        assert_eq!(c_sizeunion.get_ushort_(5), 50756_u16);
        assert_eq!(c_sizeunion.get_ushort_(6), 12621_u16);
        assert_eq!(c_sizeunion.get_ushort_(7), 0_u16);
    }
    #[cfg(target_pointer_width = "64")]
    {
        for i in 0..8
            { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
        assert_eq!(c_sizeunion.get_ubyte_(0), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(1), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(2), 137_u8);
        assert_eq!(c_sizeunion.get_ubyte_(3), 93_u8);
        assert_eq!(c_sizeunion.get_ubyte_(4), 120_u8);
        assert_eq!(c_sizeunion.get_ubyte_(5), 69_u8);
        assert_eq!(c_sizeunion.get_ubyte_(6), 99_u8);
        assert_eq!(c_sizeunion.get_ubyte_(7), 1_u8);
    }
    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..16
            { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
        assert_eq!(c_sizeunion.get_ubyte_(0), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(1), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(2), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(3), 255_u8);
        assert_eq!(c_sizeunion.get_ubyte_(4), 9_u8);
        assert_eq!(c_sizeunion.get_ubyte_(5), 91_u8);
        assert_eq!(c_sizeunion.get_ubyte_(6), 193_u8);
        assert_eq!(c_sizeunion.get_ubyte_(7), 56_u8);
        assert_eq!(c_sizeunion.get_ubyte_(8), 147_u8);
        assert_eq!(c_sizeunion.get_ubyte_(9), 141_u8);
        assert_eq!(c_sizeunion.get_ubyte_(10), 68_u8);
        assert_eq!(c_sizeunion.get_ubyte_(11), 198_u8);
        assert_eq!(c_sizeunion.get_ubyte_(12), 77_u8);
        assert_eq!(c_sizeunion.get_ubyte_(13), 49_u8);
        assert_eq!(c_sizeunion.get_ubyte_(14), 0_u8);
        assert_eq!(c_sizeunion.get_ubyte_(15), 0_u8);
    }

    let d_sizeunion = b_sizeunion - a_sizeunion;
    println!("{} - {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
    #[cfg(target_pointer_width = "128")]
    {
        assert_eq!(d_sizeunion.get(), 753086420246913575308642024691357_usize);
        for i in 0..2
            { println!("d_sizeunion.get_ulong_({}) = {}", i, d_sizeunion.get_ulong_(i)); }
        assert_eq!(d_sizeunion.get_ulong_(0), 14084888390109238941_u64);
        assert_eq!(d_sizeunion.get_ulong_(1), 40824896645051_u64);
    }
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(d_sizeunion.get(), 75308642024691357_usize);
        for i in 0..2
            { println!("d_longunion.get_uint_({}) = {}", i, d_sizeunion.get_uint_(i)); }
        assert_eq!(d_sizeunion.get_uint_(0), 2556827293_u32);
        assert_eq!(d_sizeunion.get_uint_(1), 17534159_u32);
    }
    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(d_sizeunion.get(), 75308643_usize);
        for i in 0..2
            { println!("d_sizeunion.get_ushort_({}) = {}", i, d_sizeunion.get_ushort_(i)); }
        assert_eq!(d_sizeunion.get_ushort_(0), 7779_u16);
        assert_eq!(d_sizeunion.get_ushort_(1), 1149_u16);
    }
    #[cfg(target_pointer_width = "16")]
    {
        assert_eq!(d_sizeunion.get(), 3087_usize);
        for i in 0..2
            { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
        assert_eq!(d_sizeunion.get_ubyte_(0), 15_u8);
        assert_eq!(d_sizeunion.get_ubyte_(1), 12_u8);
    }
    #[cfg(target_pointer_width = "8")]  assert_eq!(d_sizeunion.get(), 75_usize);

    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..4
            { println!("d_sizeunion.get_uint_({}) = {}", i, d_sizeunion.get_uint_(i)); }
        assert_eq!(d_sizeunion.get_uint_(0), 2843481757_u32);
        assert_eq!(d_sizeunion.get_uint_(1), 3279393629_u32);
        assert_eq!(d_sizeunion.get_uint_(2), 1232496571_u32);
        assert_eq!(d_sizeunion.get_uint_(3), 9505_u32);
    }
    #[cfg(target_pointer_width = "64")]
    {
        for i in 0..4
            { println!("d_sizeunion.get_ushort_({}) = {}", i, d_sizeunion.get_ushort_(i)); }
        assert_eq!(d_sizeunion.get_ushort_(0), 5789_u16);
        assert_eq!(d_sizeunion.get_ushort_(1), 39014_u16);
        assert_eq!(d_sizeunion.get_ushort_(2), 36047_u16);
        assert_eq!(d_sizeunion.get_ushort_(3), 267_u16);
    }
    #[cfg(target_pointer_width = "32")]
    {
        for i in 0..4
            { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
        assert_eq!(d_sizeunion.get_ubyte_(0), 99_u8);
        assert_eq!(d_sizeunion.get_ubyte_(1), 30_u8);
        assert_eq!(d_sizeunion.get_ubyte_(2), 125_u8);
        assert_eq!(d_sizeunion.get_ubyte_(3), 4_u8);
    }

    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..8
            { println!("d_sizeunion.get_ushort_({}) = {}", i, d_sizeunion.get_ushort_(i)); }
        assert_eq!(d_sizeunion.get_ushort_(0), 5789_u16);
        assert_eq!(d_sizeunion.get_ushort_(1), 43388_u16);
        assert_eq!(d_sizeunion.get_ushort_(2), 37725_u16);
        assert_eq!(d_sizeunion.get_ushort_(3), 50039_u16);
        assert_eq!(d_sizeunion.get_ushort_(4), 26555_u16);
        assert_eq!(d_sizeunion.get_ushort_(5), 18806_u16);
        assert_eq!(d_sizeunion.get_ushort_(6), 9505_u16);
        assert_eq!(d_sizeunion.get_ushort_(7), 0_u16);
    }
    #[cfg(target_pointer_width = "64")]
    {
        for i in 0..8
            { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
        assert_eq!(d_sizeunion.get_ubyte_(0), 157_u8);
        assert_eq!(d_sizeunion.get_ubyte_(1), 22_u8);
        assert_eq!(d_sizeunion.get_ubyte_(2), 102_u8);
        assert_eq!(d_sizeunion.get_ubyte_(3), 152_u8);
        assert_eq!(d_sizeunion.get_ubyte_(4), 207_u8);
        assert_eq!(d_sizeunion.get_ubyte_(5), 140_u8);
        assert_eq!(d_sizeunion.get_ubyte_(6), 11_u8);
        assert_eq!(d_sizeunion.get_ubyte_(7), 1_u8);
    }
    #[cfg(target_pointer_width = "128")]
    {
        for i in 0..16
            { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
        assert_eq!(d_sizeunion.get_ubyte_(0), 157_u8);
        assert_eq!(d_sizeunion.get_ubyte_(1), 22_u8);
        assert_eq!(d_sizeunion.get_ubyte_(2), 124_u8);
        assert_eq!(d_sizeunion.get_ubyte_(3), 169_u8);
        assert_eq!(d_sizeunion.get_ubyte_(4), 93_u8);
        assert_eq!(d_sizeunion.get_ubyte_(5), 147_u8);
        assert_eq!(d_sizeunion.get_ubyte_(6), 119_u8);
        assert_eq!(d_sizeunion.get_ubyte_(7), 195_u8);
        assert_eq!(d_sizeunion.get_ubyte_(8), 187_u8);
        assert_eq!(d_sizeunion.get_ubyte_(9), 103_u8);
        assert_eq!(d_sizeunion.get_ubyte_(10), 118_u8);
        assert_eq!(d_sizeunion.get_ubyte_(11), 73_u8);
        assert_eq!(d_sizeunion.get_ubyte_(12), 33_u8);
        assert_eq!(d_sizeunion.get_ubyte_(13), 37_u8);
        assert_eq!(d_sizeunion.get_ubyte_(14), 0_u8);
        assert_eq!(d_sizeunion.get_ubyte_(15), 0_u8);
    }

    let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion();
    println!("{} * {} = {}", d_sizeunion, 3_usize.into_sizeunion(), e_sizeunion);
    #[cfg(target_pointer_width = "128")]    assert_eq!(e_sizeunion.get(), 2259259260740740725925926074074071_usize);
    #[cfg(target_pointer_width = "64")]     assert_eq!(e_sizeunion.get(), 225925926074074071_usize);
    #[cfg(target_pointer_width = "32")]     assert_eq!(e_sizeunion.get(), 225925929_usize);
    #[cfg(target_pointer_width = "16")]     assert_eq!(e_sizeunion.get(), 9261_usize);
    #[cfg(target_pointer_width = "8")]      assert_eq!(e_sizeunion.get(), 225_usize);

    let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion();
    println!("{} / {} = {}", c_sizeunion, 10_usize.into_sizeunion(), f_sizeunion);
    #[cfg(target_pointer_width = "128")]    assert_eq!(f_sizeunion.get(), 99999999999999999999999999999999_usize);
    #[cfg(target_pointer_width = "64")]     assert_eq!(f_sizeunion.get(), 9999999999999999_usize);
    #[cfg(target_pointer_width = "32")]     assert_eq!(f_sizeunion.get(), 9999999_usize);
    #[cfg(target_pointer_width = "16")]     assert_eq!(f_shortunion.get(), 555_usize);
    #[cfg(target_pointer_width = "8")]      assert_eq!(f_shortunion.get(), 9_usize);

    let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion();
    println!("{} % {} = {}", c_sizeunion, 10_usize.into_sizeunion(), g_sizeunion);
    #[cfg(target_pointer_width = "128")]    assert_eq!(g_sizeunion.get(), 9_usize);
    #[cfg(target_pointer_width = "64")]     assert_eq!(g_sizeunion.get(), 9_usize);
    #[cfg(target_pointer_width = "32")]     assert_eq!(g_sizeunion.get(), 9_usize);
    #[cfg(target_pointer_width = "16")]     assert_eq!(g_sizeunion.get(), 5_usize);
    #[cfg(target_pointer_width = "8")]      assert_eq!(g_sizeunion.get(), 9_usize);
    println!("--------------------------------------");
}

fn size_union_new()
{
    println!("size_union_new()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_usize);
    println!("--------------------------------------");
}

fn size_union_new_with()
{
    println!("size_union_new_with()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new_with(234_usize);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 234_usize);
    println!("--------------------------------------");
}

fn size_union_new_with_signed()
{
    println!("size_union_new_with_signed()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new_with_signed(-123_isize);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -123_isize);
    println!("--------------------------------------");
}

fn size_union_new_with_bytes()
{
    println!("size_union_new_with_bytes()");
    #[cfg(target_pointer_width = "16")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_ubytes([172_u8, 216_u8]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 55468_usize);
    }
    #[cfg(target_pointer_width = "32")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_ubytes([222_u8, 0_u8, 230_u8, 228_u8]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 3840278750_usize);
    }
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_ubytes([131_u8, 21_u8, 104_u8, 195_u8, 42_u8, 157_u8, 251_u8, 255_u8]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 18445509505818563971_usize);
    }
    #[cfg(target_pointer_width = "128")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_ubytes([79_u8, 11_u8, 74_u8, 241_u8, 245_u8, 104_u8, 163_u8, 189_u8, 88_u8, 136_u8, 206_u8, 126_u8, 26_u8, 59_u8, 18_u8, 255_u8]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
    }
    println!("--------------------------------------");
}

fn size_union_new_with_shorts()
{
    println!("size_union_new_with_shorts()");
    #[cfg(target_pointer_width = "32")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_ushorts([222_u16, 58598_u16]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 3840278750_usize);
    }
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_ushorts([5507_u16, 50024_u16, 40234_u16, 65531_u16]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 18445509505818563971_usize);
    }
    #[cfg(target_pointer_width = "128")]
    {
        use cryptocol::number::SizeUnion;
        let arr = [2895_u16, 61770_u16, 26869_u16, 48547_u16, 34904_u16, 32462_u16, 15130_u16, 65298_u16];
        let a = SizeUnion::new_with_ushorts(arr);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
    }
    println!("--------------------------------------");
}

fn size_union_new_with_ints()
{
    println!("size_union_new_with_ints()");
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a = SizeUnion::new_with_uints([3278378371_u32, 4294679850_u32]);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 18445509505818563971_usize);
    }
    #[cfg(target_pointer_width = "128")]
    {
        use cryptocol::number::SizeUnion;
        let arr = [4048161615_u32, 3181603061_u32, 2127464536_u32, 4279384858_u32];
        let a = SizeUnion::new_with_uints(arr);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
    }
    println!("--------------------------------------");
}

fn size_union_new_with_longs()
{
    println!("size_union_new_with_longs()");
    #[cfg(target_pointer_width = "128")]
    {
        use cryptocol::number::SizeUnion;
        let arr = [13664881099896654671_u64, 18379818014235068504_u64];
        let a = SizeUnion::new_with_ulongs(arr);
        println!("a = {}", a.get());
        assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
    }
    println!("--------------------------------------");
}

fn size_union_new_with_u128()
{
    println!("size_union_new_with_u128()");
    use cryptocol::number::SizeUnion;
    let a = SizeUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    println!("a = {}", a.get());
    #[cfg(target_pointer_width = "128")]    assert_eq!(a.get(), 123456789012345678901234567890123456789_usize);
    #[cfg(target_pointer_width = "64")]     assert_eq!(a.get(), 12312739301371248917_usize);
    #[cfg(target_pointer_width = "32")]     assert_eq!(a.get(), 2923004181_usize);
    #[cfg(target_pointer_width = "16")]     assert_eq!(a.get(), 33045_usize);
    #[cfg(target_pointer_width = "8")]      assert_eq!(a.get(), 21_usize);
    println!("--------------------------------------");
}

fn size_union_new_with_bool()
{
    println!("size_union_new_with_bool()");
    use cryptocol::number::SizeUnion;
    let a = SizeUnion::new_with_bool(true);
    let b = SizeUnion::new_with_bool(false);
    println!("a = {}", a.get());
    println!("b = {}", b.get());
    assert_eq!(a.get(), 1_usize);
    assert_eq!(b.get(), 0_usize);
    println!("--------------------------------------");
}

fn size_union_zero()
{
    println!("size_union_zero()");
    use cryptocol::number::SizeUnion;
    let a = SizeUnion::zero();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 0_usize);
    println!("--------------------------------------");
}

fn size_union_one()
{
    println!("size_union_one()");
    use cryptocol::number::SizeUnion;
    let a = SizeUnion::one();
    println!("a = {}", a.get());
    assert_eq!(a.get(), 1_usize);
    println!("--------------------------------------");
}

fn size_union_get()
{
    println!("size_union_get()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new_with(250_usize);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 250_usize);
    println!("--------------------------------------");
}

fn size_union_set()
{
    println!("size_union_set()");
    use cryptocol::number::SizeUnion;    
    let mut a = SizeUnion::new();
    a.set(234_usize);
    println!("a = {}", a.get());
    assert_eq!(a.get(), 234_usize);
    println!("--------------------------------------");
}

fn size_union_get_signed()
{
    println!("size_union_get_signed()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new_with_signed(-123_isize);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -123_isize);
    println!("--------------------------------------");
}

fn size_union_set_signed()
{
    println!("size_union_set_signed()");
    use cryptocol::number::SizeUnion;
    let mut a = SizeUnion::new();
    a.set_signed(-123_isize);
    println!("a = {}", a.get_signed());
    assert_eq!(a.get_signed(), -123_isize);
    println!("--------------------------------------");
}

fn size_union_get_usize()
{
    println!("size_union_get_usize()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new_with(250_usize);
    println!("a = {}", a.get_usize());
    assert_eq!(a.get_usize(), 250_usize);
    println!("--------------------------------------");
}

fn size_union_set_usize()
{
    println!("size_union_set_usize()");
    use cryptocol::number::SizeUnion;    
    let mut a = SizeUnion::new();
    a.set(234_usize);
    println!("a = {}", a.get_usize());
    assert_eq!(a.get_usize(), 234_usize);
    println!("--------------------------------------");
}

fn size_union_get_ssize()
{
    println!("size_union_get_ssize()");
    use cryptocol::number::SizeUnion;    
    let a = SizeUnion::new_with_signed(-123_isize);
    println!("a = {}", a.get_ssize());
    assert_eq!(a.get_ssize(), -123_isize);
    println!("--------------------------------------");
}

fn size_union_set_ssize()
{
    println!("size_union_set_ssize()");
    use cryptocol::number::SizeUnion;
    let mut a = SizeUnion::new();
    a.set_ssize(-123_isize);
    println!("a = {}", a.get_ssize());
    assert_eq!(a.get_ssize(), -123_isize);
    println!("--------------------------------------");
}


fn shared_values_main()
{
    shared_values_quick_start();
    shared_values_new();
    shared_values_from_src();
    shared_values_get_des();
    shared_values_get_src();
    shared_values_is_src_zero();
}

fn shared_values_quick_start()
{
    println!("shared_values_quick_start()");
    use cryptocol::number::{ SmallUInt, SharedValues, IntUnion, LongerUnion };

    let a = SharedValues::<u16, u128> { src: 123456789123456789123456789123456789123_u128 };
    println!("source = {}, Destination = {}", unsafe {a.src}, unsafe {a.des});
    assert_eq!(unsafe { a.src }, 123456789123456789123456789123456789123_u128);
    assert_eq!(unsafe { a.des }, 27267_u16);
    
    let mut b = SharedValues::<IntUnion, u128>::new();
    b.src = 123456789123456789123456789123456789123_u128;
    println!("source = {}, Destination = {}", b.get_src(), b.get_des().get());
    assert_eq!(b.get_src(), 123456789123456789123456789123456789123_u128);
    assert_eq!(b.get_des().get(), 2970839683_u32);

    let c = SharedValues::<u16, LongerUnion>::from_src(123456789123456789123456789123456789123_u128.into_longerunion());
    println!("source = {}, Destination = {}", unsafe {c.src}, unsafe {c.des});
    assert_eq!(unsafe { c.src.get() }, 123456789123456789123456789123456789123_u128);
    assert_eq!(unsafe { c.des }, 27267_u16);
    
    let d = SharedValues::<IntUnion, LongerUnion>::from_src(123456789123456789123456789123456789123_u128.into_longerunion());
    println!("source = {}, Destination = {}", d.get_src().get(), d.get_des().get());
    assert_eq!(d.get_src().get(), 123456789123456789123456789123456789123_u128);
    assert_eq!(d.get_des().get(), 2970839683_u32);
    println!("--------------------------------------");
}

fn shared_values_new()
{
    println!("shared_values_new()");
    use cryptocol::number::SharedValues;    
    let a = SharedValues::<u16, u128>::new();
    println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    assert_eq!(a.get_src(), 0_u128);
    assert_eq!(a.get_des(), 0_u16);
    println!("--------------------------------------");
}

fn shared_values_from_src()
{
    println!("shared_values_from_src()");
    use cryptocol::number::SharedValues;
    let a = SharedValues::<u32, u128>::from_src(123456789123456789123456789123456789123_u128);
    println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    assert_eq!(a.get_src(), 123456789123456789123456789123456789123_u128);
    assert_eq!(a.get_des(), 2970839683_u32);
    println!("--------------------------------------");
}

fn shared_values_get_des()
{
    println!("shared_values_get_des()");
    use cryptocol::number::SharedValues;
    let a = SharedValues::<u16, u128>::from_src(123456789123456789123456789123456789123_u128);
    println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    assert_eq!(a.get_src(), 123456789123456789123456789123456789123_u128);
    assert_eq!(a.get_des(), 27267_u16);
    println!("--------------------------------------");
}

fn shared_values_get_src()
{
    println!("shared_values_get_src()");
    use cryptocol::number::SharedValues;
    let a = SharedValues::<u32, u64>::from_src(123456789123456789_u64);
    println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    assert_eq!(a.get_src(), 123456789123456789_u64);
    assert_eq!(a.get_des(), 2899336981_u32);
    println!("--------------------------------------");
}

fn shared_values_is_src_zero()
{
    println!("shared_values_is_src_zero()");
    use cryptocol::number::SharedValues;
    let a = SharedValues::<u32, u64>::new();
    println!("Is a.src zero? {}", if a.is_src_zero() {"yes"} else {"no"});
    assert_eq!(a.is_src_zero(), true);

    let b = SharedValues::<u16, u128>::from_src(123456789123456789123456789123456789123_u128);
    println!("Is b.src zero? {}", if b.is_src_zero() {"yes"} else {"no"});
    assert_eq!(b.is_src_zero(), false);
    println!("--------------------------------------");
}

fn shared_arrays_main()
{
    shared_arrays_quick_start();
    shared_arrays_new();
    shared_arrays_from_src();
    shared_arrays_get_src();
    shared_arrays_get_src_elem();
    shared_arrays_get_src_elem_();
    shared_arrays_get_des();
    shared_arrays_get_des_elem();
    shared_arrays_get_des_elem_();
    shared_arrays_put_des();
    shared_arrays_size_of_src();
    shared_arrays_size_of_des();
    shared_arrays_length_of_src();
    shared_arrays_length_of_des();
}

fn shared_arrays_quick_start()
{
    println!("shared_arrays_quick_start()");
    use cryptocol::number::{ SmallUInt, SharedArrays, IntUnion, LongUnion };

    let a = SharedArrays::<u16, 4, u64, 2> { src: [123456789123456789_u64, 987654321987654321_u64] };
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", a.get_src_elem_(i)); }
    println!("]");
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", a.get_des_elem_(i)); }
    println!("]");
    assert_eq!(a.get_src_elem_(0), 123456789123456789_u64);
    assert_eq!(a.get_src_elem_(1), 987654321987654321_u64);
    assert_eq!(a.get_des_elem_(0), 24341_u16);
    assert_eq!(a.get_des_elem_(1), 44240_u16);
    assert_eq!(a.get_des_elem_(2), 39755_u16);
    assert_eq!(a.get_des_elem_(3), 438_u16);

    let mut b = SharedArrays::<IntUnion, 4, u64, 2>::new();
    b.src = [123456789123456789_u64, 987654321987654321_u64];
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", b.get_src_elem_(i)); }
    println!("]");
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", b.get_des_elem_(i)); }
    println!("]");
    assert_eq!(b.get_src_elem_(0), 123456789123456789_u64);
    assert_eq!(b.get_src_elem_(1), 987654321987654321_u64);
    assert_eq!(b.get_des_elem_(0).get(), 2899336981_u32);
    assert_eq!(b.get_des_elem_(1).get(), 28744523_u32);
    assert_eq!(b.get_des_elem_(2).get(), 2129924785_u32);
    assert_eq!(b.get_des_elem_(3).get(), 229956191_u32);
    
    let c = SharedArrays::<u16, 4, LongUnion, 2>::from_src(&[123456789123456789_u64.into_longunion(), 987654321987654321_u64.into_longunion()]);
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", c.get_src_elem_(i)); }
    println!("]");
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", c.get_des_elem_(i)); }
    println!("]");
    assert_eq!(c.get_src_elem_(0).get(), 123456789123456789_u64);
    assert_eq!(c.get_src_elem_(1).get(), 987654321987654321_u64);
    assert_eq!(c.get_des_elem_(0), 24341_u16);
    assert_eq!(c.get_des_elem_(1), 44240_u16);
    assert_eq!(c.get_des_elem_(2), 39755_u16);
    assert_eq!(c.get_des_elem_(3), 438_u16);
        
    let d = SharedArrays::<IntUnion, 4, LongUnion, 2>::from_src(&[123456789123456789_u64.into_longunion(), 987654321987654321_u64.into_longunion()]);
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", d.get_src_elem_(i)); }
    println!("]");
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", d.get_des_elem_(i)); }
    println!("]");
    assert_eq!(d.get_src_elem_(0).get(), 123456789123456789_u64);
    assert_eq!(d.get_src_elem_(1).get(), 987654321987654321_u64);
    assert_eq!(d.get_des_elem_(0).get(), 2899336981_u32);
    assert_eq!(d.get_des_elem_(1).get(), 28744523_u32);
    assert_eq!(d.get_des_elem_(2).get(), 2129924785_u32);
    assert_eq!(d.get_des_elem_(3).get(), 229956191_u32);
    println!("--------------------------------------");
}

fn shared_arrays_new()
{
    println!("shared_arrays_new()");
    use cryptocol::number::SharedArrays;    
    let a = SharedArrays::<u32, 4, u64, 2>::new();
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", a.get_src_elem_(i)); }
    println!("]");
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", a.get_des_elem_(i)); }
    println!("]");
    assert_eq!(a.get_src_elem_(0), 0_u64);
    assert_eq!(a.get_src_elem_(1), 0_u64);
    assert_eq!(a.get_des_elem_(0), 0_u32);
    assert_eq!(a.get_des_elem_(1), 0_u32);
    assert_eq!(a.get_des_elem_(2), 0_u32);
    assert_eq!(a.get_des_elem_(3), 0_u32);
    println!("--------------------------------------");
}

fn shared_arrays_from_src()
{
    println!("shared_arrays_from_src()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", a.get_src_elem_(i)); }
    println!("]");
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", a.get_des_elem_(i)); }
    println!("]");
    assert_eq!(a.get_src_elem_(0), 123456789123456789_u64);
    assert_eq!(a.get_src_elem_(1), 987654321987654321_u64);
    assert_eq!(a.get_des_elem_(0), 2899336981_u32);
    assert_eq!(a.get_des_elem_(1), 28744523_u32);
    assert_eq!(a.get_des_elem_(2), 2129924785_u32);
    assert_eq!(a.get_des_elem_(3), 229956191_u32);
    println!("--------------------------------------");
}

fn shared_arrays_get_src()
{
    println!("shared_arrays_get_src()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    let b = a.get_src();
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", b[i]); }
    println!("]");
    assert_eq!(b[0], 123456789123456789_u64);
    assert_eq!(b[1], 987654321987654321_u64);
    println!("--------------------------------------");
}

fn shared_arrays_get_src_elem()
{
    println!("shared_arrays_get_src_elem()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    print!("source = [ ");
    for i in 0..2
    {
        match a.get_src_elem(i)
        {
            Some(b) =>  { print!("{} ", b); }
            None =>     { print!("None "); }
        }
    }
    println!("]");
    assert_eq!(a.get_src_elem(0).unwrap(), 123456789123456789_u64);
    assert_eq!(a.get_src_elem(1).unwrap(), 987654321987654321_u64);
    println!("--------------------------------------");
}

fn shared_arrays_get_src_elem_()
{
    println!("shared_arrays_get_src_elem_()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    print!("source = [ ");
    for i in 0..2
        { print!("{} ", a.get_src_elem_(i)); }
    println!("]");
    assert_eq!(a.get_src_elem_(0), 123456789123456789_u64);
    assert_eq!(a.get_src_elem_(1), 987654321987654321_u64);
    println!("--------------------------------------");
}

fn shared_arrays_get_des()
{
    println!("shared_arrays_get_des()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    let b = a.get_des();
    print!("Destination = [ ");
    for i in 0..4
        { print!("{} ", b[i]); }
    println!("]");
    assert_eq!(b[0], 2899336981_u32);
    assert_eq!(b[1], 28744523_u32);
    assert_eq!(b[2], 2129924785_u32);
    assert_eq!(b[3], 229956191_u32);
    println!("--------------------------------------");
}

fn shared_arrays_get_des_elem()
{
    println!("shared_arrays_get_des_elem()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    print!("destination = [ ");
    for i in 0..4
    {
        match a.get_des_elem(i)
        {
            Some(b) =>  { print!("{} ", b); }
            None =>     { print!("None "); }
        }
    }
    println!("]");
    assert_eq!(a.get_des_elem(0).unwrap(), 2899336981_u32);
    assert_eq!(a.get_des_elem(1).unwrap(), 28744523_u32);
    assert_eq!(a.get_des_elem(2).unwrap(), 2129924785_u32);
    assert_eq!(a.get_des_elem(3).unwrap(), 229956191_u32);
    println!("--------------------------------------");
}

fn shared_arrays_get_des_elem_()
{
    println!("shared_arrays_get_des_elem_()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    print!("destination = [ ");
    for i in 0..4
        { print!("{} ", a.get_des_elem_(i)); }
    println!("]");
    assert_eq!(a.get_des_elem_(0), 2899336981_u32);
    assert_eq!(a.get_des_elem_(1), 28744523_u32);
    assert_eq!(a.get_des_elem_(2), 2129924785_u32);
    assert_eq!(a.get_des_elem_(3), 229956191_u32);
    println!("--------------------------------------");
}

fn shared_arrays_put_des()
{
    println!("shared_arrays_put_des()");
    use cryptocol::number::SharedArrays;
    let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    let mut b = [0_u32; 4];
    a.put_des_in_array(&mut b);
    print!("destination = [ ");
    for i in 0..4
        { print!("{} ", b[i]); }
    println!("]");
    assert_eq!(b[0], 2899336981_u32);
    assert_eq!(b[1], 28744523_u32);
    assert_eq!(b[2], 2129924785_u32);
    assert_eq!(b[3], 229956191_u32);
    println!("--------------------------------------");
}

fn shared_arrays_size_of_src()
{
    println!("shared_arrays_size_of_src()");
    use cryptocol::number::SharedArrays;
    type Shared = SharedArrays::<u32, 5, u64, 3>;
    println!("The size of src is {}.",  Shared::size_of_src());
    assert_eq!(Shared::size_of_src(), 24);
    println!("--------------------------------------");
}

fn shared_arrays_size_of_des()
{
    println!("shared_arrays_size_of_des()");
    use cryptocol::number::SharedArrays;
    type Shared = SharedArrays::<u32, 5, u64, 3>;
    println!("The size of des is {}.",  Shared::size_of_des());
    assert_eq!(Shared::size_of_des(), 20);
    println!("--------------------------------------");
}

fn shared_arrays_length_of_src()
{
    println!("shared_arrays_length_of_src()");
    use cryptocol::number::SharedArrays;
    type Shared = SharedArrays::<u32, 5, u64, 3>;
    let a = Shared::from_src(&[123456789123456789_u64, 987654321987654321_u64, 13579246801357924680_u64]);
    println!("The size of src is {}.", a.length_of_src());
    assert_eq!(a.length_of_src(), 24);
    println!("--------------------------------------");
}

fn shared_arrays_length_of_des()
{
    println!("shared_arrays_length_of_des()");
    use cryptocol::number::SharedArrays;
    type Shared = SharedArrays::<u32, 5, u64, 3>;
    let a = Shared::from_src(&[123456789123456789_u64, 987654321987654321_u64, 13579246801357924680_u64]);
    println!("The size of des is {}.", a.length_of_des());
    assert_eq!(a.length_of_des(), 20);
    println!("--------------------------------------");
}


fn unions_get_set_byte_main()
{
    unions_get_ubyte_();
    unions_set_ubyte_();
    unions_get_sbyte_();
    unions_set_sbyte_();
    unions_get_ubyte();
    unions_set_ubyte();
    unions_get_sbyte();
    unions_set_sbyte();
}

fn unions_get_ubyte_()
{
    println!("unions_get_ubyte_()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let a_shortunion = ShortUnion::new_with(2895_u16);
    for i in 0..2
        { println!("a_shortunion.get_ubyte_({}) = {}", i, a_shortunion.get_ubyte_(i)); }
    assert_eq!(a_shortunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_shortunion.get_ubyte_(1), 11_u8);

    // It will panic.
    // println!("a_shortunion.get_ubyte_(2) = {}", a_shortunion.get_ubyte_(2));

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..4
        { println!("a_intunion.get_ubyte_({}) = {}", i, a_intunion.get_ubyte_(i)); }
    assert_eq!(a_intunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_intunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_intunion.get_ubyte_(2), 74_u8);
    assert_eq!(a_intunion.get_ubyte_(3), 241_u8);

    // It will panic.
    // println!("a_intunion.get_ubyte_(4) = {}", a_intunion.get_ubyte_(4));

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..8
        { println!("a_longunion.get_ubyte_({}) = {}", i, a_longunion.get_ubyte_(i)); }
    assert_eq!(a_longunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_longunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_longunion.get_ubyte_(2), 74_u8);
    assert_eq!(a_longunion.get_ubyte_(3), 241_u8);
    assert_eq!(a_longunion.get_ubyte_(4), 245_u8);
    assert_eq!(a_longunion.get_ubyte_(5), 104_u8);
    assert_eq!(a_longunion.get_ubyte_(6), 163_u8);
    assert_eq!(a_longunion.get_ubyte_(7), 189_u8);

    // It will panic.
    // println!("a_longunion.get_ubyte_(8) = {}", a_longunion.get_ubyte_(8));

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..16
        { println!("a_longerunion.get_ubyte_({}) = {}", i, a_longerunion.get_ubyte_(i)); }
    assert_eq!(a_longerunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_longerunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_longerunion.get_ubyte_(2), 74_u8);
    assert_eq!(a_longerunion.get_ubyte_(3), 241_u8);
    assert_eq!(a_longerunion.get_ubyte_(4), 245_u8);
    assert_eq!(a_longerunion.get_ubyte_(5), 104_u8);
    assert_eq!(a_longerunion.get_ubyte_(6), 163_u8);
    assert_eq!(a_longerunion.get_ubyte_(7), 189_u8);
    assert_eq!(a_longerunion.get_ubyte_(8), 88_u8);
    assert_eq!(a_longerunion.get_ubyte_(9), 136_u8);
    assert_eq!(a_longerunion.get_ubyte_(10), 206_u8);
    assert_eq!(a_longerunion.get_ubyte_(11), 126_u8);
    assert_eq!(a_longerunion.get_ubyte_(12), 26_u8);
    assert_eq!(a_longerunion.get_ubyte_(13), 59_u8);
    assert_eq!(a_longerunion.get_ubyte_(14), 18_u8);
    assert_eq!(a_longerunion.get_ubyte_(15), 255_u8);

    // It will panic.
    // println!("a_longerunion.get_ubyte_(16) = {}", a_longerunion.get_ubyte_(16));

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..8
            { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        assert_eq!(a_sizeunion.get_ubyte_(2), 74_u8);
        assert_eq!(a_sizeunion.get_ubyte_(3), 241_u8);
        assert_eq!(a_sizeunion.get_ubyte_(4), 245_u8);
        assert_eq!(a_sizeunion.get_ubyte_(5), 104_u8);
        assert_eq!(a_sizeunion.get_ubyte_(6), 163_u8);
        assert_eq!(a_sizeunion.get_ubyte_(7), 189_u8);
    
        // It will panic.
        // println!("a_sizeunion.get_ubyte_(8) = {}", a_sizeunion.get_ubyte_(8));
    }
    println!("--------------------------------------");
}

fn unions_set_ubyte_()
{
    println!("unions_set_ubyte_()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let mut a_shortunion = ShortUnion::new();
    a_shortunion.set_ubyte_(0, 79_u8);
    a_shortunion.set_ubyte_(1, 11_u8);
    // It will panic.
    // a_shortunion.set_ubyte_(2, 100_u8);
    println!("a_shortunion.get() = {}", a_shortunion.get());
    for i in 0..2
        { println!("a_shortunion.get_ubyte_({}) = {}", i, a_shortunion.get_ubyte_(i)); }
    assert_eq!(a_shortunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_shortunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_shortunion.get(), 2895_u16);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    a_intunion.set_ubyte_(0, 79_u8);
    a_intunion.set_ubyte_(1, 11_u8);
    a_intunion.set_ubyte_(2, 74_u8);
    a_intunion.set_ubyte_(3, 241_u8);
    // It will panic.
    // a_intunion.set_ubyte_(4, 100_u8);
    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..4
        { println!("a_intunion.get_ubyte_({}) = {}", i, a_intunion.get_ubyte_(i)); }
    assert_eq!(a_intunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_intunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_intunion.get_ubyte_(2), 74_u8);
    assert_eq!(a_intunion.get_ubyte_(3), 241_u8);
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    a_longunion.set_ubyte_(0, 79_u8);
    a_longunion.set_ubyte_(1, 11_u8);
    a_longunion.set_ubyte_(2, 74_u8);
    a_longunion.set_ubyte_(3, 241_u8);
    a_longunion.set_ubyte_(4, 245_u8);
    a_longunion.set_ubyte_(5, 104_u8);
    a_longunion.set_ubyte_(6, 163_u8);
    a_longunion.set_ubyte_(7, 189_u8);
    // It will panic.
    // a_longunion.set_ubyte_(8, 100_u8);
    for i in 0..8
        { println!("a_longunion.get_ubyte_({}) = {}", i, a_longunion.get_ubyte_(i)); }
    assert_eq!(a_longunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_longunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_longunion.get_ubyte_(2), 74_u8);
    assert_eq!(a_longunion.get_ubyte_(3), 241_u8);
    assert_eq!(a_longunion.get_ubyte_(4), 245_u8);
    assert_eq!(a_longunion.get_ubyte_(5), 104_u8);
    assert_eq!(a_longunion.get_ubyte_(6), 163_u8);
    assert_eq!(a_longunion.get_ubyte_(7), 189_u8);
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_ubyte_(0, 79_u8);
    a_longerunion.set_ubyte_(1, 11_u8);
    a_longerunion.set_ubyte_(2, 74_u8);
    a_longerunion.set_ubyte_(3, 241_u8);
    a_longerunion.set_ubyte_(4, 245_u8);
    a_longerunion.set_ubyte_(5, 104_u8);
    a_longerunion.set_ubyte_(6, 163_u8);
    a_longerunion.set_ubyte_(7, 189_u8);
    a_longerunion.set_ubyte_(8, 88_u8);
    a_longerunion.set_ubyte_(9, 136_u8);
    a_longerunion.set_ubyte_(10, 206_u8);
    a_longerunion.set_ubyte_(11, 126_u8);
    a_longerunion.set_ubyte_(12, 26_u8);
    a_longerunion.set_ubyte_(13, 59_u8);
    a_longerunion.set_ubyte_(14, 18_u8);
    a_longerunion.set_ubyte_(15, 255_u8);
    // It will panic.
    // a_longerunion.set_ubyte_(16, 100_u8);
    for i in 0..16
        { println!("a_longerunion.get_ubyte_({}) = {}", i, a_longerunion.get_ubyte_(i)); }
    assert_eq!(a_longerunion.get_ubyte_(0), 79_u8);
    assert_eq!(a_longerunion.get_ubyte_(1), 11_u8);
    assert_eq!(a_longerunion.get_ubyte_(2), 74_u8);
    assert_eq!(a_longerunion.get_ubyte_(3), 241_u8);
    assert_eq!(a_longerunion.get_ubyte_(4), 245_u8);
    assert_eq!(a_longerunion.get_ubyte_(5), 104_u8);
    assert_eq!(a_longerunion.get_ubyte_(6), 163_u8);
    assert_eq!(a_longerunion.get_ubyte_(7), 189_u8);
    assert_eq!(a_longerunion.get_ubyte_(8), 88_u8);
    assert_eq!(a_longerunion.get_ubyte_(9), 136_u8);
    assert_eq!(a_longerunion.get_ubyte_(10), 206_u8);
    assert_eq!(a_longerunion.get_ubyte_(11), 126_u8);
    assert_eq!(a_longerunion.get_ubyte_(12), 26_u8);
    assert_eq!(a_longerunion.get_ubyte_(13), 59_u8);
    assert_eq!(a_longerunion.get_ubyte_(14), 18_u8);
    assert_eq!(a_longerunion.get_ubyte_(15), 255_u8);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        a_sizeunion.set_ubyte_(0, 79_u8);
        a_sizeunion.set_ubyte_(1, 11_u8);
        a_sizeunion.set_ubyte_(2, 74_u8);
        a_sizeunion.set_ubyte_(3, 241_u8);
        a_sizeunion.set_ubyte_(4, 245_u8);
        a_sizeunion.set_ubyte_(5, 104_u8);
        a_sizeunion.set_ubyte_(6, 163_u8);
        a_sizeunion.set_ubyte_(7, 189_u8);
        // It will panic.
        // a_sizeunion.set_ubyte_(8, 100_u8);
        for i in 0..8
            { println!("a_longunion.get_ubyte_({}) = {}", i, a_longunion.get_ubyte_(i)); }
        assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        assert_eq!(a_sizeunion.get_ubyte_(2), 74_u8);
        assert_eq!(a_sizeunion.get_ubyte_(3), 241_u8);
        assert_eq!(a_sizeunion.get_ubyte_(4), 245_u8);
        assert_eq!(a_sizeunion.get_ubyte_(5), 104_u8);
        assert_eq!(a_sizeunion.get_ubyte_(6), 163_u8);
        assert_eq!(a_sizeunion.get_ubyte_(7), 189_u8);
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_ubyte()
{
    println!("unions_get_ubyte()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let a_shortunion = ShortUnion::new_with(2895_u16);
    for i in 0..2
    {
        match a_shortunion.get_ubyte(i)
        {
            Some(a) => { println!("a_shortunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_shortunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_shortunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_shortunion.get_ubyte(2), None);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..4
    {
        match a_intunion.get_ubyte(i)
        {
            Some(a) => { println!("a_intunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_intunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_intunion.get_ubyte(2), Some(74_u8));
    assert_eq!(a_intunion.get_ubyte(3), Some(241_u8));
    assert_eq!(a_intunion.get_ubyte(4), None);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..8
    {
        match a_longunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_longunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_longunion.get_ubyte(2), Some(74_u8));
    assert_eq!(a_longunion.get_ubyte(3), Some(241_u8));
    assert_eq!(a_longunion.get_ubyte(4), Some(245_u8));
    assert_eq!(a_longunion.get_ubyte(5), Some(104_u8));
    assert_eq!(a_longunion.get_ubyte(6), Some(163_u8));
    assert_eq!(a_longunion.get_ubyte(7), Some(189_u8));
    assert_eq!(a_longunion.get_ubyte(8), None);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..16
    {
        match a_longerunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longerunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_longerunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_longerunion.get_ubyte(2), Some(74_u8));
    assert_eq!(a_longerunion.get_ubyte(3), Some(241_u8));
    assert_eq!(a_longerunion.get_ubyte(4), Some(245_u8));
    assert_eq!(a_longerunion.get_ubyte(5), Some(104_u8));
    assert_eq!(a_longerunion.get_ubyte(6), Some(163_u8));
    assert_eq!(a_longerunion.get_ubyte(7), Some(189_u8));
    assert_eq!(a_longerunion.get_ubyte(8), Some(88_u8));
    assert_eq!(a_longerunion.get_ubyte(9), Some(136_u8));
    assert_eq!(a_longerunion.get_ubyte(10), Some(206_u8));
    assert_eq!(a_longerunion.get_ubyte(11), Some(126_u8));
    assert_eq!(a_longerunion.get_ubyte(12), Some(26_u8));
    assert_eq!(a_longerunion.get_ubyte(13), Some(59_u8));
    assert_eq!(a_longerunion.get_ubyte(14), Some(18_u8));
    assert_eq!(a_longerunion.get_ubyte(15), Some(255_u8));
    assert_eq!(a_longerunion.get_ubyte(16), None);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..8
        {
            match a_sizeunion.get_ubyte(i)
            {
                Some(a) => { println!("a_sizeunion.get_ubyte({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_ubyte(0), Some(79_u8));
        assert_eq!(a_sizeunion.get_ubyte(1), Some(11_u8));
        assert_eq!(a_sizeunion.get_ubyte(2), Some(74_u8));
        assert_eq!(a_sizeunion.get_ubyte(3), Some(241_u8));
        assert_eq!(a_sizeunion.get_ubyte(4), Some(245_u8));
        assert_eq!(a_sizeunion.get_ubyte(5), Some(104_u8));
        assert_eq!(a_sizeunion.get_ubyte(6), Some(163_u8));
        assert_eq!(a_sizeunion.get_ubyte(7), Some(189_u8));
        assert_eq!(a_sizeunion.get_ubyte(8), None);
    }
    println!("--------------------------------------");
}

fn unions_set_ubyte()
{
    println!("unions_set_ubyte()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let mut a_shortunion = ShortUnion::new();
    let b0 = a_shortunion.set_ubyte(0, 79_u8);
    let b1 = a_shortunion.set_ubyte(1, 11_u8);
    let b2 = a_shortunion.set_ubyte(2, 100_u8);  // Nothing will be done
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
    println!("a_shortunion.get() = {}", a_shortunion.get());
    for i in 0..2
    {
        match a_shortunion.get_ubyte(i)
        {
            Some(a) => { println!("a_shortunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_shortunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_shortunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_shortunion.get(), 2895_u16);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    let b0 = a_intunion.set_ubyte(0, 79_u8);
    let b1 = a_intunion.set_ubyte(1, 11_u8);
    let b2 = a_intunion.set_ubyte(2, 74_u8);
    let b3 = a_intunion.set_ubyte(3, 241_u8);
    let b4 = a_intunion.set_ubyte(4, 100_u8);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, false);
    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..4
    {
        match a_intunion.get_ubyte(i)
        {
            Some(a) => { println!("a_intunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_intunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_intunion.get_ubyte(2), Some(74_u8));
    assert_eq!(a_intunion.get_ubyte(3), Some(241_u8));
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    let b0 = a_longunion.set_ubyte(0, 79_u8);
    let b1 = a_longunion.set_ubyte(1, 11_u8);
    let b2 = a_longunion.set_ubyte(2, 74_u8);
    let b3 = a_longunion.set_ubyte(3, 241_u8);
    let b4 = a_longunion.set_ubyte(4, 245_u8);
    let b5 = a_longunion.set_ubyte(5, 104_u8);
    let b6 = a_longunion.set_ubyte(6, 163_u8);
    let b7 = a_longunion.set_ubyte(7, 189_u8);
    let b8 = a_longunion.set_ubyte(8, 100_u8);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, true);
    assert_eq!(b5, true);
    assert_eq!(b6, true);
    assert_eq!(b7, true);
    assert_eq!(b8, false);
    println!("a_longunion.get() = {}", a_longunion.get());
    for i in 0..8
    {
        match a_longunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_longunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_longunion.get_ubyte(2), Some(74_u8));
    assert_eq!(a_longunion.get_ubyte(3), Some(241_u8));
    assert_eq!(a_longunion.get_ubyte(4), Some(245_u8));
    assert_eq!(a_longunion.get_ubyte(5), Some(104_u8));
    assert_eq!(a_longunion.get_ubyte(6), Some(163_u8));
    assert_eq!(a_longunion.get_ubyte(7), Some(189_u8));
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_ubyte(0, 79_u8);
    let b1 = a_longerunion.set_ubyte(1, 11_u8);
    let b2 = a_longerunion.set_ubyte(2, 74_u8);
    let b3 = a_longerunion.set_ubyte(3, 241_u8);
    let b4 = a_longerunion.set_ubyte(4, 245_u8);
    let b5 = a_longerunion.set_ubyte(5, 104_u8);
    let b6 = a_longerunion.set_ubyte(6, 163_u8);
    let b7 = a_longerunion.set_ubyte(7, 189_u8);
    let b8 = a_longerunion.set_ubyte(8, 88_u8);
    let b9 = a_longerunion.set_ubyte(9, 136_u8);
    let b10 = a_longerunion.set_ubyte(10, 206_u8);
    let b11 = a_longerunion.set_ubyte(11, 126_u8);
    let b12 = a_longerunion.set_ubyte(12, 26_u8);
    let b13 = a_longerunion.set_ubyte(13, 59_u8);
    let b14 = a_longerunion.set_ubyte(14, 18_u8);
    let b15 = a_longerunion.set_ubyte(15, 255_u8);
    let b16 = a_longerunion.set_ubyte(16, 100_u8);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, true);
    assert_eq!(b5, true);
    assert_eq!(b6, true);
    assert_eq!(b7, true);
    assert_eq!(b8, true);
    assert_eq!(b9, true);
    assert_eq!(b10, true);
    assert_eq!(b11, true);
    assert_eq!(b12, true);
    assert_eq!(b13, true);
    assert_eq!(b14, true);
    assert_eq!(b15, true);
    assert_eq!(b16, false);
    println!("a_longerunion.get() = {}", a_longerunion.get());
    for i in 0..16
    {
        match a_longerunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longerunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_ubyte(0), Some(79_u8));
    assert_eq!(a_longerunion.get_ubyte(1), Some(11_u8));
    assert_eq!(a_longerunion.get_ubyte(2), Some(74_u8));
    assert_eq!(a_longerunion.get_ubyte(3), Some(241_u8));
    assert_eq!(a_longerunion.get_ubyte(4), Some(245_u8));
    assert_eq!(a_longerunion.get_ubyte(5), Some(104_u8));
    assert_eq!(a_longerunion.get_ubyte(6), Some(163_u8));
    assert_eq!(a_longerunion.get_ubyte(7), Some(189_u8));
    assert_eq!(a_longerunion.get_ubyte(8), Some(88_u8));
    assert_eq!(a_longerunion.get_ubyte(9), Some(136_u8));
    assert_eq!(a_longerunion.get_ubyte(10), Some(206_u8));
    assert_eq!(a_longerunion.get_ubyte(11), Some(126_u8));
    assert_eq!(a_longerunion.get_ubyte(12), Some(26_u8));
    assert_eq!(a_longerunion.get_ubyte(13), Some(59_u8));
    assert_eq!(a_longerunion.get_ubyte(14), Some(18_u8));
    assert_eq!(a_longerunion.get_ubyte(15), Some(255_u8));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        let b0 = a_sizeunion.set_ubyte(0, 79_u8);
        let b1 = a_sizeunion.set_ubyte(1, 11_u8);
        let b2 = a_sizeunion.set_ubyte(2, 74_u8);
        let b3 = a_sizeunion.set_ubyte(3, 241_u8);
        let b4 = a_sizeunion.set_ubyte(4, 245_u8);
        let b5 = a_sizeunion.set_ubyte(5, 104_u8);
        let b6 = a_sizeunion.set_ubyte(6, 163_u8);
        let b7 = a_sizeunion.set_ubyte(7, 189_u8);
        let b8 = a_sizeunion.set_ubyte(8, 100_u8);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, false);
        println!("a_sizeunion.get() = {}", a_sizeunion.get());
        for i in 0..8
        {
            match a_sizeunion.get_ubyte(i)
            {
                Some(a) => { println!("a_sizeunion.get_ubyte({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_ubyte(0), Some(79_u8));
        assert_eq!(a_sizeunion.get_ubyte(1), Some(11_u8));
        assert_eq!(a_sizeunion.get_ubyte(2), Some(74_u8));
        assert_eq!(a_sizeunion.get_ubyte(3), Some(241_u8));
        assert_eq!(a_sizeunion.get_ubyte(4), Some(245_u8));
        assert_eq!(a_sizeunion.get_ubyte(5), Some(104_u8));
        assert_eq!(a_sizeunion.get_ubyte(6), Some(163_u8));
        assert_eq!(a_sizeunion.get_ubyte(7), Some(189_u8));
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_sbyte_()
{
    println!("unions_get_sbyte_()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let a_shortunion = ShortUnion::new_with(2895_u16);
    for i in 0..2
        { println!("a_shortunion.get_sbyte_({}) = {}", i, a_shortunion.get_sbyte_(i)); }
    assert_eq!(a_shortunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_shortunion.get_sbyte_(1), 11_i8);

    // It will panic.
    // println!("a_shortunion.get_sbyte_(2) = {}", a_shortunion.get_sbyte_(2));

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..4
        { println!("a_intunion.get_sbyte_({}) = {}", i, a_intunion.get_sbyte_(i)); }
    assert_eq!(a_intunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_intunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_intunion.get_sbyte_(2), 74_i8);
    assert_eq!(a_intunion.get_sbyte_(3), -15_i8);

    // It will panic.
    // println!("a_intunion.get_sbyte_(4) = {}", a_intunion.get_sbyte_(4));

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..8
        { println!("a_longunion.get_sbyte_({}) = {}", i, a_longunion.get_sbyte_(i)); }
    assert_eq!(a_longunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_longunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_longunion.get_sbyte_(2), 74_i8);
    assert_eq!(a_longunion.get_sbyte_(3), -15_i8);
    assert_eq!(a_longunion.get_sbyte_(4), -11_i8);
    assert_eq!(a_longunion.get_sbyte_(5), 104_i8);
    assert_eq!(a_longunion.get_sbyte_(6), -93_i8);
    assert_eq!(a_longunion.get_sbyte_(7), -67_i8);

    // It will panic.
    // println!("a_longunion.get_sbyte_(8) = {}", a_longunion.get_sbyte_(8));

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..16
        { println!("a_longerunion.get_sbyte_({}) = {}", i, a_longerunion.get_sbyte_(i)); }
    assert_eq!(a_longerunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_longerunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_longerunion.get_sbyte_(2), 74_i8);
    assert_eq!(a_longerunion.get_sbyte_(3), -15_i8);
    assert_eq!(a_longerunion.get_sbyte_(4), -11_i8);
    assert_eq!(a_longerunion.get_sbyte_(5), 104_i8);
    assert_eq!(a_longerunion.get_sbyte_(6), -93_i8);
    assert_eq!(a_longerunion.get_sbyte_(7), -67_i8);
    assert_eq!(a_longerunion.get_sbyte_(8), 88_i8);
    assert_eq!(a_longerunion.get_sbyte_(9), -120_i8);
    assert_eq!(a_longerunion.get_sbyte_(10), -50_i8);
    assert_eq!(a_longerunion.get_sbyte_(11), 126_i8);
    assert_eq!(a_longerunion.get_sbyte_(12), 26_i8);
    assert_eq!(a_longerunion.get_sbyte_(13), 59_i8);
    assert_eq!(a_longerunion.get_sbyte_(14), 18_i8);
    assert_eq!(a_longerunion.get_sbyte_(15), -1_i8);

    // It will panic.
    // println!("a_longerunion.get_sbyte_(16) = {}", a_longerunion.get_sbyte_(16));

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..8
            { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        assert_eq!(a_sizeunion.get_sbyte_(2), 74_i8);
        assert_eq!(a_sizeunion.get_sbyte_(3), -15_i8);
        assert_eq!(a_sizeunion.get_sbyte_(4), -11_i8);
        assert_eq!(a_sizeunion.get_sbyte_(5), 104_i8);
        assert_eq!(a_sizeunion.get_sbyte_(6), -93_i8);
        assert_eq!(a_sizeunion.get_sbyte_(7), -67_i8);
    
        // It will panic.
        // println!("a_sizeunion.get_sbyte_(8) = {}", a_sizeunion.get_sbyte_(8));
    }
    println!("--------------------------------------");
}

fn unions_set_sbyte_()
{
    println!("unions_set_sbyte_()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let mut a_shortunion = ShortUnion::new();
    a_shortunion.set_sbyte_(0, 79_i8);
    a_shortunion.set_sbyte_(1, 11_i8);
    // It will panic.
    // a_shortunion.set_sbyte_(2, 100_i8);
    println!("a_shortunion.get() = {}", a_shortunion.get());
    for i in 0..2
        { println!("a_shortunion.get_sbyte_({}) = {}", i, a_shortunion.get_sbyte_(i)); }
    assert_eq!(a_shortunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_shortunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_shortunion.get(), 2895_u16);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    a_intunion.set_sbyte_(0, 79_i8);
    a_intunion.set_sbyte_(1, 11_i8);
    a_intunion.set_sbyte_(2, 74_i8);
    a_intunion.set_sbyte_(3, -15_i8);
    // It will panic.
    // a_intunion.set_sbyte_(4, 100_i8);
    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..4
        { println!("a_intunion.get_sbyte_({}) = {}", i, a_intunion.get_sbyte_(i)); }
    assert_eq!(a_intunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_intunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_intunion.get_sbyte_(2), 74_i8);
    assert_eq!(a_intunion.get_sbyte_(3), -15_i8);
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    a_longunion.set_sbyte_(0, 79_i8);
    a_longunion.set_sbyte_(1, 11_i8);
    a_longunion.set_sbyte_(2, 74_i8);
    a_longunion.set_sbyte_(3, -15_i8);
    a_longunion.set_sbyte_(4, -11_i8);
    a_longunion.set_sbyte_(5, 104_i8);
    a_longunion.set_sbyte_(6, -93_i8);
    a_longunion.set_sbyte_(7, -67_i8);
    // It will panic.
    // a_intunion.set_sbyte_(8, 100_i8);
    println!("a_longunion.get() = {}", a_longunion.get());
    for i in 0..8
        { println!("a_longunion.get_sbyte_({}) = {}", i, a_longunion.get_sbyte_(i)); }
    assert_eq!(a_longunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_longunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_longunion.get_sbyte_(2), 74_i8);
    assert_eq!(a_longunion.get_sbyte_(3), -15_i8);
    assert_eq!(a_longunion.get_sbyte_(4), -11_i8);
    assert_eq!(a_longunion.get_sbyte_(5), 104_i8);
    assert_eq!(a_longunion.get_sbyte_(6), -93_i8);
    assert_eq!(a_longunion.get_sbyte_(7), -67_i8);
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_sbyte_(0, 79_i8);
    a_longerunion.set_sbyte_(1, 11_i8);
    a_longerunion.set_sbyte_(2, 74_i8);
    a_longerunion.set_sbyte_(3, -15_i8);
    a_longerunion.set_sbyte_(4, -11_i8);
    a_longerunion.set_sbyte_(5, 104_i8);
    a_longerunion.set_sbyte_(6, -93_i8);
    a_longerunion.set_sbyte_(7, -67_i8);
    a_longerunion.set_sbyte_(8, 88_i8);
    a_longerunion.set_sbyte_(9, -120_i8);
    a_longerunion.set_sbyte_(10, -50_i8);
    a_longerunion.set_sbyte_(11, 126_i8);
    a_longerunion.set_sbyte_(12, 26_i8);
    a_longerunion.set_sbyte_(13, 59_i8);
    a_longerunion.set_sbyte_(14, 18_i8);
    a_longerunion.set_sbyte_(15, -1_i8);
    // It will panic.
    // a_longerunion.set_sbyte_(16, 100_i8);
    println!("a_longerunion.get() = {}", a_longerunion.get());
    for i in 0..16
        { println!("a_longerunion.get_sbyte_({}) = {}", i, a_longerunion.get_sbyte_(i)); }
    assert_eq!(a_longerunion.get_sbyte_(0), 79_i8);
    assert_eq!(a_longerunion.get_sbyte_(1), 11_i8);
    assert_eq!(a_longerunion.get_sbyte_(2), 74_i8);
    assert_eq!(a_longerunion.get_sbyte_(3), -15_i8);
    assert_eq!(a_longerunion.get_sbyte_(4), -11_i8);
    assert_eq!(a_longerunion.get_sbyte_(5), 104_i8);
    assert_eq!(a_longerunion.get_sbyte_(6), -93_i8);
    assert_eq!(a_longerunion.get_sbyte_(7), -67_i8);
    assert_eq!(a_longerunion.get_sbyte_(8), 88_i8);
    assert_eq!(a_longerunion.get_sbyte_(9), -120_i8);
    assert_eq!(a_longerunion.get_sbyte_(10), -50_i8);
    assert_eq!(a_longerunion.get_sbyte_(11), 126_i8);
    assert_eq!(a_longerunion.get_sbyte_(12), 26_i8);
    assert_eq!(a_longerunion.get_sbyte_(13), 59_i8);
    assert_eq!(a_longerunion.get_sbyte_(14), 18_i8);
    assert_eq!(a_longerunion.get_sbyte_(15), -1_i8);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        a_sizeunion.set_sbyte_(0, 79_i8);
        a_sizeunion.set_sbyte_(1, 11_i8);
        a_sizeunion.set_sbyte_(2, 74_i8);
        a_sizeunion.set_sbyte_(3, -15_i8);
        a_sizeunion.set_sbyte_(4, -11_i8);
        a_sizeunion.set_sbyte_(5, 104_i8);
        a_sizeunion.set_sbyte_(6, -93_i8);
        a_sizeunion.set_sbyte_(7, -67_i8);
        // It will panic.
        // a_sizeunion.set_sbyte_(8, 100_i8);
        println!("a_sizeunion.get() = {}", a_sizeunion.get());
        for i in 0..8
            { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        assert_eq!(a_sizeunion.get_sbyte_(2), 74_i8);
        assert_eq!(a_sizeunion.get_sbyte_(3), -15_i8);
        assert_eq!(a_sizeunion.get_sbyte_(4), -11_i8);
        assert_eq!(a_sizeunion.get_sbyte_(5), 104_i8);
        assert_eq!(a_sizeunion.get_sbyte_(6), -93_i8);
        assert_eq!(a_sizeunion.get_sbyte_(7), -67_i8);
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_sbyte()
{
    println!("unions_get_sbyte()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let a_shortunion = ShortUnion::new_with(2895_u16);
    for i in 0..2
    {
        match a_shortunion.get_sbyte(i)
        {
            Some(a) => { println!("a_shortunion.get_sbyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_shortunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_shortunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_shortunion.get_sbyte(2), None);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..4
    {
        match a_intunion.get_sbyte(i)
        {
            Some(a) => { println!("a_intunion.get_sbyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_intunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_intunion.get_sbyte(2), Some(74_i8));
    assert_eq!(a_intunion.get_sbyte(3), Some(-15_i8));
    assert_eq!(a_intunion.get_sbyte(4), None);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..8
    {
        match a_longunion.get_sbyte(i)
        {
            Some(a) => { println!("a_longunion.get_sbyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_longunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_longunion.get_sbyte(2), Some(74_i8));
    assert_eq!(a_longunion.get_sbyte(3), Some(-15_i8));
    assert_eq!(a_longunion.get_sbyte(4), Some(-11_i8));
    assert_eq!(a_longunion.get_sbyte(5), Some(104_i8));
    assert_eq!(a_longunion.get_sbyte(6), Some(-93_i8));
    assert_eq!(a_longunion.get_sbyte(7), Some(-67_i8));
    assert_eq!(a_longunion.get_sbyte(8), None);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..16
    {
        match a_longerunion.get_sbyte(i)
        {
            Some(a) => { println!("a_longerunion.get_sbyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_longerunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_longerunion.get_sbyte(2), Some(74_i8));
    assert_eq!(a_longerunion.get_sbyte(3), Some(-15_i8));
    assert_eq!(a_longerunion.get_sbyte(4), Some(-11_i8));
    assert_eq!(a_longerunion.get_sbyte(5), Some(104_i8));
    assert_eq!(a_longerunion.get_sbyte(6), Some(-93_i8));
    assert_eq!(a_longerunion.get_sbyte(7), Some(-67_i8));
    assert_eq!(a_longerunion.get_sbyte(8), Some(88_i8));
    assert_eq!(a_longerunion.get_sbyte(9), Some(-120_i8));
    assert_eq!(a_longerunion.get_sbyte(10), Some(-50_i8));
    assert_eq!(a_longerunion.get_sbyte(11), Some(126_i8));
    assert_eq!(a_longerunion.get_sbyte(12), Some(26_i8));
    assert_eq!(a_longerunion.get_sbyte(13), Some(59_i8));
    assert_eq!(a_longerunion.get_sbyte(14), Some(18_i8));
    assert_eq!(a_longerunion.get_sbyte(15), Some(-1_i8));
    assert_eq!(a_longerunion.get_sbyte(16), None);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..8
        {
            match a_sizeunion.get_sbyte(i)
            {
                Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        assert_eq!(a_sizeunion.get_sbyte(2), Some(74_i8));
        assert_eq!(a_sizeunion.get_sbyte(3), Some(-15_i8));
        assert_eq!(a_sizeunion.get_sbyte(4), Some(-11_i8));
        assert_eq!(a_sizeunion.get_sbyte(5), Some(104_i8));
        assert_eq!(a_sizeunion.get_sbyte(6), Some(-93_i8));
        assert_eq!(a_sizeunion.get_sbyte(7), Some(-67_i8));
        assert_eq!(a_sizeunion.get_sbyte(8), None);
    }
    println!("--------------------------------------");
}

fn unions_set_sbyte()
{
    println!("unions_set_sbyte()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    let mut a_shortunion = ShortUnion::new();
    let b0 = a_shortunion.set_sbyte(0, 79_i8);
    let b1 = a_shortunion.set_sbyte(1, 11_i8);
    let b2 = a_shortunion.set_sbyte(2, 100_i8);  // Nothing will be done
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
    println!("a_shortunion.get() = {}", a_shortunion.get());
    for i in 0..2
    {
        match a_shortunion.get_sbyte(i)
        {
            Some(a) => { println!("a_shortunion.get_sbyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_shortunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_shortunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_shortunion.get_sbyte(2), None);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    let b0 = a_intunion.set_sbyte(0, 79_i8);
    let b1 = a_intunion.set_sbyte(1, 11_i8);
    let b2 = a_intunion.set_sbyte(2, 74_i8);
    let b3 = a_intunion.set_sbyte(3, -15_i8);
    let b4 = a_intunion.set_sbyte(4, 100_i8);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, false);
    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..4
    {
        match a_intunion.get_sbyte(i)
        {
            Some(a) => { println!("a_intunion.get_sbyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_intunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_intunion.get_sbyte(2), Some(74_i8));
    assert_eq!(a_intunion.get_sbyte(3), Some(-15_i8));
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    let b0 = a_longunion.set_sbyte(0, 79_i8);
    let b1 = a_longunion.set_sbyte(1, 11_i8);
    let b2 = a_longunion.set_sbyte(2, 74_i8);
    let b3 = a_longunion.set_sbyte(3, -15_i8);
    let b4 = a_longunion.set_sbyte(4, -11_i8);
    let b5 = a_longunion.set_sbyte(5, 104_i8);
    let b6 = a_longunion.set_sbyte(6, -93_i8);
    let b7 = a_longunion.set_sbyte(7, -67_i8);
    let b8 = a_longunion.set_sbyte(8, 100_i8);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, true);
    assert_eq!(b5, true);
    assert_eq!(b6, true);
    assert_eq!(b7, true);
    assert_eq!(b8, false);
    println!("a_longunion.get() = {}", a_longunion.get());
    for i in 0..8
    {
        match a_longunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_longunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_longunion.get_sbyte(2), Some(74_i8));
    assert_eq!(a_longunion.get_sbyte(3), Some(-15_i8));
    assert_eq!(a_longunion.get_sbyte(4), Some(-11_i8));
    assert_eq!(a_longunion.get_sbyte(5), Some(104_i8));
    assert_eq!(a_longunion.get_sbyte(6), Some(-93_i8));
    assert_eq!(a_longunion.get_sbyte(7), Some(-67_i8));
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_sbyte(0, 79_i8);
    let b1 = a_longerunion.set_sbyte(1, 11_i8);
    let b2 = a_longerunion.set_sbyte(2, 74_i8);
    let b3 = a_longerunion.set_sbyte(3, -15_i8);
    let b4 = a_longerunion.set_sbyte(4, -11_i8);
    let b5 = a_longerunion.set_sbyte(5, 104_i8);
    let b6 = a_longerunion.set_sbyte(6, -93_i8);
    let b7 = a_longerunion.set_sbyte(7, -67_i8);
    let b8 = a_longerunion.set_sbyte(8, 88_i8);
    let b9 = a_longerunion.set_sbyte(9, -120_i8);
    let b10 = a_longerunion.set_sbyte(10, -50_i8);
    let b11 = a_longerunion.set_sbyte(11, 126_i8);
    let b12 = a_longerunion.set_sbyte(12, 26_i8);
    let b13 = a_longerunion.set_sbyte(13, 59_i8);
    let b14 = a_longerunion.set_sbyte(14, 18_i8);
    let b15 = a_longerunion.set_sbyte(15, -1_i8);
    let b16 = a_longerunion.set_sbyte(16, 100_i8);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, true);
    assert_eq!(b5, true);
    assert_eq!(b6, true);
    assert_eq!(b7, true);
    assert_eq!(b8, true);
    assert_eq!(b9, true);
    assert_eq!(b10, true);
    assert_eq!(b11, true);
    assert_eq!(b12, true);
    assert_eq!(b13, true);
    assert_eq!(b14, true);
    assert_eq!(b15, true);
    assert_eq!(b16, false);
    println!("a_longerunion.get() = {}", a_longerunion.get());
    for i in 0..16
    {
        match a_longerunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longerunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_sbyte(0), Some(79_i8));
    assert_eq!(a_longerunion.get_sbyte(1), Some(11_i8));
    assert_eq!(a_longerunion.get_sbyte(2), Some(74_i8));
    assert_eq!(a_longerunion.get_sbyte(3), Some(-15_i8));
    assert_eq!(a_longerunion.get_sbyte(4), Some(-11_i8));
    assert_eq!(a_longerunion.get_sbyte(5), Some(104_i8));
    assert_eq!(a_longerunion.get_sbyte(6), Some(-93_i8));
    assert_eq!(a_longerunion.get_sbyte(7), Some(-67_i8));
    assert_eq!(a_longerunion.get_sbyte(8), Some(88_i8));
    assert_eq!(a_longerunion.get_sbyte(9), Some(-120_i8));
    assert_eq!(a_longerunion.get_sbyte(10), Some(-50_i8));
    assert_eq!(a_longerunion.get_sbyte(11), Some(126_i8));
    assert_eq!(a_longerunion.get_sbyte(12), Some(26_i8));
    assert_eq!(a_longerunion.get_sbyte(13), Some(59_i8));
    assert_eq!(a_longerunion.get_sbyte(14), Some(18_i8));
    assert_eq!(a_longerunion.get_sbyte(15), Some(-1_i8));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        let b0 = a_sizeunion.set_sbyte(0, 79_i8);
        let b1 = a_sizeunion.set_sbyte(1, 11_i8);
        let b2 = a_sizeunion.set_sbyte(2, 74_i8);
        let b3 = a_sizeunion.set_sbyte(3, -15_i8);
        let b4 = a_sizeunion.set_sbyte(4, -11_i8);
        let b5 = a_sizeunion.set_sbyte(5, 104_i8);
        let b6 = a_sizeunion.set_sbyte(6, -93_i8);
        let b7 = a_sizeunion.set_sbyte(7, -67_i8);
        let b8 = a_sizeunion.set_sbyte(8, 100_i8);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, false);
        println!("a_sizeunion.get() = {}", a_sizeunion.get());
        for i in 0..8
        {
            match a_sizeunion.get_sbyte(i)
            {
                Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        assert_eq!(a_sizeunion.get_sbyte(2), Some(74_i8));
        assert_eq!(a_sizeunion.get_sbyte(3), Some(-15_i8));
        assert_eq!(a_sizeunion.get_sbyte(4), Some(-11_i8));
        assert_eq!(a_sizeunion.get_sbyte(5), Some(104_i8));
        assert_eq!(a_sizeunion.get_sbyte(6), Some(-93_i8));
        assert_eq!(a_sizeunion.get_sbyte(7), Some(-67_i8));
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}


fn unions_get_set_short_main()
{
    unions_get_ushort_();
    unions_set_ushort_();
    unions_get_sshort_();
    unions_set_sshort_();
    unions_get_ushort();
    unions_set_ushort();
    unions_get_sshort();
    unions_set_sshort();
}

fn unions_get_ushort_()
{
    println!("unions_get_ushort_()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..2
        { println!("a_intunion.get_ushort_({}) = {}", i, a_intunion.get_ushort_(i)); }
    assert_eq!(a_intunion.get_ushort_(0), 2895_u16);
    assert_eq!(a_intunion.get_ushort_(1), 61770_u16);

    // It will panic.
    // println!("a_intunion.get_ushort_(2) = {}", a_intunion.get_ushort_(2));

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..4
        { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
    assert_eq!(a_longunion.get_ushort_(0), 2895_u16);
    assert_eq!(a_longunion.get_ushort_(1), 61770_u16);
    assert_eq!(a_longunion.get_ushort_(2), 26869_u16);
    assert_eq!(a_longunion.get_ushort_(3), 48547_u16);

    // It will panic.
    // println!("a_longunion.get_ushort_(4) = {}", a_longunion.get_ushort_(4));

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..8
        { println!("a_longerunion.get_ushort_({}) = {}", i, a_longerunion.get_ushort_(i)); }
    assert_eq!(a_longerunion.get_ushort_(0), 2895_u16);
    assert_eq!(a_longerunion.get_ushort_(1), 61770_u16);
    assert_eq!(a_longerunion.get_ushort_(2), 26869_u16);
    assert_eq!(a_longerunion.get_ushort_(3), 48547_u16);
    assert_eq!(a_longerunion.get_ushort_(4), 34904_u16);
    assert_eq!(a_longerunion.get_ushort_(5), 32462_u16);
    assert_eq!(a_longerunion.get_ushort_(6), 15130_u16);
    assert_eq!(a_longerunion.get_ushort_(7), 65298_u16);

    // It will panic.
    // println!("a_longerunion.get_ushort_(8) = {}", a_longerunion.get_ushort_(8));

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..4
            { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        assert_eq!(a_sizeunion.get_ushort_(0), 2895_u16);
        assert_eq!(a_sizeunion.get_ushort_(1), 61770_u16);
        assert_eq!(a_sizeunion.get_ushort_(2), 26869_u16);
        assert_eq!(a_sizeunion.get_ushort_(3), 48547_u16);
    
        // It will panic.
        // println!("a_sizeunion.get_ushort_(4) = {}", a_sizeunion.get_ushort_(4));
    }
    println!("--------------------------------------");
}

fn unions_set_ushort_()
{
    println!("unions_set_ushort_()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    a_intunion.set_ushort_(0, 2895_u16);
    a_intunion.set_ushort_(1, 61770_u16);

    // It will panic.
    // a_intunion.set_ushort_(2, 100_u16);

    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..2
        { println!("a_intunion.get_ushort_({}) = {}", i, a_intunion.get_ushort_(i)); }
    assert_eq!(a_intunion.get_ushort_(0), 2895_u16);
    assert_eq!(a_intunion.get_ushort_(1), 61770_u16);
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    a_longunion.set_ushort_(0, 2895_u16);
    a_longunion.set_ushort_(1, 61770_u16);
    a_longunion.set_ushort_(2, 26869_u16);
    a_longunion.set_ushort_(3, 48547_u16);

    // It will panic.
    // a_longunion.set_ushort_(4, 100_u16);

    for i in 0..4
        { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
    assert_eq!(a_longunion.get_ushort_(0), 2895_u16);
    assert_eq!(a_longunion.get_ushort_(1), 61770_u16);
    assert_eq!(a_longunion.get_ushort_(2), 26869_u16);
    assert_eq!(a_longunion.get_ushort_(3), 48547_u16);
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_ushort_(0, 2895_u16);
    a_longerunion.set_ushort_(1, 61770_u16);
    a_longerunion.set_ushort_(2, 26869_u16);
    a_longerunion.set_ushort_(3, 48547_u16);
    a_longerunion.set_ushort_(4, 34904_u16);
    a_longerunion.set_ushort_(5, 32462_u16);
    a_longerunion.set_ushort_(6, 15130_u16);
    a_longerunion.set_ushort_(7, 65298_u16);

    // It will panic.
    // a_longerunion.set_ushort_(8, 100_u16);

    for i in 0..8
        { println!("a_longerunion.get_ushort_({}) = {}", i, a_longerunion.get_ushort_(i)); }
    assert_eq!(a_longerunion.get_ushort_(0), 2895_u16);
    assert_eq!(a_longerunion.get_ushort_(1), 61770_u16);
    assert_eq!(a_longerunion.get_ushort_(2), 26869_u16);
    assert_eq!(a_longerunion.get_ushort_(3), 48547_u16);
    assert_eq!(a_longerunion.get_ushort_(4), 34904_u16);
    assert_eq!(a_longerunion.get_ushort_(5), 32462_u16);
    assert_eq!(a_longerunion.get_ushort_(6), 15130_u16);
    assert_eq!(a_longerunion.get_ushort_(7), 65298_u16);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        a_sizeunion.set_ushort_(0, 2895_u16);
        a_sizeunion.set_ushort_(1, 61770_u16);
        a_sizeunion.set_ushort_(2, 26869_u16);
        a_sizeunion.set_ushort_(3, 48547_u16);

        // It will panic.
        // a_sizeunion.set_ushort_(4, 100_u16);

        for i in 0..4
            { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        assert_eq!(a_sizeunion.get_ushort_(0), 2895_u16);
        assert_eq!(a_sizeunion.get_ushort_(1), 61770_u16);
        assert_eq!(a_sizeunion.get_ushort_(2), 26869_u16);
        assert_eq!(a_sizeunion.get_ushort_(3), 48547_u16);
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_sshort_()
{
    println!("unions_get_sshort_()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..2
        { println!("a_intunion.get_sshort_({}) = {}", i, a_intunion.get_sshort_(i)); }
    assert_eq!(a_intunion.get_sshort_(0), 2895_i16);
    assert_eq!(a_intunion.get_sshort_(1), -3766_i16);

    // It will panic.
    // println!("a_intunion.get_sshort_(2) = {}", a_intunion.get_sshort_(2));

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..4
        { println!("a_longunion.get_sshort_({}) = {}", i, a_longunion.get_sshort_(i)); }
    assert_eq!(a_longunion.get_sshort_(0), 2895_i16);
    assert_eq!(a_longunion.get_sshort_(1), -3766_i16);
    assert_eq!(a_longunion.get_sshort_(2), 26869_i16);
    assert_eq!(a_longunion.get_sshort_(3), -16989_i16);

    // It will panic.
    // println!("a_longunion.get_sshort_(4) = {}", a_longunion.get_sshort_(4));

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..8
        { println!("a_longerunion.get_sshort_({}) = {}", i, a_longerunion.get_sshort_(i)); }
    assert_eq!(a_longerunion.get_sshort_(0), 2895_i16);
    assert_eq!(a_longerunion.get_sshort_(1), -3766_i16);
    assert_eq!(a_longerunion.get_sshort_(2), 26869_i16);
    assert_eq!(a_longerunion.get_sshort_(3), -16989_i16);
    assert_eq!(a_longerunion.get_sshort_(4), -30632_i16);
    assert_eq!(a_longerunion.get_sshort_(5), 32462_i16);
    assert_eq!(a_longerunion.get_sshort_(6), 15130_i16);
    assert_eq!(a_longerunion.get_sshort_(7), -238_i16);

    // It will panic.
    // println!("a_longerunion.get_sshort_(8) = {}", a_longerunion.get_sshort_(8));

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..4
            { println!("a_sizeunion.get_sshort_({}) = {}", i, a_sizeunion.get_sshort_(i)); }
        assert_eq!(a_sizeunion.get_sshort_(0), 2895_i16);
        assert_eq!(a_sizeunion.get_sshort_(1), -3766_i16);
        assert_eq!(a_sizeunion.get_sshort_(2), 26869_i16);
        assert_eq!(a_sizeunion.get_sshort_(3), -16989_i16);
    
        // It will panic.
        // println!("a_sizeunion.get_sshort_(4) = {}", a_sizeunion.get_sshort_(4));
    }
    println!("--------------------------------------");
}

fn unions_set_sshort_()
{
    println!("unions_set_sshort_()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    a_intunion.set_sshort_(0, 2895_i16);
    a_intunion.set_sshort_(1, -3766_i16);

    // It will panic.
    // a_intunion.set_sshort_(2, 100_i16);

    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..2
        { println!("a_intunion.get_sshort_({}) = {}", i, a_intunion.get_sshort_(i)); }
    assert_eq!(a_intunion.get_sshort_(0), 2895_i16);
    assert_eq!(a_intunion.get_sshort_(1), -3766_i16);
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    a_longunion.set_sshort_(0, 2895_i16);
    a_longunion.set_sshort_(1, -3766_i16);
    a_longunion.set_sshort_(2, 26869_i16);
    a_longunion.set_sshort_(3, -16989_i16);

    // It will panic.
    // a_longunion.set_sshort_(4, 100_i16);

    for i in 0..4
        { println!("a_longunion.get_sshort_({}) = {}", i, a_longunion.get_sshort_(i)); }
    assert_eq!(a_longunion.get_sshort_(0), 2895_i16);
    assert_eq!(a_longunion.get_sshort_(1), -3766_i16);
    assert_eq!(a_longunion.get_sshort_(2), 26869_i16);
    assert_eq!(a_longunion.get_sshort_(3), -16989_i16);
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_sshort_(0, 2895_i16);
    a_longerunion.set_sshort_(1, -3766_i16);
    a_longerunion.set_sshort_(2, 26869_i16);
    a_longerunion.set_sshort_(3, -16989_i16);
    a_longerunion.set_sshort_(4, -30632_i16);
    a_longerunion.set_sshort_(5, 32462_i16);
    a_longerunion.set_sshort_(6, 15130_i16);
    a_longerunion.set_sshort_(7, -238_i16);

    // It will panic.
    // a_longerunion.set_sshort_(8, 100_i16);

    for i in 0..8
        { println!("a_longerunion.get_sshort_({}) = {}", i, a_longerunion.get_sshort_(i)); }
    assert_eq!(a_longerunion.get_sshort_(0), 2895_i16);
    assert_eq!(a_longerunion.get_sshort_(1), -3766_i16);
    assert_eq!(a_longerunion.get_sshort_(2), 26869_i16);
    assert_eq!(a_longerunion.get_sshort_(3), -16989_i16);
    assert_eq!(a_longerunion.get_sshort_(4), -30632_i16);
    assert_eq!(a_longerunion.get_sshort_(5), 32462_i16);
    assert_eq!(a_longerunion.get_sshort_(6), 15130_i16);
    assert_eq!(a_longerunion.get_sshort_(7), -238_i16);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        a_sizeunion.set_sshort_(0, 2895_i16);
        a_sizeunion.set_sshort_(1, -3766_i16);
        a_sizeunion.set_sshort_(2, 26869_i16);
        a_sizeunion.set_sshort_(3, -16989_i16);

        // It will panic.
        // a_sizeunion.set_sshort_(4, 100_i16);

        for i in 0..4
            { println!("a_sizeunion.get_sshort_({}) = {}", i, a_sizeunion.get_sshort_(i)); }
        assert_eq!(a_sizeunion.get_sshort_(0), 2895_i16);
        assert_eq!(a_sizeunion.get_sshort_(1), -3766_i16);
        assert_eq!(a_sizeunion.get_sshort_(2), 26869_i16);
        assert_eq!(a_sizeunion.get_sshort_(3), -16989_i16);
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_ushort()
{
    println!("unions_get_ushort()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..2
    {
        match a_intunion.get_ushort(i)
        {
            Some(a) => { println!("a_intunion.get_ushort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_ushort(0), Some(2895_u16));
    assert_eq!(a_intunion.get_ushort(1), Some(61770_u16));
    assert_eq!(a_intunion.get_ushort(2), None);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..4
    {
        match a_longunion.get_ushort(i)
        {
            Some(a) => { println!("a_longunion.get_ushort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_ushort(0), Some(2895_u16));
    assert_eq!(a_longunion.get_ushort(1), Some(61770_u16));
    assert_eq!(a_longunion.get_ushort(2), Some(26869_u16));
    assert_eq!(a_longunion.get_ushort(3), Some(48547_u16));
    assert_eq!(a_longunion.get_ushort(4), None);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..8
    {
        match a_longerunion.get_ushort(i)
        {
            Some(a) => { println!("a_longerunion.get_ushort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_ushort(0), Some(2895_u16));
    assert_eq!(a_longerunion.get_ushort(1), Some(61770_u16));
    assert_eq!(a_longerunion.get_ushort(2), Some(26869_u16));
    assert_eq!(a_longerunion.get_ushort(3), Some(48547_u16));
    assert_eq!(a_longerunion.get_ushort(4), Some(34904_u16));
    assert_eq!(a_longerunion.get_ushort(5), Some(32462_u16));
    assert_eq!(a_longerunion.get_ushort(6), Some(15130_u16));
    assert_eq!(a_longerunion.get_ushort(7), Some(65298_u16));
    assert_eq!(a_longerunion.get_ushort(8), None);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..8
        {
            match a_sizeunion.get_ushort(i)
            {
                Some(a) => { println!("a_sizeunion.get_ushort({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_ushort(0), Some(2895_u16));
        assert_eq!(a_sizeunion.get_ushort(1), Some(61770_u16));
        assert_eq!(a_sizeunion.get_ushort(2), Some(26869_u16));
        assert_eq!(a_sizeunion.get_ushort(3), Some(48547_u16));
        assert_eq!(a_sizeunion.get_ushort(4), None);
    }
    println!("--------------------------------------");
}

fn unions_set_ushort()
{
    println!("unions_set_ushort()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    let b0 = a_intunion.set_ushort(0, 2895_u16);
    let b1 = a_intunion.set_ushort(1, 61770_u16);
    let b2 = a_intunion.set_ushort(2, 100_u16);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..2
    {
        match a_intunion.get_ushort(i)
        {
            Some(a) => { println!("a_intunion.get_ushort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_ushort(0), Some(2895_u16));
    assert_eq!(a_intunion.get_ushort(1), Some(61770_u16));
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    let b0 = a_longunion.set_ushort(0, 2895_u16);
    let b1 = a_longunion.set_ushort(1, 61770_u16);
    let b2 = a_longunion.set_ushort(2, 26869_u16);
    let b3 = a_longunion.set_ushort(3, 48547_u16);
    let b4 = a_longunion.set_ushort(4, 100_u16);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, false);
    println!("a_longunion.get() = {}", a_longunion.get());
    for i in 0..4
    {
        match a_longunion.get_ubyte(i)
        {
            Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_ushort(0), Some(2895_u16));
    assert_eq!(a_longunion.get_ushort(1), Some(61770_u16));
    assert_eq!(a_longunion.get_ushort(2), Some(26869_u16));
    assert_eq!(a_longunion.get_ushort(3), Some(48547_u16));
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_ushort(0, 2895_u16);
    let b1 = a_longerunion.set_ushort(1, 61770_u16);
    let b2 = a_longerunion.set_ushort(2, 26869_u16);
    let b3 = a_longerunion.set_ushort(3, 48547_u16);
    let b4 = a_longerunion.set_ushort(4, 34904_u16);
    let b5 = a_longerunion.set_ushort(5, 32462_u16);
    let b6 = a_longerunion.set_ushort(6, 15130_u16);
    let b7 = a_longerunion.set_ushort(7, 65298_u16);
    let b8 = a_longerunion.set_ushort(8, 100_u16);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, true);
    assert_eq!(b5, true);
    assert_eq!(b6, true);
    assert_eq!(b7, true);
    assert_eq!(b8, false);
    println!("a_longerunion.get() = {}", a_longerunion.get());
    for i in 0..8
    {
        match a_longerunion.get_ushort(i)
        {
            Some(a) => { println!("a_longerunion.get_ushort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_ushort(0), Some(2895_u16));
    assert_eq!(a_longerunion.get_ushort(1), Some(61770_u16));
    assert_eq!(a_longerunion.get_ushort(2), Some(26869_u16));
    assert_eq!(a_longerunion.get_ushort(3), Some(48547_u16));
    assert_eq!(a_longerunion.get_ushort(4), Some(34904_u16));
    assert_eq!(a_longerunion.get_ushort(5), Some(32462_u16));
    assert_eq!(a_longerunion.get_ushort(6), Some(15130_u16));
    assert_eq!(a_longerunion.get_ushort(7), Some(65298_u16));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        let b0 = a_sizeunion.set_ushort(0, 2895_u16);
        let b1 = a_sizeunion.set_ushort(1, 61770_u16);
        let b2 = a_sizeunion.set_ushort(2, 26869_u16);
        let b3 = a_sizeunion.set_ushort(3, 48547_u16);
        let b4 = a_sizeunion.set_ushort(4, 100_u16);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);
        println!("a_sizeunion.get() = {}", a_sizeunion.get());
        for i in 0..4
        {
            match a_sizeunion.get_ushort(i)
            {
                Some(a) => { println!("a_sizeunion.get_ushort({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_ushort(0), Some(2895_u16));
        assert_eq!(a_sizeunion.get_ushort(1), Some(61770_u16));
        assert_eq!(a_sizeunion.get_ushort(2), Some(26869_u16));
        assert_eq!(a_sizeunion.get_ushort(3), Some(48547_u16));
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_sshort()
{
    println!("unions_get_sshort()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let a_intunion = IntUnion::new_with(4048161615_u32);
    for i in 0..2
    {
        match a_intunion.get_sshort(i)
        {
            Some(a) => { println!("a_intunion.get_sshort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_sshort(0), Some(2895_i16));
    assert_eq!(a_intunion.get_sshort(1), Some(-3766_i16));
    assert_eq!(a_intunion.get_sshort(2), None);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..4
    {
        match a_longunion.get_sshort(i)
        {
            Some(a) => { println!("a_longunion.get_sshort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_sshort(0), Some(2895_i16));
    assert_eq!(a_longunion.get_sshort(1), Some(-3766_i16));
    assert_eq!(a_longunion.get_sshort(2), Some(26869_i16));
    assert_eq!(a_longunion.get_sshort(3), Some(-16989_i16));
    assert_eq!(a_longunion.get_sshort(4), None);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..8
    {
        match a_longerunion.get_sshort(i)
        {
            Some(a) => { println!("a_longerunion.get_sshort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_sshort(0), Some(2895_i16));
    assert_eq!(a_longerunion.get_sshort(1), Some(-3766_i16));
    assert_eq!(a_longerunion.get_sshort(2), Some(26869_i16));
    assert_eq!(a_longerunion.get_sshort(3), Some(-16989_i16));
    assert_eq!(a_longerunion.get_sshort(4), Some(-30632_i16));
    assert_eq!(a_longerunion.get_sshort(5), Some(32462_i16));
    assert_eq!(a_longerunion.get_sshort(6), Some(15130_i16));
    assert_eq!(a_longerunion.get_sshort(7), Some(-238_i16));
    assert_eq!(a_longerunion.get_sshort(8), None);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..8
        {
            match a_sizeunion.get_sshort(i)
            {
                Some(a) => { println!("a_sizeunion.get_sshort({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_sshort(0), Some(2895_i16));
        assert_eq!(a_sizeunion.get_sshort(1), Some(-3766_i16));
        assert_eq!(a_sizeunion.get_sshort(2), Some(26869_i16));
        assert_eq!(a_sizeunion.get_sshort(3), Some(-16989_i16));
        assert_eq!(a_sizeunion.get_sshort(4), None);
    }
    println!("--------------------------------------");
}

fn unions_set_sshort()
{
    println!("unions_set_sshort()");
    // Example for IntUnion
    use cryptocol::number::IntUnion;
    let mut a_intunion = IntUnion::new();
    let b0 = a_intunion.set_sshort(0, 2895_i16);
    let b1 = a_intunion.set_sshort(1, -3766_i16);
    let b2 = a_intunion.set_sshort(2, 100_i16);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
    println!("a_intunion.get() = {}", a_intunion.get());
    for i in 0..2
    {
        match a_intunion.get_sshort(i)
        {
            Some(a) => { println!("a_intunion.set_sshort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_intunion.get_sshort(0), Some(2895_i16));
    assert_eq!(a_intunion.get_sshort(1), Some(-3766_i16));
    assert_eq!(a_intunion.get(), 4048161615_u32);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    let b0 = a_longunion.set_sshort(0, 2895_i16);
    let b1 = a_longunion.set_sshort(1, -3766_i16);
    let b2 = a_longunion.set_sshort(2, 26869_i16);
    let b3 = a_longunion.set_sshort(3, -16989_i16);
    let b4 = a_longunion.set_sshort(4, 100_i16);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, false);
    println!("a_longunion.get() = {}", a_longunion.get());
    for i in 0..4
    {
        match a_longunion.get_sshort(i)
        {
            Some(a) => { println!("a_longunion.get_sshort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_sshort(0), Some(2895_i16));
    assert_eq!(a_longunion.get_sshort(1), Some(-3766_i16));
    assert_eq!(a_longunion.get_sshort(2), Some(26869_i16));
    assert_eq!(a_longunion.get_sshort(3), Some(-16989_i16));
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_sshort(0, 2895_i16);
    let b1 = a_longerunion.set_sshort(1, -3766_i16);
    let b2 = a_longerunion.set_sshort(2, 26869_i16);
    let b3 = a_longerunion.set_sshort(3, -16989_i16);
    let b4 = a_longerunion.set_sshort(4, -30632_i16);
    let b5 = a_longerunion.set_sshort(5, 32462_i16);
    let b6 = a_longerunion.set_sshort(6, 15130_i16);
    let b7 = a_longerunion.set_sshort(7, -238_i16);
    let b8 = a_longerunion.set_sshort(8, 100_i16);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, true);
    assert_eq!(b5, true);
    assert_eq!(b6, true);
    assert_eq!(b7, true);
    assert_eq!(b8, false);
    println!("a_longerunion.get() = {}", a_longerunion.get());
    for i in 0..8
    {
        match a_longerunion.get_ushort(i)
        {
            Some(a) => { println!("a_longerunion.get_ushort({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_sshort(0), Some(2895_i16));
    assert_eq!(a_longerunion.get_sshort(1), Some(-3766_i16));
    assert_eq!(a_longerunion.get_sshort(2), Some(26869_i16));
    assert_eq!(a_longerunion.get_sshort(3), Some(-16989_i16));
    assert_eq!(a_longerunion.get_sshort(4), Some(-30632_i16));
    assert_eq!(a_longerunion.get_sshort(5), Some(32462_i16));
    assert_eq!(a_longerunion.get_sshort(6), Some(15130_i16));
    assert_eq!(a_longerunion.get_sshort(7), Some(-238_i16));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        let b0 = a_sizeunion.set_sshort(0, 2895_i16);
        let b1 = a_sizeunion.set_sshort(1, -3766_i16);
        let b2 = a_sizeunion.set_sshort(2, 26869_i16);
        let b3 = a_sizeunion.set_sshort(3, -16989_i16);
        let b4 = a_sizeunion.set_sshort(4, 100_i16);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);
        println!("a_sizeunion.get() = {}", a_sizeunion.get());
        for i in 0..4
        {
            match a_sizeunion.get_sshort(i)
            {
                Some(a) => { println!("a_sizeunion.get_sshort({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_sshort(0), Some(2895_i16));
        assert_eq!(a_sizeunion.get_sshort(1), Some(-3766_i16));
        assert_eq!(a_sizeunion.get_sshort(2), Some(26869_i16));
        assert_eq!(a_sizeunion.get_sshort(3), Some(-16989_i16));
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}


fn unions_get_set_int_main()
{
    unions_get_uint_();
    unions_set_uint_();
    unions_get_sint_();
    unions_set_sint_();
    unions_get_uint();
    unions_set_uint();
    unions_get_sint();
    unions_set_sint();
}

fn unions_get_uint_()
{
    println!("unions_get_uint_()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..2
        { println!("a_longunion.get_uint_({}) = {}", i, a_longunion.get_uint_(i)); }
    assert_eq!(a_longunion.get_uint_(0), 4048161615_u32);
    assert_eq!(a_longunion.get_uint_(1), 3181603061_u32);

    // It will panic.
    // println!("a_longunion.get_uint_(2) = {}", a_longunion.get_uint_(2));

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..4
        { println!("a_longerunion.get_uint_({}) = {}", i, a_longerunion.get_uint_(i)); }
    assert_eq!(a_longerunion.get_uint_(0), 4048161615_u32);
    assert_eq!(a_longerunion.get_uint_(1), 3181603061_u32);
    assert_eq!(a_longerunion.get_uint_(2), 2127464536_u32);
    assert_eq!(a_longerunion.get_uint_(3), 4279384858_u32);

    // It will panic.
    // println!("a_longerunion.get_uint_(4) = {}", a_longerunion.get_uint_(4));

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..2
            { println!("a_sizeunion.get_uint_({}) = {}", i, a_sizeunion.get_uint_(i)); }
        assert_eq!(a_sizeunion.get_uint_(0), 4048161615_u32);
        assert_eq!(a_sizeunion.get_uint_(1), 3181603061_u32);

        // It will panic.
        // println!("a_sizeunion.get_uint_(2) = {}", a_sizeunion.get_uint_(2));
    }
    println!("--------------------------------------");
}

fn unions_set_uint_()
{
    println!("unions_set_uint_()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    a_longunion.set_uint_(0, 4048161615_u32);
    a_longunion.set_uint_(1, 3181603061_u32);

    // It will panic.
    // a_longunion.set_ushort_(2, 100_u32);

    for i in 0..2
        { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
    assert_eq!(a_longunion.get_uint_(0), 4048161615_u32);
    assert_eq!(a_longunion.get_uint_(1), 3181603061_u32);
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_uint_(0, 4048161615_u32);
    a_longerunion.set_uint_(1, 3181603061_u32);
    a_longerunion.set_uint_(2, 2127464536_u32);
    a_longerunion.set_uint_(3, 4279384858_u32);

    // It will panic.
    // a_longerunion.set_uint_(4, 100_u32);

    for i in 0..4
        { println!("a_longerunion.get_uint_({}) = {}", i, a_longerunion.get_uint_(i)); }
    assert_eq!(a_longerunion.get_uint_(0), 4048161615_u32);
    assert_eq!(a_longerunion.get_uint_(1), 3181603061_u32);
    assert_eq!(a_longerunion.get_uint_(2), 2127464536_u32);
    assert_eq!(a_longerunion.get_uint_(3), 4279384858_u32);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        a_sizeunion.set_uint_(0, 4048161615_u32);
        a_sizeunion.set_uint_(1, 3181603061_u32);
    
        // It will panic.
        // a_sizeunion.set_ushort_(2, 100_u32);
    
        for i in 0..2
            { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        assert_eq!(a_sizeunion.get_uint_(0), 4048161615_u32);
        assert_eq!(a_sizeunion.get_uint_(1), 3181603061_u32);
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_sint_()
{
    println!("unions_get_sint_()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..2
        { println!("a_longunion.get_sint_({}) = {}", i, a_longunion.get_sint_(i)); }
    assert_eq!(a_longunion.get_sint_(0), -246805681_i32);
    assert_eq!(a_longunion.get_sint_(1), -1113364235_i32);

    // It will panic.
    // println!("a_longunion.get_sint_(2) = {}", a_longunion.get_sint_(2));

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..4
        { println!("a_longerunion.get_sint_({}) = {}", i, a_longerunion.get_sint_(i)); }
    assert_eq!(a_longerunion.get_sint_(0), -246805681_i32);
    assert_eq!(a_longerunion.get_sint_(1), -1113364235_i32);
    assert_eq!(a_longerunion.get_sint_(2), 2127464536_i32);
    assert_eq!(a_longerunion.get_sint_(3), -15582438_i32);

    // It will panic.
    // println!("a_longerunion.get_sint_(4) = {}", a_longerunion.get_sint_(4));

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..2
            { println!("a_sizeunion.get_sint_({}) = {}", i, a_longunion.get_sint_(i)); }
        assert_eq!(a_sizeunion.get_sint_(0), -246805681_i32);
        assert_eq!(a_sizeunion.get_sint_(1), -1113364235_i32);

    // It will panic.
    // println!("a_sizeunion.get_sint_(2) = {}", a_sizeunion.get_sint_(2));
    }
    println!("--------------------------------------");
}

fn unions_set_sint_()
{
    println!("unions_set_sint_()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    a_longunion.set_sint_(0, -246805681_i32);
    a_longunion.set_sint_(1, -1113364235_i32);

    // It will panic.
    // a_longunion.set_sint_(2, 100_i32);

    for i in 0..2
        { println!("a_longunion.get_sint_({}) = {}", i, a_longunion.get_sint_(i)); }
    assert_eq!(a_longunion.get_sint_(0), -246805681_i32);
    assert_eq!(a_longunion.get_sint_(1), -1113364235_i32);
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_sint_(0, -246805681_i32);
    a_longerunion.set_sint_(1, -1113364235_i32);
    a_longerunion.set_sint_(2, 2127464536_i32);
    a_longerunion.set_sint_(3, -15582438_i32);

    // It will panic.
    // a_longerunion.set_uint_(4, 100_u32);

    for i in 0..4
        { println!("a_longerunion.get_uint_({}) = {}", i, a_longerunion.get_uint_(i)); }
    assert_eq!(a_longerunion.get_sint_(0), -246805681_i32);
    assert_eq!(a_longerunion.get_sint_(1), -1113364235_i32);
    assert_eq!(a_longerunion.get_sint_(2), 2127464536_i32);
    assert_eq!(a_longerunion.get_sint_(3), -15582438_i32);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        a_sizeunion.set_sint_(0, -246805681_i32);
        a_sizeunion.set_sint_(1, -1113364235_i32);
    
        // It will panic.
        // a_sizeunion.set_sint_(2, 100_i32);
    
        for i in 0..2
            { println!("a_longunion.get_sint_({}) = {}", i, a_longunion.get_sint_(i)); }
        assert_eq!(a_sizeunion.get_sint_(0), -246805681_i32);
        assert_eq!(a_sizeunion.get_sint_(1), -1113364235_i32);
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_uint()
{
    println!("unions_get_uint()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..2
    {
        match a_longunion.get_uint(i)
        {
            Some(a) => { println!("a_longunion.get_uint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_uint(0), Some(4048161615_u32));
    assert_eq!(a_longunion.get_uint(1), Some(3181603061_u32));
    assert_eq!(a_longunion.get_uint(2), None);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..4
    {
        match a_longerunion.get_uint(i)
        {
            Some(a) => { println!("a_longerunion.get_uint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_uint(0), Some(4048161615_u32));
    assert_eq!(a_longerunion.get_uint(1), Some(3181603061_u32));
    assert_eq!(a_longerunion.get_uint(2), Some(2127464536_u32));
    assert_eq!(a_longerunion.get_uint(3), Some(4279384858_u32));
    assert_eq!(a_longerunion.get_uint(4), None);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..2
        {
            match a_sizeunion.get_uint(i)
            {
                Some(a) => { println!("a_sizeunion.get_uint({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_uint(0), Some(4048161615_u32));
        assert_eq!(a_sizeunion.get_uint(1), Some(3181603061_u32));
        assert_eq!(a_sizeunion.get_uint(2), None);
    }
    println!("--------------------------------------");
}

fn unions_set_uint()
{
    println!("unions_set_uint()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    let b0 = a_longunion.set_uint(0, 4048161615_u32);
    let b1 = a_longunion.set_uint(1, 3181603061_u32);
    let b2 = a_longunion.set_uint(2, 100_u32);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
    for i in 0..2
    {
        match a_longunion.get_uint(i)
        {
            Some(a) => { println!("a_longunion.get_uint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_uint(0), Some(4048161615_u32));
    assert_eq!(a_longunion.get_uint(1), Some(3181603061_u32));
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_uint(0, 4048161615_u32);
    let b1 = a_longerunion.set_uint(1, 3181603061_u32);
    let b2 = a_longerunion.set_uint(2, 2127464536_u32);
    let b3 = a_longerunion.set_uint(3, 4279384858_u32);
    let b4 = a_longerunion.set_uint(4, 100_u32);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, false);

    for i in 0..4
    {
        match a_longerunion.get_uint(i)
        {
            Some(a) => { println!("a_longerunion.get_uint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_uint(0), Some(4048161615_u32));
    assert_eq!(a_longerunion.get_uint(1), Some(3181603061_u32));
    assert_eq!(a_longerunion.get_uint(2), Some(2127464536_u32));
    assert_eq!(a_longerunion.get_uint(3), Some(4279384858_u32));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        let b0 = a_sizeunion.set_uint(0, 4048161615_u32);
        let b1 = a_sizeunion.set_uint(1, 3181603061_u32);
        let b2 = a_sizeunion.set_uint(2, 100_u32);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        for i in 0..2
        {
            match a_sizeunion.get_uint(i)
            {
                Some(a) => { println!("a_sizeunion.get_uint({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_uint(0), Some(4048161615_u32));
        assert_eq!(a_sizeunion.get_uint(1), Some(3181603061_u32));
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}

fn unions_get_sint()
{
    println!("unions_get_sint()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let a_longunion = LongUnion::new_with(13664881099896654671_u64);
    for i in 0..2
    {
        match a_longunion.get_sint(i)
        {
            Some(a) => { println!("a_longunion.get_sint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_sint(0), Some(-246805681_i32));
    assert_eq!(a_longunion.get_sint(1), Some(-1113364235_i32));
    assert_eq!(a_longunion.get_sint(2), None);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..4
    {
        match a_longerunion.get_sint(i)
        {
            Some(a) => { println!("a_longerunion.get_sint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_sint(0), Some(-246805681_i32));
    assert_eq!(a_longerunion.get_sint(1), Some(-1113364235_i32));
    assert_eq!(a_longerunion.get_sint(2), Some(2127464536_i32));
    assert_eq!(a_longerunion.get_sint(3), Some(-15582438_i32));
    assert_eq!(a_longerunion.get_sint(4), None);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        for i in 0..2
        {
            match a_sizeunion.get_sint(i)
            {
                Some(a) => { println!("a_sizeunion.get_sint({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_sint(0), Some(-246805681_i32));
        assert_eq!(a_sizeunion.get_sint(1), Some(-1113364235_i32));
        assert_eq!(a_sizeunion.get_sint(2), None);
    }
    println!("--------------------------------------");
}

fn unions_set_sint()
{
    println!("unions_set_sint()");
    // Example for LongUnion
    use cryptocol::number::LongUnion;
    let mut a_longunion = LongUnion::new();
    let b0 = a_longunion.set_sint(0, -246805681_i32);
    let b1 = a_longunion.set_sint(1, -1113364235_i32);
    let b2 = a_longunion.set_sint(2, 100_i32);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
    for i in 0..2
    {
        match a_longunion.get_sint(i)
        {
            Some(a) => { println!("a_longunion.set_sint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longunion.get_sint(0), Some(-246805681_i32));
    assert_eq!(a_longunion.get_sint(1), Some(-1113364235_i32));
    assert_eq!(a_longunion.get(), 13664881099896654671_u64);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_sint(0, -246805681_i32);
    let b1 = a_longerunion.set_sint(1, -1113364235_i32);
    let b2 = a_longerunion.set_sint(2, 2127464536_i32);
    let b3 = a_longerunion.set_sint(3, -15582438_i32);
    let b4 = a_longerunion.set_sint(4, 100_i32);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, true);
    assert_eq!(b3, true);
    assert_eq!(b4, false);

    for i in 0..4
    {
        match a_longerunion.get_uint(i)
        {
            Some(a) => { println!("a_longerunion.get_uint({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_sint(0), Some(-246805681_i32));
    assert_eq!(a_longerunion.get_sint(1), Some(-1113364235_i32));
    assert_eq!(a_longerunion.get_sint(2), Some(2127464536_i32));
    assert_eq!(a_longerunion.get_sint(3), Some(-15582438_i32));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);

    // Example for SizeUnion for 64-bit CPU
    #[cfg(target_pointer_width = "64")]
    {
        use cryptocol::number::SizeUnion;
        let mut a_sizeunion = SizeUnion::new();
        let b0 = a_sizeunion.set_sint(0, -246805681_i32);
        let b1 = a_sizeunion.set_sint(1, -1113364235_i32);
        let b2 = a_sizeunion.set_sint(2, 100_i32);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        for i in 0..2
        {
            match a_sizeunion.get_sint(i)
            {
                Some(a) => { println!("a_sizeunion.set_sint({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_sizeunion.get_sint(0), Some(-246805681_i32));
        assert_eq!(a_sizeunion.get_sint(1), Some(-1113364235_i32));
        assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
    }
    println!("--------------------------------------");
}


fn unions_get_set_long_main()
{
    unions_get_ulong_();
    unions_set_ulong_();
    unions_get_slong_();
    unions_set_slong_();
    unions_get_ulong();
    unions_set_ulong();
    unions_get_slong();
    unions_set_slong();
}

fn unions_get_ulong_()
{
    println!("unions_get_ulong_()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..2
        { println!("a_longerunion.get_ulong_({}) = {}", i, a_longerunion.get_ulong_(i)); }
    assert_eq!(a_longerunion.get_ulong_(0), 13664881099896654671_u64);
    assert_eq!(a_longerunion.get_ulong_(1), 18379818014235068504_u64);

    // It will panic.
    // println!("a_longerunion.get_ulong_(2) = {}", a_longerunion.get_ulong_(2));
    println!("--------------------------------------");
}

fn unions_set_ulong_()
{
    println!("unions_set_ulong_()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_ulong_(0, 13664881099896654671_u64);
    a_longerunion.set_ulong_(1, 18379818014235068504_u64);

    // It will panic.
    // a_longerunion.set_ulong_(2, 100_u64);

    for i in 0..2
        { println!("a_longerunion.get_ulong_({}) = {}", i, a_longerunion.get_ulong_(i)); }
    assert_eq!(a_longerunion.get_ulong_(0), 13664881099896654671_u64);
    assert_eq!(a_longerunion.get_ulong_(1), 18379818014235068504_u64);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn unions_get_slong_()
{
    println!("unions_get_slong_()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..2
        { println!("a_longerunion.get_sint_({}) = {}", i, a_longerunion.get_sint_(i)); }
    assert_eq!(a_longerunion.get_slong_(0), -4781862973812896945_i64);
    assert_eq!(a_longerunion.get_slong_(1), -66926059474483112_i64);

    // It will panic.
    // println!("a_longerunion.get_slong_(2) = {}", a_longerunion.get_slong_(2));
    println!("--------------------------------------");
}

fn unions_set_slong_()
{
    println!("unions_set_slong_()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    a_longerunion.set_slong_(0, -4781862973812896945_i64);
    a_longerunion.set_slong_(1, -66926059474483112_i64);

    // It will panic.
    // a_longerunion.set_slong_(2, 100_i64);

    for i in 0..2
        { println!("a_longerunion.get_slong_({}) = {}", i, a_longerunion.get_slong_(i)); }
    assert_eq!(a_longerunion.get_slong_(0), -4781862973812896945_i64);
    assert_eq!(a_longerunion.get_slong_(1), -66926059474483112_i64);
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn unions_get_ulong()
{
    println!("unions_get_ulong()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..2
    {
        match a_longerunion.get_ulong(i)
        {
            Some(a) => { println!("a_longerunion.get_ulong({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_ulong(0), Some(13664881099896654671_u64));
    assert_eq!(a_longerunion.get_ulong(1), Some(18379818014235068504_u64));
    assert_eq!(a_longerunion.get_ulong(4), None);
    println!("--------------------------------------");
}

fn unions_set_ulong()
{
    println!("unions_set_ulong()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_ulong(0, 13664881099896654671_u64);
    let b1 = a_longerunion.set_ulong(1, 18379818014235068504_u64);
    let b2 = a_longerunion.set_ulong(4, 100_u64);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);

    for i in 0..2
    {
        match a_longerunion.get_ulong(i)
        {
            Some(a) => { println!("a_longunion.get_ulong({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_ulong(0), Some(13664881099896654671_u64));
    assert_eq!(a_longerunion.get_ulong(1), Some(18379818014235068504_u64));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}

fn unions_get_slong()
{
    println!("unions_get_slong()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    for i in 0..2
    {
        match a_longerunion.get_slong(i)
        {
            Some(a) => { println!("a_longerunion.get_slong({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_slong(0), Some(-4781862973812896945_i64));
    assert_eq!(a_longerunion.get_slong(1), Some(-66926059474483112_i64));
    assert_eq!(a_longerunion.get_sint(4), None);
    println!("--------------------------------------");
}

fn unions_set_slong()
{
    println!("unions_set_slong()");
    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    let mut a_longerunion = LongerUnion::new();
    let b0 = a_longerunion.set_slong(0, -4781862973812896945_i64);
    let b1 = a_longerunion.set_slong(1, -66926059474483112_i64);
    let b2 = a_longerunion.set_slong(4, 100_i64);
    assert_eq!(b0, true);
    assert_eq!(b1, true);
    assert_eq!(b2, false);

    for i in 0..2
    {
        match a_longerunion.get_slong(i)
        {
            Some(a) => { println!("a_longerunion.get_slong({}) = {}", i, a); },
            _ => {},
        }
    }
    assert_eq!(a_longerunion.get_slong(0), Some(-4781862973812896945_i64));
    assert_eq!(a_longerunion.get_slong(1), Some(-66926059474483112_i64));
    assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    println!("--------------------------------------");
}


fn unions_get_set_size_main()
{
    unions_get_usize_();
    unions_set_usize_();
    unions_get_ssize_();
    unions_set_ssize_();
    unions_get_usize();
    unions_set_usize();
    unions_get_ssize();
    unions_set_ssize();
}

fn unions_get_usize_()
{
    println!("unions_get_usize_()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let a_shortunion = ShortUnion::new_with(2895_u16);
        for i in 0..2
            { println!("a_shortunion.get_usize_({}) = {}", i, a_shortunion.get_usize_(i)); }
        assert_eq!(a_shortunion.get_usize_(0), 79_usize);
        assert_eq!(a_shortunion.get_usize_(1), 11_usize);

        // It will panic.
        // println!("a_shortunion.get_usize_(2) = {}", a_shortunion.get_usize_(2));

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..4
            { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        assert_eq!(a_intunion.get_usize_(0), 79_usize);
        assert_eq!(a_intunion.get_usize_(1), 11_usize);
        assert_eq!(a_intunion.get_usize_(2), 74_usize);
        assert_eq!(a_intunion.get_usize_(3), 241_usize);

        // It will panic.
        // println!("a_intunion.get_usize_(4) = {}", a_intunion.get_usize_(4));

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..8
            { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        assert_eq!(a_longunion.get_usize_(0), 79_usize);
        assert_eq!(a_longunion.get_usize_(1), 11_usize);
        assert_eq!(a_longunion.get_usize_(2), 74_usize);
        assert_eq!(a_longunion.get_usize_(3), 241_usize);
        assert_eq!(a_longunion.get_usize_(4), 245_usize);
        assert_eq!(a_longunion.get_usize_(5), 104_usize);
        assert_eq!(a_longunion.get_usize_(6), 163_usize);
        assert_eq!(a_longunion.get_usize_(7), 189_usize);

        // It will panic.
        // println!("a_longunion.get_usize_(8) = {}", a_longunion.get_usize_(8));

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..16
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_usize_(0), 79_usize);
        assert_eq!(a_longerunion.get_usize_(1), 11_usize);
        assert_eq!(a_longerunion.get_usize_(2), 74_usize);
        assert_eq!(a_longerunion.get_usize_(3), 241_usize);
        assert_eq!(a_longerunion.get_usize_(4), 245_usize);
        assert_eq!(a_longerunion.get_usize_(5), 104_usize);
        assert_eq!(a_longerunion.get_usize_(6), 163_usize);
        assert_eq!(a_longerunion.get_usize_(7), 189_usize);
        assert_eq!(a_longerunion.get_usize_(8), 88_usize);
        assert_eq!(a_longerunion.get_usize_(9), 136_usize);
        assert_eq!(a_longerunion.get_usize_(10), 206_usize);
        assert_eq!(a_longerunion.get_usize_(11), 126_usize);
        assert_eq!(a_longerunion.get_usize_(12), 26_usize);
        assert_eq!(a_longerunion.get_usize_(13), 59_usize);
        assert_eq!(a_longerunion.get_usize_(14), 18_usize);
        assert_eq!(a_longerunion.get_usize_(15), 255_usize);

        // It will panic.
        // println!("a_longerunion.get_usize_(16) = {}", a_longerunion.get_usize_(16));
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..2
            { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        assert_eq!(a_intunion.get_usize_(0), 2895_usize);
        assert_eq!(a_intunion.get_usize_(1), 61770_usize);

        // It will panic.
        // println!("a_intunion.get_usize_(2) = {}", a_intunion.get_usize_(2));

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..4
            { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        assert_eq!(a_longunion.get_usize_(0), 2895_usize);
        assert_eq!(a_longunion.get_usize_(1), 61770_usize);
        assert_eq!(a_longunion.get_usize_(2), 26869_usize);
        assert_eq!(a_longunion.get_usize_(3), 48547_usize);

        // It will panic.
        // println!("a_longunion.get_usize_(4) = {}", a_longunion.get_usize_(4));

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..8
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_usize_(0), 2895_usize);
        assert_eq!(a_longerunion.get_usize_(1), 61770_usize);
        assert_eq!(a_longerunion.get_usize_(2), 26869_usize);
        assert_eq!(a_longerunion.get_usize_(3), 48547_usize);
        assert_eq!(a_longerunion.get_usize_(4), 34904_usize);
        assert_eq!(a_longerunion.get_usize_(5), 32462_usize);
        assert_eq!(a_longerunion.get_usize_(6), 15130_usize);
        assert_eq!(a_longerunion.get_usize_(7), 65298_usize);

        // It will panic.
        // println!("a_longerunion.get_usize_(8) = {}", a_longerunion.get_usize_(8));
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..2
            { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        assert_eq!(a_longunion.get_usize_(0), 4048161615_usize);
        assert_eq!(a_longunion.get_usize_(1), 3181603061_usize);

        // It will panic.
        // println!("a_longunion.get_usize_(2) = {}", a_longunion.get_usize_(2));

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..4
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_usize_(0), 4048161615_usize);
        assert_eq!(a_longerunion.get_usize_(1), 3181603061_usize);
        assert_eq!(a_longerunion.get_usize_(2), 2127464536_usize);
        assert_eq!(a_longerunion.get_usize_(3), 4279384858_usize);

        // It will panic.
        // println!("a_longerunion.get_usize_(4) = {}", a_longerunion.get_usize_(4));
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_sizeerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..2
            { println!("a_sizeerunion.get_usize_({}) = {}", i, a_sizeerunion.get_usize_(i)); }
        assert_eq!(a_sizeerunion.get_usize_(0), 13664881099896654671_usize);
        assert_eq!(a_sizeerunion.get_usize_(1), 18379818014235068504_usize);

        // It will panic.
        // println!("a_sizeerunion.get_usize_(2) = {}", a_sizeerunion.get_usize_(2));
    }
    println!("--------------------------------------");
}

fn unions_set_usize_()
{
    println!("unions_set_usize_()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let mut a_shortunion = ShortUnion::new();
        a_shortunion.set_usize_(0, 79_usize);
        a_shortunion.set_usize_(1, 11_usize);

        // It will panic.
        // a_shortunion.set_usize_(2, 100_usize);

        println!("a_shortunion.get() = {}", a_shortunion.get());
        for i in 0..2
            { println!("a_shortunion.get_usize_({}) = {}", i, a_shortunion.get_usize_(i)); }
        assert_eq!(a_shortunion.get_usize_(0), 79_usize);
        assert_eq!(a_shortunion.get_usize_(1), 11_usize);
        assert_eq!(a_shortunion.get(), 2895_u16);

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        a_intunion.set_usize_(0, 79_usize);
        a_intunion.set_usize_(1, 11_usize);
        a_intunion.set_usize_(2, 74_usize);
        a_intunion.set_usize_(3, 241_usize);

        // It will panic.
        // a_intunion.set_usize_(4, 100_usize);

        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..4
            { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        assert_eq!(a_intunion.get_usize_(0), 79_usize);
        assert_eq!(a_intunion.get_usize_(1), 11_usize);
        assert_eq!(a_intunion.get_usize_(2), 74_usize);
        assert_eq!(a_intunion.get_usize_(3), 241_usize);
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        a_longunion.set_usize_(0, 79_usize);
        a_longunion.set_usize_(1, 11_usize);
        a_longunion.set_usize_(2, 74_usize);
        a_longunion.set_usize_(3, 241_usize);
        a_longunion.set_usize_(4, 245_usize);
        a_longunion.set_usize_(5, 104_usize);
        a_longunion.set_usize_(6, 163_usize);
        a_longunion.set_usize_(7, 189_usize);

        // It will panic.
        // a_longunion.set_usize_(8, 100_usize);

        for i in 0..8
            { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        assert_eq!(a_longunion.get_usize_(0), 79_usize);
        assert_eq!(a_longunion.get_usize_(1), 11_usize);
        assert_eq!(a_longunion.get_usize_(2), 74_usize);
        assert_eq!(a_longunion.get_usize_(3), 241_usize);
        assert_eq!(a_longunion.get_usize_(4), 245_usize);
        assert_eq!(a_longunion.get_usize_(5), 104_usize);
        assert_eq!(a_longunion.get_usize_(6), 163_usize);
        assert_eq!(a_longunion.get_usize_(7), 189_usize);
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_usize_(0, 79_usize);
        a_longerunion.set_usize_(1, 11_usize);
        a_longerunion.set_usize_(2, 74_usize);
        a_longerunion.set_usize_(3, 241_usize);
        a_longerunion.set_usize_(4, 245_usize);
        a_longerunion.set_usize_(5, 104_usize);
        a_longerunion.set_usize_(6, 163_usize);
        a_longerunion.set_usize_(7, 189_usize);
        a_longerunion.set_usize_(8, 88_usize);
        a_longerunion.set_usize_(9, 136_usize);
        a_longerunion.set_usize_(10, 206_usize);
        a_longerunion.set_usize_(11, 126_usize);
        a_longerunion.set_usize_(12, 26_usize);
        a_longerunion.set_usize_(13, 59_usize);
        a_longerunion.set_usize_(14, 18_usize);
        a_longerunion.set_usize_(15, 255_usize);

        // It will panic.
        // a_longerunion.set_usize_(16, 100_usize);

        for i in 0..16
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_usize_(0), 79_usize);
        assert_eq!(a_longerunion.get_usize_(1), 11_usize);
        assert_eq!(a_longerunion.get_usize_(2), 74_usize);
        assert_eq!(a_longerunion.get_usize_(3), 241_usize);
        assert_eq!(a_longerunion.get_usize_(4), 245_usize);
        assert_eq!(a_longerunion.get_usize_(5), 104_usize);
        assert_eq!(a_longerunion.get_usize_(6), 163_usize);
        assert_eq!(a_longerunion.get_usize_(7), 189_usize);
        assert_eq!(a_longerunion.get_usize_(8), 88_usize);
        assert_eq!(a_longerunion.get_usize_(9), 136_usize);
        assert_eq!(a_longerunion.get_usize_(10), 206_usize);
        assert_eq!(a_longerunion.get_usize_(11), 126_usize);
        assert_eq!(a_longerunion.get_usize_(12), 26_usize);
        assert_eq!(a_longerunion.get_usize_(13), 59_usize);
        assert_eq!(a_longerunion.get_usize_(14), 18_usize);
        assert_eq!(a_longerunion.get_usize_(15), 255_usize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        a_intunion.set_usize_(0, 2895_usize);
        a_intunion.set_usize_(1, 61770_usize);

        // It will panic.
        // a_intunion.set_usize_(2, 100_usize);

        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..2
            { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        assert_eq!(a_intunion.get_usize_(0), 2895_usize);
        assert_eq!(a_intunion.get_usize_(1), 61770_usize);
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        a_longunion.set_usize_(0, 2895_usize);
        a_longunion.set_usize_(1, 61770_usize);
        a_longunion.set_usize_(2, 26869_usize);
        a_longunion.set_usize_(3, 48547_usize);

        // It will panic.
        // a_longunion.set_usize_(4, 100_usize);

        for i in 0..4
            { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        assert_eq!(a_longunion.get_usize_(0), 2895_usize);
        assert_eq!(a_longunion.get_usize_(1), 61770_usize);
        assert_eq!(a_longunion.get_usize_(2), 26869_usize);
        assert_eq!(a_longunion.get_usize_(3), 48547_usize);
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_usize_(0, 2895_usize);
        a_longerunion.set_usize_(1, 61770_usize);
        a_longerunion.set_usize_(2, 26869_usize);
        a_longerunion.set_usize_(3, 48547_usize);
        a_longerunion.set_usize_(4, 34904_usize);
        a_longerunion.set_usize_(5, 32462_usize);
        a_longerunion.set_usize_(6, 15130_usize);
        a_longerunion.set_usize_(7, 65298_usize);

        // It will panic.
        // a_longerunion.set_usize_(8, 100_usize);

        for i in 0..8
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_usize_(0), 2895_usize);
        assert_eq!(a_longerunion.get_usize_(1), 61770_usize);
        assert_eq!(a_longerunion.get_usize_(2), 26869_usize);
        assert_eq!(a_longerunion.get_usize_(3), 48547_usize);
        assert_eq!(a_longerunion.get_usize_(4), 34904_usize);
        assert_eq!(a_longerunion.get_usize_(5), 32462_usize);
        assert_eq!(a_longerunion.get_usize_(6), 15130_usize);
        assert_eq!(a_longerunion.get_usize_(7), 65298_usize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        a_longunion.set_usize_(0, 4048161615_usize);
        a_longunion.set_usize_(1, 3181603061_usize);

        // It will panic.
        // a_longunion.set_ushort_(2, 100_usize);

        for i in 0..2
            { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
        assert_eq!(a_longunion.get_usize_(0), 4048161615_usize);
        assert_eq!(a_longunion.get_usize_(1), 3181603061_usize);
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_usize_(0, 4048161615_usize);
        a_longerunion.set_usize_(1, 3181603061_usize);
        a_longerunion.set_usize_(2, 2127464536_usize);
        a_longerunion.set_usize_(3, 4279384858_usize);

        // It will panic.
        // a_longerunion.set_usize_(4, 100_usize);

        for i in 0..4
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_usize_(0), 4048161615_usize);
        assert_eq!(a_longerunion.get_usize_(1), 3181603061_usize);
        assert_eq!(a_longerunion.get_usize_(2), 2127464536_usize);
        assert_eq!(a_longerunion.get_usize_(3), 4279384858_usize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_sizeerunion = LongerUnion::new();
        a_sizeerunion.set_usize_(0, 13664881099896654671_usize);
        a_sizeerunion.set_usize_(1, 18379818014235068504_usize);

        // It will panic.
        // a_sizeerunion.set_usize_(2, 100_usize);

        for i in 0..2
            { println!("a_sizeerunion.get_usize_({}) = {}", i, a_sizeerunion.get_usize_(i)); }
        assert_eq!(a_sizeerunion.get_usize_(0), 13664881099896654671_usize);
        assert_eq!(a_sizeerunion.get_usize_(1), 18379818014235068504_usize);
        assert_eq!(a_sizeerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    println!("--------------------------------------");
}

fn unions_get_ssize_()
{
    println!("unions_get_ssize_()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let a_shortunion = ShortUnion::new_with(2895_u16);
        for i in 0..2
            { println!("a_shortunion.get_ssize_({}) = {}", i, a_shortunion.get_ssize_(i)); }
        assert_eq!(a_shortunion.get_ssize_(0), 79__isize);
        assert_eq!(a_shortunion.get_ssize_(1), 11__isize);

        // It will panic.
        // println!("a_shortunion.get_ssize_(2) = {}", a_shortunion.get_ssize_(2));

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..4
            { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        assert_eq!(a_intunion.get_ssize_(0), 79__isize);
        assert_eq!(a_intunion.get_ssize_(1), 11__isize);
        assert_eq!(a_intunion.get_ssize_(2), 74__isize);
        assert_eq!(a_intunion.get_ssize_(3), -15__isize);

        // It will panic.
        // println!("a_intunion.get_ssize_(4) = {}", a_intunion.get_ssize_(4));

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..8
            { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        assert_eq!(a_longunion.get_ssize_(0), 79__isize);
        assert_eq!(a_longunion.get_ssize_(1), 11__isize);
        assert_eq!(a_longunion.get_ssize_(2), 74__isize);
        assert_eq!(a_longunion.get_ssize_(3), -15__isize);
        assert_eq!(a_longunion.get_ssize_(4), -11__isize);
        assert_eq!(a_longunion.get_ssize_(5), 104__isize);
        assert_eq!(a_longunion.get_ssize_(6), -93__isize);
        assert_eq!(a_longunion.get_ssize_(7), -67__isize);

        // It will panic.
        // println!("a_longunion.get_ssize_(8) = {}", a_longunion.get_ssize_(8));

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..16
            { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), 79__isize);
        assert_eq!(a_longerunion.get_ssize_(1), 11__isize);
        assert_eq!(a_longerunion.get_ssize_(2), 74__isize);
        assert_eq!(a_longerunion.get_ssize_(3), -15__isize);
        assert_eq!(a_longerunion.get_ssize_(4), -11__isize);
        assert_eq!(a_longerunion.get_ssize_(5), 104__isize);
        assert_eq!(a_longerunion.get_ssize_(6), -93__isize);
        assert_eq!(a_longerunion.get_ssize_(7), -67__isize);
        assert_eq!(a_longerunion.get_ssize_(8), 88__isize);
        assert_eq!(a_longerunion.get_ssize_(9), -120__isize);
        assert_eq!(a_longerunion.get_ssize_(10), -50__isize);
        assert_eq!(a_longerunion.get_ssize_(11), 126__isize);
        assert_eq!(a_longerunion.get_ssize_(12), 26__isize);
        assert_eq!(a_longerunion.get_ssize_(13), 59__isize);
        assert_eq!(a_longerunion.get_ssize_(14), 18__isize);
        assert_eq!(a_longerunion.get_ssize_(15), -1__isize);

        // It will panic.
        // println!("a_longerunion.get_ssize_(16) = {}", a_longerunion.get_ssize_(16));
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..2
            { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        assert_eq!(a_intunion.get_ssize_(0), 2895_isize);
        assert_eq!(a_intunion.get_ssize_(1), -3766_isize);

        // It will panic.
        // println!("a_intunion.get_ssize_(2) = {}", a_intunion.get_ssize_(2));

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..4
            { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        assert_eq!(a_longunion.get_ssize_(0), 2895_isize);
        assert_eq!(a_longunion.get_ssize_(1), -3766_isize);
        assert_eq!(a_longunion.get_ssize_(2), 26869_isize);
        assert_eq!(a_longunion.get_ssize_(3), -16989_isize);

        // It will panic.
        // println!("a_longunion.get_ssize_(4) = {}", a_longunion.get_ssize_(4));

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..8
            { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), 2895_isize);
        assert_eq!(a_longerunion.get_ssize_(1), -3766_isize);
        assert_eq!(a_longerunion.get_ssize_(2), 26869_isize);
        assert_eq!(a_longerunion.get_ssize_(3), -16989_isize);
        assert_eq!(a_longerunion.get_ssize_(4), -30632_isize);
        assert_eq!(a_longerunion.get_ssize_(5), 32462_isize);
        assert_eq!(a_longerunion.get_ssize_(6), 15130_isize);
        assert_eq!(a_longerunion.get_ssize_(7), -238_isize);

        // It will panic.
        // println!("a_longerunion.get_ssize_(8) = {}", a_longerunion.get_ssize_(8));
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..2
            { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        assert_eq!(a_longunion.get_ssize_(0), -246805681_isize);
        assert_eq!(a_longunion.get_ssize_(1), -1113364235_isize);

        // It will panic.
        // println!("a_longunion.get_ssize_(2) = {}", a_longunion.get_ssize_(2));

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..4
            { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), -246805681_isize);
        assert_eq!(a_longerunion.get_ssize_(1), -1113364235_isize);
        assert_eq!(a_longerunion.get_ssize_(2), 2127464536_isize);
        assert_eq!(a_longerunion.get_ssize_(3), -15582438_isize);

        // It will panic.
        // println!("a_longerunion.get_ssize_(4) = {}", a_longerunion.get_ssize_(4));
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..2
            { println!("a_sizeerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), -4781862973812896945_isize);
        assert_eq!(a_longerunion.get_ssize_(1), -66926059474483112_isize);

        // It will panic.
        // println!("a_sizeerunion.get_ssize_(2) = {}", a_sizeerunion.get_ssize_(2));
    }
    println!("--------------------------------------");
}

fn unions_set_ssize_()
{
    println!("unions_set_ssize_()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let mut a_shortunion = ShortUnion::new();
        a_shortunion.set_ssize_(0, 79_isize);
        a_shortunion.set_ssize_(1, 11_isize);

        // It will panic.
        // a_shortunion.set_ssize_(2, 100_isize);

        println!("a_shortunion.get() = {}", a_shortunion.get());
        for i in 0..2
            { println!("a_shortunion.get_ssize_({}) = {}", i, a_shortunion.get_ssize_(i)); }
        assert_eq!(a_shortunion.get_ssize_(0), 79_isize);
        assert_eq!(a_shortunion.get_ssize_(1), 11_isize);
        assert_eq!(a_shortunion.get(), 2895_u16);

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        a_intunion.set_ssize_(0, 79_isize);
        a_intunion.set_ssize_(1, 11_isize);
        a_intunion.set_ssize_(2, 74_isize);
        a_intunion.set_ssize_(3, -15_isize);

        // It will panic.
        // a_intunion.set_ssize_(4, 100_isize);

        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..4
            { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        assert_eq!(a_intunion.get_ssize_(0), 79_isize);
        assert_eq!(a_intunion.get_ssize_(1), 11_isize);
        assert_eq!(a_intunion.get_ssize_(2), 74_isize);
        assert_eq!(a_intunion.get_ssize_(3), -15_isize);
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        a_longunion.set_ssize_(0, 79_isize);
        a_longunion.set_ssize_(1, 11_isize);
        a_longunion.set_ssize_(2, 74_isize);
        a_longunion.set_ssize_(3, -15_isize);
        a_longunion.set_ssize_(4, -11_isize);
        a_longunion.set_ssize_(5, 104_isize);
        a_longunion.set_ssize_(6, -93_isize);
        a_longunion.set_ssize_(7, -67_isize);

        // It will panic.
        // a_intunion.set_ssize_(8, 100_isize);

        println!("a_longunion.get() = {}", a_longunion.get());
        for i in 0..8
            { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        assert_eq!(a_longunion.get_ssize_(0), 79_isize);
        assert_eq!(a_longunion.get_ssize_(1), 11_isize);
        assert_eq!(a_longunion.get_ssize_(2), 74_isize);
        assert_eq!(a_longunion.get_ssize_(3), -15_isize);
        assert_eq!(a_longunion.get_ssize_(4), -11_isize);
        assert_eq!(a_longunion.get_ssize_(5), 104_isize);
        assert_eq!(a_longunion.get_ssize_(6), -93_isize);
        assert_eq!(a_longunion.get_ssize_(7), -67_isize);
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_ssize_(0, 79_isize);
        a_longerunion.set_ssize_(1, 11_isize);
        a_longerunion.set_ssize_(2, 74_isize);
        a_longerunion.set_ssize_(3, -15_isize);
        a_longerunion.set_ssize_(4, -11_isize);
        a_longerunion.set_ssize_(5, 104_isize);
        a_longerunion.set_ssize_(6, -93_isize);
        a_longerunion.set_ssize_(7, -67_isize);
        a_longerunion.set_ssize_(8, 88_isize);
        a_longerunion.set_ssize_(9, -120_isize);
        a_longerunion.set_ssize_(10, -50_isize);
        a_longerunion.set_ssize_(11, 126_isize);
        a_longerunion.set_ssize_(12, 26_isize);
        a_longerunion.set_ssize_(13, 59_isize);
        a_longerunion.set_ssize_(14, 18_isize);
        a_longerunion.set_ssize_(15, -1_isize);

        // It will panic.
        // a_longerunion.set_ssize_(16, 100_isize);

        println!("a_longerunion.get() = {}", a_longerunion.get());
        for i in 0..16
            { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), 79_isize);
        assert_eq!(a_longerunion.get_ssize_(1), 11_isize);
        assert_eq!(a_longerunion.get_ssize_(2), 74_isize);
        assert_eq!(a_longerunion.get_ssize_(3), -15_isize);
        assert_eq!(a_longerunion.get_ssize_(4), -11_isize);
        assert_eq!(a_longerunion.get_ssize_(5), 104_isize);
        assert_eq!(a_longerunion.get_ssize_(6), -93_isize);
        assert_eq!(a_longerunion.get_ssize_(7), -67_isize);
        assert_eq!(a_longerunion.get_ssize_(8), 88_isize);
        assert_eq!(a_longerunion.get_ssize_(9), -120_isize);
        assert_eq!(a_longerunion.get_ssize_(10), -50_isize);
        assert_eq!(a_longerunion.get_ssize_(11), 126_isize);
        assert_eq!(a_longerunion.get_ssize_(12), 26_isize);
        assert_eq!(a_longerunion.get_ssize_(13), 59_isize);
        assert_eq!(a_longerunion.get_ssize_(14), 18_isize);
        assert_eq!(a_longerunion.get_ssize_(15), -1_isize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_usize);
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        a_intunion.set_ssize_(0, 2895_isize);
        a_intunion.set_ssize_(1, -3766_isize);

        // It will panic.
        // a_intunion.set_ssize_(2, 100_isize);

        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..2
            { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        assert_eq!(a_intunion.get_ssize_(0), 2895_isize);
        assert_eq!(a_intunion.get_ssize_(1), -3766_isize);
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        a_longunion.set_ssize_(0, 2895_isize);
        a_longunion.set_ssize_(1, -3766_isize);
        a_longunion.set_ssize_(2, 26869_isize);
        a_longunion.set_ssize_(3, -16989_isize);

        // It will panic.
        // a_longunion.set_ssize_(4, 100_isize);

        for i in 0..4
            { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        assert_eq!(a_longunion.get_ssize_(0), 2895_isize);
        assert_eq!(a_longunion.get_ssize_(1), -3766_isize);
        assert_eq!(a_longunion.get_ssize_(2), 26869_isize);
        assert_eq!(a_longunion.get_ssize_(3), -16989_isize);
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_ssize_(0, 2895_isize);
        a_longerunion.set_ssize_(1, -3766_isize);
        a_longerunion.set_ssize_(2, 26869_isize);
        a_longerunion.set_ssize_(3, -16989_isize);
        a_longerunion.set_ssize_(4, -30632_isize);
        a_longerunion.set_ssize_(5, 32462_isize);
        a_longerunion.set_ssize_(6, 15130_isize);
        a_longerunion.set_ssize_(7, -238_isize);

        // It will panic.
        // a_longerunion.set_ssize_(8, 100_isize);

        for i in 0..8
            { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), 2895_isize);
        assert_eq!(a_longerunion.get_ssize_(1), -3766_isize);
        assert_eq!(a_longerunion.get_ssize_(2), 26869_isize);
        assert_eq!(a_longerunion.get_ssize_(3), -16989_isize);
        assert_eq!(a_longerunion.get_ssize_(4), -30632_isize);
        assert_eq!(a_longerunion.get_ssize_(5), 32462_isize);
        assert_eq!(a_longerunion.get_ssize_(6), 15130_isize);
        assert_eq!(a_longerunion.get_ssize_(7), -238_isize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        a_longunion.set_ssize_(0, -246805681_isize);
        a_longunion.set_ssize_(1, -1113364235_isize);

        // It will panic.
        // a_longunion.set_ssize_(2, 100_isize);

        for i in 0..2
            { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        assert_eq!(a_longunion.get_ssize_(0), -246805681_isize);
        assert_eq!(a_longunion.get_ssize_(1), -1113364235_isize);
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_ssize_(0, -246805681_isize);
        a_longerunion.set_ssize_(1, -1113364235_isize);
        a_longerunion.set_ssize_(2, 2127464536_isize);
        a_longerunion.set_ssize_(3, -15582438_isize);

        // It will panic.
        // a_longerunion.set_usize_(4, 100_usize);

        for i in 0..4
            { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), -246805681_isize);
        assert_eq!(a_longerunion.get_ssize_(1), -1113364235_isize);
        assert_eq!(a_longerunion.get_ssize_(2), 2127464536_isize);
        assert_eq!(a_longerunion.get_ssize_(3), -15582438_isize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        a_longerunion.set_ssize_(0, -4781862973812896945_isize);
        a_longerunion.set_ssize_(1, -66926059474483112_isize);

        // It will panic.
        // a_longerunion.set_ssize_(2, 100_isize);

        for i in 0..2
            { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        assert_eq!(a_longerunion.get_ssize_(0), -4781862973812896945_isize);
        assert_eq!(a_longerunion.get_ssize_(1), -66926059474483112_isize);
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    println!("--------------------------------------");
}

fn unions_get_usize()
{
    println!("unions_get_usize()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let a_shortunion = ShortUnion::new_with(2895_u16);
        for i in 0..2
        {
            match a_shortunion.get_usize(i)
            {
                Some(a) => { println!("a_shortunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_shortunion.get_usize(0), Some(79_usize));
        assert_eq!(a_shortunion.get_usize(1), Some(11_usize));
        assert_eq!(a_shortunion.get_usize(2), None);

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..4
        {
            match a_intunion.get_usize(i)
            {
                Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_usize(0), Some(79_usize));
        assert_eq!(a_intunion.get_usize(1), Some(11_usize));
        assert_eq!(a_intunion.get_usize(2), Some(74_usize));
        assert_eq!(a_intunion.get_usize(3), Some(241_usize));
        assert_eq!(a_intunion.get_usize(4), None);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..8
        {
            match a_longunion.get_usize(i)
            {
                Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_usize(0), Some(79_usize));
        assert_eq!(a_longunion.get_usize(1), Some(11_usize));
        assert_eq!(a_longunion.get_usize(2), Some(74_usize));
        assert_eq!(a_longunion.get_usize(3), Some(241_usize));
        assert_eq!(a_longunion.get_usize(4), Some(245_usize));
        assert_eq!(a_longunion.get_usize(5), Some(104_usize));
        assert_eq!(a_longunion.get_usize(6), Some(163_usize));
        assert_eq!(a_longunion.get_usize(7), Some(189_usize));
        assert_eq!(a_longunion.get_usize(8), None);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_usize);
        for i in 0..16
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(79_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(11_usize));
        assert_eq!(a_longerunion.get_usize(2), Some(74_usize));
        assert_eq!(a_longerunion.get_usize(3), Some(241_usize));
        assert_eq!(a_longerunion.get_usize(4), Some(245_usize));
        assert_eq!(a_longerunion.get_usize(5), Some(104_usize));
        assert_eq!(a_longerunion.get_usize(6), Some(163_usize));
        assert_eq!(a_longerunion.get_usize(7), Some(189_usize));
        assert_eq!(a_longerunion.get_usize(8), Some(88_usize));
        assert_eq!(a_longerunion.get_usize(9), Some(136_usize));
        assert_eq!(a_longerunion.get_usize(10), Some(206_usize));
        assert_eq!(a_longerunion.get_usize(11), Some(126_usize));
        assert_eq!(a_longerunion.get_usize(12), Some(26_usize));
        assert_eq!(a_longerunion.get_usize(13), Some(59_usize));
        assert_eq!(a_longerunion.get_usize(14), Some(18_usize));
        assert_eq!(a_longerunion.get_usize(15), Some(255_usize));
        assert_eq!(a_longerunion.get_usize(16), None);
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..2
        {
            match a_intunion.get_usize(i)
            {
                Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_usize(0), Some(2895_usize));
        assert_eq!(a_intunion.get_usize(1), Some(61770_usize));
        assert_eq!(a_intunion.get_usize(2), None);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..4
        {
            match a_longunion.get_usize(i)
            {
                Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_usize(0), Some(2895_usize));
        assert_eq!(a_longunion.get_usize(1), Some(61770_usize));
        assert_eq!(a_longunion.get_usize(2), Some(26869_usize));
        assert_eq!(a_longunion.get_usize(3), Some(48547_usize));
        assert_eq!(a_longunion.get_usize(4), None);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..8
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(2895_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(61770_usize));
        assert_eq!(a_longerunion.get_usize(2), Some(26869_usize));
        assert_eq!(a_longerunion.get_usize(3), Some(48547_usize));
        assert_eq!(a_longerunion.get_usize(4), Some(34904_usize));
        assert_eq!(a_longerunion.get_usize(5), Some(32462_usize));
        assert_eq!(a_longerunion.get_usize(6), Some(15130_usize));
        assert_eq!(a_longerunion.get_usize(7), Some(65298_usize));
        assert_eq!(a_longerunion.get_usize(8), None);
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..2
        {
            match a_longunion.get_usize(i)
            {
                Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_usize(0), Some(4048161615_usize));
        assert_eq!(a_longunion.get_usize(1), Some(3181603061_usize));
        assert_eq!(a_longunion.get_usize(2), None);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..4
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(4048161615_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(3181603061_usize));
        assert_eq!(a_longerunion.get_usize(2), Some(2127464536_usize));
        assert_eq!(a_longerunion.get_usize(3), Some(4279384858_usize));
        assert_eq!(a_longerunion.get_usize(4), None);
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..2
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(13664881099896654671_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(18379818014235068504_usize));
        assert_eq!(a_longerunion.get_usize(4), None);
    }
    println!("--------------------------------------");
}

fn unions_set_usize()
{
    println!("unions_set_usize()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let mut a_shortunion = ShortUnion::new();
        let b0 = a_shortunion.set_usize(0, 79_usize);
        let b1 = a_shortunion.set_usize(1, 11_usize);
        let b2 = a_shortunion.set_usize(2, 100_usize);  // Nothing will be done
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        println!("a_shortunion.get() = {}", a_shortunion.get());
        for i in 0..2
        {
            match a_shortunion.get_usize(i)
            {
                Some(a) => { println!("a_shortunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_shortunion.get_usize(0), Some(79_usize));
        assert_eq!(a_shortunion.get_usize(1), Some(11_usize));
        assert_eq!(a_shortunion.get(), 2895_u16);

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        let b0 = a_intunion.set_usize(0, 79_usize);
        let b1 = a_intunion.set_usize(1, 11_usize);
        let b2 = a_intunion.set_usize(2, 74_usize);
        let b3 = a_intunion.set_usize(3, 241_usize);
        let b4 = a_intunion.set_usize(4, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);
        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..4
        {
            match a_intunion.get_usize(i)
            {
                Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_usize(0), Some(79_usize));
        assert_eq!(a_intunion.get_usize(1), Some(11_usize));
        assert_eq!(a_intunion.get_usize(2), Some(74_usize));
        assert_eq!(a_intunion.get_usize(3), Some(241_usize));
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        let b0 = a_longunion.set_usize(0, 79_usize);
        let b1 = a_longunion.set_usize(1, 11_usize);
        let b2 = a_longunion.set_usize(2, 74_usize);
        let b3 = a_longunion.set_usize(3, 241_usize);
        let b4 = a_longunion.set_usize(4, 245_usize);
        let b5 = a_longunion.set_usize(5, 104_usize);
        let b6 = a_longunion.set_usize(6, 163_usize);
        let b7 = a_longunion.set_usize(7, 189_usize);
        let b8 = a_longunion.set_usize(8, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, false);
        println!("a_longunion.get() = {}", a_longunion.get());
        for i in 0..8
        {
            match a_longunion.get_usize(i)
            {
                Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_usize(0), Some(79_usize));
        assert_eq!(a_longunion.get_usize(1), Some(11_usize));
        assert_eq!(a_longunion.get_usize(2), Some(74_usize));
        assert_eq!(a_longunion.get_usize(3), Some(241_usize));
        assert_eq!(a_longunion.get_usize(4), Some(245_usize));
        assert_eq!(a_longunion.get_usize(5), Some(104_usize));
        assert_eq!(a_longunion.get_usize(6), Some(163_usize));
        assert_eq!(a_longunion.get_usize(7), Some(189_usize));
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_usize(0, 79_usize);
        let b1 = a_longerunion.set_usize(1, 11_usize);
        let b2 = a_longerunion.set_usize(2, 74_usize);
        let b3 = a_longerunion.set_usize(3, 241_usize);
        let b4 = a_longerunion.set_usize(4, 245_usize);
        let b5 = a_longerunion.set_usize(5, 104_usize);
        let b6 = a_longerunion.set_usize(6, 163_usize);
        let b7 = a_longerunion.set_usize(7, 189_usize);
        let b8 = a_longerunion.set_usize(8, 88_usize);
        let b9 = a_longerunion.set_usize(9, 136_usize);
        let b10 = a_longerunion.set_usize(10, 206_usize);
        let b11 = a_longerunion.set_usize(11, 126_usize);
        let b12 = a_longerunion.set_usize(12, 26_usize);
        let b13 = a_longerunion.set_usize(13, 59_usize);
        let b14 = a_longerunion.set_usize(14, 18_usize);
        let b15 = a_longerunion.set_usize(15, 255_usize);
        let b16 = a_longerunion.set_usize(16, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, true);
        assert_eq!(b9, true);
        assert_eq!(b10, true);
        assert_eq!(b11, true);
        assert_eq!(b12, true);
        assert_eq!(b13, true);
        assert_eq!(b14, true);
        assert_eq!(b15, true);
        assert_eq!(b16, false);
        println!("a_longerunion.get() = {}", a_longerunion.get());
        for i in 0..16
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(79_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(11_usize));
        assert_eq!(a_longerunion.get_usize(2), Some(74_usize));
        assert_eq!(a_longerunion.get_usize(3), Some(241_usize));
        assert_eq!(a_longerunion.get_usize(4), Some(245_usize));
        assert_eq!(a_longerunion.get_usize(5), Some(104_usize));
        assert_eq!(a_longerunion.get_usize(6), Some(163_usize));
        assert_eq!(a_longerunion.get_usize(7), Some(189_usize));
        assert_eq!(a_longerunion.get_usize(8), Some(88_usize));
        assert_eq!(a_longerunion.get_usize(9), Some(136_usize));
        assert_eq!(a_longerunion.get_usize(10), Some(206_usize));
        assert_eq!(a_longerunion.get_usize(11), Some(126_usize));
        assert_eq!(a_longerunion.get_usize(12), Some(26_usize));
        assert_eq!(a_longerunion.get_usize(13), Some(59_usize));
        assert_eq!(a_longerunion.get_usize(14), Some(18_usize));
        assert_eq!(a_longerunion.get_usize(15), Some(255_usize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_usize);
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        let b0 = a_intunion.set_usize(0, 2895_usize);
        let b1 = a_intunion.set_usize(1, 61770_usize);
        let b2 = a_intunion.set_usize(2, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..2
        {
            match a_intunion.get_usize(i)
            {
                Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_usize(0), Some(2895_usize));
        assert_eq!(a_intunion.get_usize(1), Some(61770_usize));
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        let b0 = a_longunion.set_usize(0, 2895_usize);
        let b1 = a_longunion.set_usize(1, 61770_usize);
        let b2 = a_longunion.set_usize(2, 26869_usize);
        let b3 = a_longunion.set_usize(3, 48547_usize);
        let b4 = a_longunion.set_usize(4, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);
        println!("a_longunion.get() = {}", a_longunion.get());
        for i in 0..4
        {
            match a_longunion.get_ubyte(i)
            {
                Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_usize(0), Some(2895_usize));
        assert_eq!(a_longunion.get_usize(1), Some(61770_usize));
        assert_eq!(a_longunion.get_usize(2), Some(26869_usize));
        assert_eq!(a_longunion.get_usize(3), Some(48547_usize));
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_usize(0, 2895_usize);
        let b1 = a_longerunion.set_usize(1, 61770_usize);
        let b2 = a_longerunion.set_usize(2, 26869_usize);
        let b3 = a_longerunion.set_usize(3, 48547_usize);
        let b4 = a_longerunion.set_usize(4, 34904_usize);
        let b5 = a_longerunion.set_usize(5, 32462_usize);
        let b6 = a_longerunion.set_usize(6, 15130_usize);
        let b7 = a_longerunion.set_usize(7, 65298_usize);
        let b8 = a_longerunion.set_usize(8, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, false);
        println!("a_longerunion.get() = {}", a_longerunion.get());
        for i in 0..8
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(2895_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(61770_usize));
        assert_eq!(a_longerunion.get_usize(2), Some(26869_usize));
        assert_eq!(a_longerunion.get_usize(3), Some(48547_usize));
        assert_eq!(a_longerunion.get_usize(4), Some(34904_usize));
        assert_eq!(a_longerunion.get_usize(5), Some(32462_usize));
        assert_eq!(a_longerunion.get_usize(6), Some(15130_usize));
        assert_eq!(a_longerunion.get_usize(7), Some(65298_usize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        let b0 = a_longunion.set_usize(0, 4048161615_usize);
        let b1 = a_longunion.set_usize(1, 3181603061_usize);
        let b2 = a_longunion.set_usize(2, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        for i in 0..2
        {
            match a_longunion.get_usize(i)
            {
                Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_usize(0), Some(4048161615_usize));
        assert_eq!(a_longunion.get_usize(1), Some(3181603061_usize));
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_usize(0, 4048161615_usize);
        let b1 = a_longerunion.set_usize(1, 3181603061_usize);
        let b2 = a_longerunion.set_usize(2, 2127464536_usize);
        let b3 = a_longerunion.set_usize(3, 4279384858_usize);
        let b4 = a_longerunion.set_usize(4, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);

        for i in 0..4
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(4048161615_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(3181603061_usize));
        assert_eq!(a_longerunion.get_usize(2), Some(2127464536_usize));
        assert_eq!(a_longerunion.get_usize(3), Some(4279384858_usize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_usize(0, 13664881099896654671_usize);
        let b1 = a_longerunion.set_usize(1, 18379818014235068504_usize);
        let b2 = a_longerunion.set_usize(4, 100_usize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);

        for i in 0..2
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_sizeunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_usize(0), Some(13664881099896654671_usize));
        assert_eq!(a_longerunion.get_usize(1), Some(18379818014235068504_usize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    println!("--------------------------------------");
}

fn unions_get_ssize()
{
    println!("unions_get_ssize()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let a_shortunion = ShortUnion::new_with(2895_u16);
        for i in 0..2
        {
            match a_shortunion.get_ssize(i)
            {
                Some(a) => { println!("a_shortunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_shortunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_shortunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_shortunion.get_ssize(2), None);

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..4
        {
            match a_intunion.get_ssize(i)
            {
                Some(a) => { println!("a_intunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_intunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_intunion.get_ssize(2), Some(74_isize));
        assert_eq!(a_intunion.get_ssize(3), Some(-15_isize));
        assert_eq!(a_intunion.get_ssize(4), None);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..8
        {
            match a_longunion.get_ssize(i)
            {
                Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_longunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_longunion.get_ssize(2), Some(74_isize));
        assert_eq!(a_longunion.get_ssize(3), Some(-15_isize));
        assert_eq!(a_longunion.get_ssize(4), Some(-11_isize));
        assert_eq!(a_longunion.get_ssize(5), Some(104_isize));
        assert_eq!(a_longunion.get_ssize(6), Some(-93_isize));
        assert_eq!(a_longunion.get_ssize(7), Some(-67_isize));
        assert_eq!(a_longunion.get_ssize(8), None);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_usize);
        for i in 0..16
        {
            match a_longerunion.get_ssize(i)
            {
                Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_longerunion.get_ssize(2), Some(74_isize));
        assert_eq!(a_longerunion.get_ssize(3), Some(-15_isize));
        assert_eq!(a_longerunion.get_ssize(4), Some(-11_isize));
        assert_eq!(a_longerunion.get_ssize(5), Some(104_isize));
        assert_eq!(a_longerunion.get_ssize(6), Some(-93_isize));
        assert_eq!(a_longerunion.get_ssize(7), Some(-67_isize));
        assert_eq!(a_longerunion.get_ssize(8), Some(88_isize));
        assert_eq!(a_longerunion.get_ssize(9), Some(-120_isize));
        assert_eq!(a_longerunion.get_ssize(10), Some(-50_isize));
        assert_eq!(a_longerunion.get_ssize(11), Some(126_isize));
        assert_eq!(a_longerunion.get_ssize(12), Some(26_isize));
        assert_eq!(a_longerunion.get_ssize(13), Some(59_isize));
        assert_eq!(a_longerunion.get_ssize(14), Some(18_isize));
        assert_eq!(a_longerunion.get_ssize(15), Some(-1_isize));
        assert_eq!(a_longerunion.get_ssize(16), None);
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let a_intunion = IntUnion::new_with(4048161615_u32);
        for i in 0..2
        {
            match a_intunion.get_ssize(i)
            {
                Some(a) => { println!("a_intunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_ssize(0), Some(2895_isize));
        assert_eq!(a_intunion.get_ssize(1), Some(-3766_isize));
        assert_eq!(a_intunion.get_ssize(2), None);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..4
        {
            match a_longunion.get_ssize(i)
            {
                Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_ssize(0), Some(2895_isize));
        assert_eq!(a_longunion.get_ssize(1), Some(-3766_isize));
        assert_eq!(a_longunion.get_ssize(2), Some(26869_isize));
        assert_eq!(a_longunion.get_ssize(3), Some(-16989_isize));
        assert_eq!(a_longunion.get_ssize(4), None);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..8
        {
            match a_longerunion.get_ssize(i)
            {
                Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(2895_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(-3766_isize));
        assert_eq!(a_longerunion.get_ssize(2), Some(26869_isize));
        assert_eq!(a_longerunion.get_ssize(3), Some(-16989_isize));
        assert_eq!(a_longerunion.get_ssize(4), Some(-30632_isize));
        assert_eq!(a_longerunion.get_ssize(5), Some(32462_isize));
        assert_eq!(a_longerunion.get_ssize(6), Some(15130_isize));
        assert_eq!(a_longerunion.get_ssize(7), Some(-238_isize));
        assert_eq!(a_longerunion.get_ssize(8), None);
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        for i in 0..2
        {
            match a_longunion.get_ssize(i)
            {
                Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_ssize(0), Some(-246805681_isize));
        assert_eq!(a_longunion.get_ssize(1), Some(-1113364235_isize));
        assert_eq!(a_longunion.get_ssize(2), None);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..4
        {
            match a_longerunion.get_ssize(i)
            {
                Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(-246805681_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(-1113364235_isize));
        assert_eq!(a_longerunion.get_ssize(2), Some(2127464536_isize));
        assert_eq!(a_longerunion.get_ssize(3), Some(-15582438_isize));
        assert_eq!(a_longerunion.get_ssize(4), None);
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        for i in 0..2
        {
            match a_longerunion.get_ssize(i)
            {
                Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(-4781862973812896945_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(-66926059474483112_isize));
        assert_eq!(a_longerunion.get_sint(4), None);
    }
    println!("--------------------------------------");
}

fn unions_set_ssize()
{
    println!("unions_set_ssize()");
    #[cfg(target_pointer_width = "8")]
    {
        // Example for ShortUnion
        use cryptocol::number::ShortUnion;
        let mut a_shortunion = ShortUnion::new();
        let b0 = a_shortunion.set_ssize(0, 79_isize);
        let b1 = a_shortunion.set_ssize(1, 11_isize);
        let b2 = a_shortunion.set_ssize(2, 100_isize);  // Nothing will be done
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        println!("a_shortunion.get() = {}", a_shortunion.get());
        for i in 0..2
        {
            match a_shortunion.get_ssize(i)
            {
                Some(a) => { println!("a_shortunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_shortunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_shortunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_shortunion.get_ssize(2), None);

        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        let b0 = a_intunion.set_ssize(0, 79_isize);
        let b1 = a_intunion.set_ssize(1, 11_isize);
        let b2 = a_intunion.set_ssize(2, 74_isize);
        let b3 = a_intunion.set_ssize(3, -15_isize);
        let b4 = a_intunion.set_ssize(4, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);
        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..4
        {
            match a_intunion.get_ssize(i)
            {
                Some(a) => { println!("a_intunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_intunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_intunion.get_ssize(2), Some(74_isize));
        assert_eq!(a_intunion.get_ssize(3), Some(-15_isize));
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        let b0 = a_longunion.set_ssize(0, 79_isize);
        let b1 = a_longunion.set_ssize(1, 11_isize);
        let b2 = a_longunion.set_ssize(2, 74_isize);
        let b3 = a_longunion.set_ssize(3, -15_isize);
        let b4 = a_longunion.set_ssize(4, -11_isize);
        let b5 = a_longunion.set_ssize(5, 104_isize);
        let b6 = a_longunion.set_ssize(6, -93_isize);
        let b7 = a_longunion.set_ssize(7, -67_isize);
        let b8 = a_longunion.set_ssize(8, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, false);
        println!("a_longunion.get() = {}", a_longunion.get());
        for i in 0..8
        {
            match a_longunion.get_usize(i)
            {
                Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_longunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_longunion.get_ssize(2), Some(74_isize));
        assert_eq!(a_longunion.get_ssize(3), Some(-15_isize));
        assert_eq!(a_longunion.get_ssize(4), Some(-11_isize));
        assert_eq!(a_longunion.get_ssize(5), Some(104_isize));
        assert_eq!(a_longunion.get_ssize(6), Some(-93_isize));
        assert_eq!(a_longunion.get_ssize(7), Some(-67_isize));
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_ssize(0, 79_isize);
        let b1 = a_longerunion.set_ssize(1, 11_isize);
        let b2 = a_longerunion.set_ssize(2, 74_isize);
        let b3 = a_longerunion.set_ssize(3, -15_isize);
        let b4 = a_longerunion.set_ssize(4, -11_isize);
        let b5 = a_longerunion.set_ssize(5, 104_isize);
        let b6 = a_longerunion.set_ssize(6, -93_isize);
        let b7 = a_longerunion.set_ssize(7, -67_isize);
        let b8 = a_longerunion.set_ssize(8, 88_isize);
        let b9 = a_longerunion.set_ssize(9, -120_isize);
        let b10 = a_longerunion.set_ssize(10, -50_isize);
        let b11 = a_longerunion.set_ssize(11, 126_isize);
        let b12 = a_longerunion.set_ssize(12, 26_isize);
        let b13 = a_longerunion.set_ssize(13, 59_isize);
        let b14 = a_longerunion.set_ssize(14, 18_isize);
        let b15 = a_longerunion.set_ssize(15, -1_isize);
        let b16 = a_longerunion.set_ssize(16, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, true);
        assert_eq!(b9, true);
        assert_eq!(b10, true);
        assert_eq!(b11, true);
        assert_eq!(b12, true);
        assert_eq!(b13, true);
        assert_eq!(b14, true);
        assert_eq!(b15, true);
        assert_eq!(b16, false);
        println!("a_longerunion.get() = {}", a_longerunion.get());
        for i in 0..16
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(79_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(11_isize));
        assert_eq!(a_longerunion.get_ssize(2), Some(74_isize));
        assert_eq!(a_longerunion.get_ssize(3), Some(-15_isize));
        assert_eq!(a_longerunion.get_ssize(4), Some(-11_isize));
        assert_eq!(a_longerunion.get_ssize(5), Some(104_isize));
        assert_eq!(a_longerunion.get_ssize(6), Some(-93_isize));
        assert_eq!(a_longerunion.get_ssize(7), Some(-67_isize));
        assert_eq!(a_longerunion.get_ssize(8), Some(88_isize));
        assert_eq!(a_longerunion.get_ssize(9), Some(-120_isize));
        assert_eq!(a_longerunion.get_ssize(10), Some(-50_isize));
        assert_eq!(a_longerunion.get_ssize(11), Some(126_isize));
        assert_eq!(a_longerunion.get_ssize(12), Some(26_isize));
        assert_eq!(a_longerunion.get_ssize(13), Some(59_isize));
        assert_eq!(a_longerunion.get_ssize(14), Some(18_isize));
        assert_eq!(a_longerunion.get_ssize(15), Some(-1_isize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_usize);
    }
    #[cfg(target_pointer_width = "16")]
    {
        // Example for IntUnion
        use cryptocol::number::IntUnion;
        let mut a_intunion = IntUnion::new();
        let b0 = a_intunion.set_ssize(0, 2895_isize);
        let b1 = a_intunion.set_ssize(1, -3766_isize);
        let b2 = a_intunion.set_ssize(2, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        println!("a_intunion.get() = {}", a_intunion.get());
        for i in 0..2
        {
            match a_intunion.get_ssize(i)
            {
                Some(a) => { println!("a_intunion.set_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_intunion.get_ssize(0), Some(2895_isize));
        assert_eq!(a_intunion.get_ssize(1), Some(-3766_isize));
        assert_eq!(a_intunion.get(), 4048161615_u32);

        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        let b0 = a_longunion.set_ssize(0, 2895_isize);
        let b1 = a_longunion.set_ssize(1, -3766_isize);
        let b2 = a_longunion.set_ssize(2, 26869_isize);
        let b3 = a_longunion.set_ssize(3, -16989_isize);
        let b4 = a_longunion.set_ssize(4, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);
        println!("a_longunion.get() = {}", a_longunion.get());
        for i in 0..4
        {
            match a_longunion.get_ssize(i)
            {
                Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_ssize(0), Some(2895_isize));
        assert_eq!(a_longunion.get_ssize(1), Some(-3766_isize));
        assert_eq!(a_longunion.get_ssize(2), Some(26869_isize));
        assert_eq!(a_longunion.get_ssize(3), Some(-16989_isize));
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_ssize(0, 2895_isize);
        let b1 = a_longerunion.set_ssize(1, -3766_isize);
        let b2 = a_longerunion.set_ssize(2, 26869_isize);
        let b3 = a_longerunion.set_ssize(3, -16989_isize);
        let b4 = a_longerunion.set_ssize(4, -30632_isize);
        let b5 = a_longerunion.set_ssize(5, 32462_isize);
        let b6 = a_longerunion.set_ssize(6, 15130_isize);
        let b7 = a_longerunion.set_ssize(7, -238_isize);
        let b8 = a_longerunion.set_ssize(8, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, true);
        assert_eq!(b5, true);
        assert_eq!(b6, true);
        assert_eq!(b7, true);
        assert_eq!(b8, false);
        println!("a_longerunion.get() = {}", a_longerunion.get());
        for i in 0..8
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(2895_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(-3766_isize));
        assert_eq!(a_longerunion.get_ssize(2), Some(26869_isize));
        assert_eq!(a_longerunion.get_ssize(3), Some(-16989_isize));
        assert_eq!(a_longerunion.get_ssize(4), Some(-30632_isize));
        assert_eq!(a_longerunion.get_ssize(5), Some(32462_isize));
        assert_eq!(a_longerunion.get_ssize(6), Some(15130_isize));
        assert_eq!(a_longerunion.get_ssize(7), Some(-238_isize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "32")]
    {
        // Example for LongUnion
        use cryptocol::number::LongUnion;
        let mut a_longunion = LongUnion::new();
        let b0 = a_longunion.set_ssize(0, -246805681_isize);
        let b1 = a_longunion.set_ssize(1, -1113364235_isize);
        let b2 = a_longunion.set_ssize(2, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);
        for i in 0..2
        {
            match a_longunion.get_ssize(i)
            {
                Some(a) => { println!("a_longunion.set_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longunion.get_ssize(0), Some(-246805681_isize));
        assert_eq!(a_longunion.get_ssize(1), Some(-1113364235_isize));
        assert_eq!(a_longunion.get(), 13664881099896654671_u64);

        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_ssize(0, -246805681_isize);
        let b1 = a_longerunion.set_ssize(1, -1113364235_isize);
        let b2 = a_longerunion.set_ssize(2, 2127464536_isize);
        let b3 = a_longerunion.set_ssize(3, -15582438_isize);
        let b4 = a_longerunion.set_ssize(4, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, true);
        assert_eq!(b3, true);
        assert_eq!(b4, false);

        for i in 0..4
        {
            match a_longerunion.get_usize(i)
            {
                Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(-246805681_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(-1113364235_isize));
        assert_eq!(a_longerunion.get_ssize(2), Some(2127464536_isize));
        assert_eq!(a_longerunion.get_ssize(3), Some(-15582438_isize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Example for LongerUnion
        use cryptocol::number::LongerUnion;
        let mut a_longerunion = LongerUnion::new();
        let b0 = a_longerunion.set_ssize(0, -4781862973812896945_isize);
        let b1 = a_longerunion.set_ssize(1, -66926059474483112_isize);
        let b2 = a_longerunion.set_ssize(4, 100_isize);
        assert_eq!(b0, true);
        assert_eq!(b1, true);
        assert_eq!(b2, false);

        for i in 0..2
        {
            match a_longerunion.get_ssize(i)
            {
                Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
                _ => {},
            }
        }
        assert_eq!(a_longerunion.get_ssize(0), Some(-4781862973812896945_isize));
        assert_eq!(a_longerunion.get_ssize(1), Some(-66926059474483112_isize));
        assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
    }
    println!("--------------------------------------");
}


fn unions_add_main()
{
    unions_carrying_add();
    unions_wrapping_add();
    unions_overflowing_add();
    unions_checked_add();
    unions_unchecked_add();
    unions_saturating_add();
}

fn unions_carrying_add()
{
    println!("unions_carrying_add()");
    // Example for ShortUnion
    use cryptocol::number::ShortUnion;
    // a_intunion: IntUnion === (a_high_shortunion, a_low_shortunion) === (10000_u16, 10100_u16) === 655370100_u32
    let a_high_shortunion = ShortUnion::new_with(10000_u16);
    let a_low_shortunion = ShortUnion::new_with(10100_u16);
    // b_shortunion: IntUnion === (b_high_shortunion, b_low_shortunion) === (10000_u16, 20000_u16) === 3276830000_u32
    let b_high_shortunion = ShortUnion::new_with(50000_u16);
    let b_low_shortunion = ShortUnion::new_with(30000_u16);

    // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 655370100_u32 + 3276830000_u32 == 3932200100_u32
    //    655370100_u32 == (10000_u16, 10100_u16)
    // + 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   3932200100_u32 == (60000_u16, 40100_u16)

    // c_u32: u32 === (c_high_shortunion, c_low_shortunion)
    let (c_low_shortunion, carry) = a_low_shortunion.carrying_add(b_low_shortunion, false);
    let (c_high_shortunion, carry) = a_high_shortunion.carrying_add(b_high_shortunion, carry);
    println!("{}-{}, {}", c_high_shortunion, c_low_shortunion, carry);
    assert_eq!(c_high_shortunion.get(), 60000_u16);
    assert_eq!(c_low_shortunion.get(), 40100_u16);
    assert_eq!(carry, false);

    // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 3932200100_u32 + 3276830000_u32 == 51501_u16
    //   3932200100_u32 == (60000_u16, 40100_u16)
    // + 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   2914062804_u32 == (44465_u16,  4564_u16)

    // d: u32 === (d_high_shortunion, d_low_shortunion)
    let (d_low_shortunion, carry) = c_low_shortunion.carrying_add(b_low_shortunion, false);
    let (d_high_shortunion, carry) = c_high_shortunion.carrying_add(b_high_shortunion, carry);
    println!("{}-{}, {}", d_high_shortunion, d_low_shortunion, carry);
    assert_eq!(d_high_shortunion.get(), 44465_u16);
    assert_eq!(d_low_shortunion.get(), 4564_u16);
    assert_eq!(carry, true);

    // Example for IntUnion
    use cryptocol::number::IntUnion;
    //  1234567890123456789_u64 == ( 287445236_u32, 2112454933_u32)
    //+ 9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    //---------------------------------------------------------------------------------------------------------------------
    // 11111111100246913578_u64 == (2587007149_u32,  773714474_u32)

    // a: u64 === (a_high_intunion, a_low_intunion)
    let (a_low_intunion, carry) = IntUnion::new_with(2112454933_u32).carrying_add(IntUnion::new_with(2956226837_u32), false);
    let (a_high_intunion, carry) = IntUnion::new_with(287445236_u32).carrying_add(IntUnion::new_with(2299561912_u32), carry);
    println!("{}-{}, {}", a_high_intunion, a_low_intunion, carry);
    assert_eq!(a_high_intunion.get(), 2587007149_u32);
    assert_eq!(a_low_intunion.get(), 773714474_u32);
    assert_eq!(carry, false);

    //  11111111100246913578_u64 == (2587007149_u32,  773714474_u32)
    //+  9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    //--------------------------------------------------------------
    //   2540910236660818751_u64 == ( 591601765_u32, 3729941311_u32)

    // b: u64 === (b_high_intunion, b_low_intunion)
    let (b_low_intunion, carry) = IntUnion::new_with(773714474_u32).carrying_add(IntUnion::new_with(2956226837_u32), false);
    let (b_high_intunion, carry) = IntUnion::new_with(2587007149_u32).carrying_add(IntUnion::new_with(2299561912_u32), carry);
    println!("{}-{}, {}", b_high_intunion, b_low_intunion, carry);
    assert_eq!(b_high_intunion.get(), 591601765_u32);
    assert_eq!(b_low_intunion.get(), 3729941311_u32);
    assert_eq!(carry, true);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    // a_longerunion: LongerUnion === (a_high_longunion, a_low_longunion) === (6692605942763486917_u64, 12312739301371248917_u64) === 322222221211111111100000000088888888987_u128
    let a_high_longunion = LongUnion::new_with(6692605942763486917_u64);
    let a_low_longunion = LongUnion::new_with(12312739301371248917_u64);
    // b_longunion: LongerUnion === (b_high_longunion, b_low_longunion) === (10775095670246085798_u64, 7681743649119882630_u64) === 198765432198765432198765432198765432198_u128
    let b_high_longunion = LongUnion::new_with(10775095670246085798_u64);
    let b_low_longunion = LongUnion::new_with(7681743649119882630_u64);

    // (6692605942763486917_u64, 12312739301371248917_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
    //   123456789012345678901234567890123456789_u128 == (6692605942763486917_u64, 12312739301371248917_u64)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
    // -----------------------------------------------------------------------------------------------------
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)

    // c_u128: u128 === (c_high_longunion, c_low_longunion)
    let (c_low_longunion, carry) = a_low_longunion.carrying_add(b_low_longunion, false);
    let (c_high_longunion, carry) = a_high_longunion.carrying_add(b_high_longunion, carry);
    println!("{}-{}, {}", c_high_longunion, c_low_longunion, carry);
    assert_eq!(c_high_longunion.get(), 17467701613009572716_u64);
    assert_eq!(c_low_longunion.get(), 1547738876781579931_u64);
    assert_eq!(carry, false);

    // (17467701613009572716_u64, 1547738876781579931_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_u64
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
    // ------------------------------------------
    //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_u64, 9229482525901462561_u64)

    // d: u128 === (d_high_longunion, d_low_longunion)
    let (d_low_longunion, carry) = c_low_longunion.carrying_add(b_low_longunion, false);
    let (d_high_longunion, carry) = c_high_longunion.carrying_add(b_high_longunion, carry);
    println!("{}-{}, {}", d_high_longunion, d_low_longunion, carry);
    assert_eq!(d_high_longunion.get(), 9796053209546106898_u64);
    assert_eq!(d_low_longunion.get(), 9229482525901462561_u64);
    assert_eq!(carry, true);

    // Example for LongerUnion
    use cryptocol::number::LongerUnion;
    //  4201016837757989640311993609423984479246482890531986660185_u256 == (12345678901234567890_u128, 6789012345678912345_u128)
    //+                 419908440780438063913804265570801972943493_u256 == (                1234_u128,                6789_u128)
    //---------------------------------------------------------------------------------------------------------------------
    //  4201016837757990060220434389862048393050748461333959603678_u256 == (12345678901234569124_u128, 6789012345678919134_u128)

    // a: u256 === (a_high_longerunion, a_low_longerunion)
    let (a_low_longerunion, carry) = LongerUnion::new_with(6789012345678912345_u128).carrying_add(LongerUnion::new_with(6789_u128), false);
    let (a_high_longerunion, carry) = LongerUnion::new_with(12345678901234567890_u128).carrying_add(LongerUnion::new_with(1234_u128), carry);
    println!("{}-{}, {}", a_high_longerunion, a_low_longerunion, carry);
    assert_eq!(a_high_longerunion.get(), 12345678901234569124_u128);
    assert_eq!(a_low_longerunion.get(), 6789012345678919134_u128);
    assert_eq!(carry, false);

    //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)

    // b: u256 === (b_high_longerunion, b_low_longerunion)
    let (b_low_longerunion, carry) = LongerUnion::new_with(56789012345678912345678901234567890123_u128).carrying_add(LongerUnion::new_with(12345678901234567890123456789012345678_u128), false);
    let (b_high_longerunion, carry) = LongerUnion::new_with(226854911280625642308916404954512140970_u128).carrying_add(LongerUnion::new_with(170141183460469231731687303715884105727_u128), carry);
    println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, carry);
    assert_eq!(b_high_longerunion.get(), 56713727820156410577229101238628035241_u128);
    assert_eq!(b_low_longerunion.get(), 69134691246913480235802358023580235801_u128);
    assert_eq!(carry, true);

    // Example for SizeUnion for 64-bit CPU
    use cryptocol::number::SizeUnion;
    // a_longerunion: LongerUnion === (a_high_sizeunion, a_low_sizeunion) === (6692605942763486917_usize, 12312739301371248917_usize) === 322222221211111111100000000088888888987_u128
    let a_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
    let a_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);
    // b_sizeunion: LongerUnion === (b_high_sizeunion, b_low_sizeunion) === (10775095670246085798_usize, 7681743649119882630_usize) === 198765432198765432198765432198765432198_u128
    let b_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
    let b_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);

    // (6692605942763486917_usize, 12312739301371248917_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
    //   123456789012345678901234567890123456789_u128 == (6692605942763486917_usize, 12312739301371248917_usize)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize, 7681743649119882630_usize)
    // ---------------------------------------------------------------------------------------------------------
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize, 1547738876781579931_usize)

    // c_u128: u128 === (c_high_sizeunion, c_low_sizeunion)
    let (c_low_sizeunion, carry) = a_low_sizeunion.carrying_add(b_low_sizeunion, false);
    let (c_high_sizeunion, carry) = a_high_sizeunion.carrying_add(b_high_sizeunion, carry);
    println!("{}-{}, {}", c_high_sizeunion, c_low_sizeunion, carry);
    assert_eq!(c_high_sizeunion.get(), 17467701613009572716_usize);
    assert_eq!(c_low_sizeunion.get(), 1547738876781579931_usize);
    assert_eq!(carry, false);

    // (17467701613009572716_usize, 1547738876781579931_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_usize
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize, 1547738876781579931_usize)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize, 7681743649119882630_usize)
    // ---------------------------------------------------------------------------------------------------------
    //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_usize, 9229482525901462561_usize)

    // d: u128 === (d_high_sizeunion, d_low_sizeunion)
    let (d_low_sizeunion, carry) = c_low_sizeunion.carrying_add(b_low_sizeunion, false);
    let (d_high_sizeunion, carry) = c_high_sizeunion.carrying_add(b_high_sizeunion, carry);
    println!("{}-{}, {}", d_high_sizeunion, d_low_sizeunion, carry);
    assert_eq!(d_high_sizeunion.get(), 9796053209546106898_usize);
    assert_eq!(d_low_sizeunion.get(), 9229482525901462561_usize);
    assert_eq!(carry, true);
    println!("--------------------------------------");
}

fn unions_wrapping_add()
{
    println!("unions_wrapping_add()");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
    let b_shortunion = ShortUnion::new_with(55);
    let c_shortunion = a_shortunion.wrapping_add(b_shortunion);
    println!("{} + {} = {}", a_shortunion, b_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), u16::MAX);

    let d_shortunion = c_shortunion.wrapping_add(1_u16.into_shortunion());
    println!("{} + 1 = {}", a_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), 0_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
    let b_intunion = IntUnion::new_with(55);
    let c_intunion = a_intunion.wrapping_add(b_intunion);
    println!("{} + {} = {}", a_intunion, b_intunion, c_intunion);
    assert_eq!(c_intunion.get(), u32::MAX);

    let d_intunion = c_intunion.wrapping_add(1_u32.into_intunion());
    println!("{} + 1 = {}", a_intunion, d_intunion);
    assert_eq!(d_intunion.get(), 0_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
    let b_longunion = LongUnion::new_with(55);
    let c_longunion = a_longunion.wrapping_add(b_longunion);
    println!("{} + {} = {}", a_longunion, b_longunion, c_longunion);
    assert_eq!(c_longunion.get(), u64::MAX);

    let d_longunion = c_longunion.wrapping_add(1_u32.into_longunion());
    println!("{} + 1 = {}", a_longunion, d_longunion);
    assert_eq!(d_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let b_longerunion = LongerUnion::new_with(55);
    let c_longerunion = a_longerunion.wrapping_add(b_longerunion);
    println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    let d_longerunion = c_longerunion.wrapping_add(1_u128.into_longerunion());
    println!("{} + 1 = {}", a_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
    let b_sizeunion = SizeUnion::new_with(55);
    let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
    println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), usize::MAX);

    let d_sizeunion = c_sizeunion.wrapping_add(1_usize.into_sizeunion());
    println!("{} + 1 = {}", a_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn unions_overflowing_add()
{
    println!("unions_overflowing_add");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
    let (b_shortunion, overflow) = a_shortunion.overflowing_add(55_u16.into_shortunion());
    println!("{} + 55 = {}\nOverflow = {}", a_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), u16::MAX);
    assert_eq!(overflow, false);

    let (c_shortunion, overflow) = b_shortunion.overflowing_add(1_u16.into_shortunion());
    println!("{} + 1 = {}\nOverflow = {}", b_shortunion, c_shortunion, overflow);
    assert_eq!(c_shortunion.get(), 0_u16);
    assert_eq!(overflow, true);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
    let (b_intunion, overflow) = a_intunion.overflowing_add(55_u32.into_intunion());
    println!("{} + 55 = {}\nOverflow = {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.get(), u32::MAX);
    assert_eq!(overflow, false);
 
    let (c_intunion, overflow) = b_intunion.overflowing_add(1_u32.into_intunion());
    println!("{} + 1 = {}\nOverflow = {}", b_intunion, c_intunion, overflow);
    assert_eq!(c_intunion.get(), 0_u32);
    assert_eq!(overflow, true);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
    let (b_longunion, overflow) = a_longunion.overflowing_add(55_u64.into_longunion());
    println!("{} + 55 = {}\nOverflow = {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.get(), u64::MAX);
    assert_eq!(overflow, false);
 
    let (c_longunion, overflow) = b_longunion.overflowing_add(1_u64.into_longunion());
    println!("{} + 1 = {}\nOverflow = {}", b_longunion, c_longunion, overflow);
    assert_eq!(c_longunion.get(), 0_u64);
    assert_eq!(overflow, true);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let (b_longerunion, overflow) = a_longerunion.overflowing_add(55_u128.into_longerunion());
    println!("{} + 55 = {}\nOverflow = {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), u128::MAX);
    assert_eq!(overflow, false);
 
    let (c_longerunion, overflow) = b_longerunion.overflowing_add(1_u128.into_longerunion());
    println!("{} + 1 = {}\nOverflow = {}", b_longerunion, c_longerunion, overflow);
    assert_eq!(c_longerunion.into_u128(), 0_u128);
    assert_eq!(overflow, true);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
    let (b_sizeunion, overflow) = a_sizeunion.overflowing_add(55_usize.into_sizeunion());
    println!("{} + 55 = {}\nOverflow = {}", a_sizeunion, b_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    assert_eq!(overflow, false);
 
    let (c_sizeunion, overflow) = b_sizeunion.overflowing_add(1_usize.into_sizeunion());
    println!("{} + 1 = {}\nOverflow = {}", b_sizeunion, c_sizeunion, overflow);
    assert_eq!(c_sizeunion.get(), 0_usize);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}

fn unions_checked_add()
{
    println!("unions_checked_add");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
    let b_shortunion = 55_u16.into_shortunion();
    let c_shortunion = a_shortunion.checked_add(b_shortunion);
    match c_shortunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_shortunion, c);
                assert_eq!(c.get(), u16::MAX);
            },
        None => { println!("Overflow happened."); },
    }

    let d_shortunion = c_shortunion.unwrap().checked_add(1_u16.into_shortunion());
    match d_shortunion
    {
        Some(d) => { println!("{} + 1 = {}", a_shortunion, d); },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
    let b_intunion = 55_u32.into_intunion();
    let c_intunion = a_intunion.checked_add(b_intunion);
    match c_intunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_intunion, c);
                assert_eq!(c.get(), u32::MAX);
            },
        None => { println!("Overflow happened."); },
    }

    let d_intunion = c_intunion.unwrap().checked_add(1_u32.into_intunion());
    match d_intunion
    {
        Some(d) => { println!("{} + 1 = {}", a_intunion, d); },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
    let b_longunion = 55_u64.into_longunion();
    let c_longunion = a_longunion.checked_add(b_longunion);
    match c_longunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_longunion, c);
                assert_eq!(c.get(), u64::MAX);
            },
        None => { println!("Overflow happened."); },
    }

    let d_longunion = c_longunion.unwrap().checked_add(1_u64.into_longunion());
    match d_longunion
    {
        Some(d) => { println!("{} + 1 = {}", a_longunion, d); },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let b_longerunion = 55_u128.into_longerunion();
    let c_longerunion = a_longerunion.checked_add(b_longerunion);
    match c_longerunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_longerunion, c);
                assert_eq!(c.get(), u128::MAX);
            },
        None => { println!("Overflow happened."); },
    }

    let d_longerunion = c_longerunion.unwrap().checked_add(1_u128.into_longerunion());
    match d_longerunion
    {
        Some(d) => { println!("{} + 1 = {}", a_longerunion, d); },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
    let b_sizeunion = 55_usize.into_sizeunion();
    let c_sizeunion = a_sizeunion.checked_add(b_sizeunion);
    match c_sizeunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_sizeunion, c);
                assert_eq!(c.get(), usize::MAX);
            },
        None => { println!("Overflow happened."); },
    }

    let d_sizeunion = c_sizeunion.unwrap().checked_add(1_usize.into_sizeunion());
    match d_sizeunion
    {
        Some(d) => { println!("{} + 1 = {}", a_sizeunion, d); },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_sizeunion, None);
            },
    }
    println!("--------------------------------------");
}

fn unions_unchecked_add()
{
    println!("unions_unchecked_add");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
    let b_shortunion = ShortUnion::new_with(55_u16);
    let c_shortunion = a_shortunion.saturating_add(b_shortunion);
    println!("{} + 55 = {}", a_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), u16::MAX);

    // It will panic
    // let d_shortunion = small_uint_unchecked_add_func(c_shortunion, 1_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
    let b_intunion = IntUnion::new_with(55_u32);
    let c_intunion = a_intunion.saturating_add(b_intunion);
    println!("{} + 55 = {}", a_intunion, c_intunion);
    assert_eq!(c_intunion.get(), u32::MAX);

    // It will panic
    // let d_intunion = small_uint_unchecked_add_func(c_intunion, 1_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
    let b_longunion = LongUnion::new_with(55_u64);
    let c_longunion = a_longunion.saturating_add(b_longunion);
    println!("{} + 55 = {}", a_longunion, c_longunion);
    assert_eq!(c_longunion.get(), u64::MAX);

    // It will panic
    // let d_longunion = small_uint_unchecked_add_func(c_longunion, 1_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let b_longerunion = LongerUnion::new_with(55_u128);
    let c_longerunion = a_longerunion.saturating_add(b_longerunion);
    println!("{} + 55 = {}", a_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    // It will panic
    // let d_longerunion = small_uint_unchecked_add_func(c_longerunion, 1_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
    let b_sizeunion = SizeUnion::new_with(55_usize);
    let c_sizeunion = a_sizeunion.saturating_add(b_sizeunion);
    println!("{} + 55 = {}", a_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), usize::MAX);

    // It will panic
    // let d_sizeunion = small_uint_unchecked_add_func(c_sizeunion, 1_usize);
    println!("--------------------------------------");
}

fn unions_saturating_add()
{
    println!("unions_saturating_add");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
    let b_shortunion = ShortUnion::new_with(55_u16);
    let c_shortunion = a_shortunion.saturating_add(b_shortunion);
    println!("{} + 55 = {}", a_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), u16::MAX);

    let d_shortunion = c_shortunion.saturating_add(b_shortunion);
    println!("{} + 55 = {}", c_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
    let b_intunion = IntUnion::new_with(55_u32);
    let c_intunion = a_intunion.saturating_add(b_intunion);
    println!("{} + 55 = {}", a_intunion, c_intunion);
    assert_eq!(c_intunion.get(), u32::MAX);

    let d_intunion = c_intunion.saturating_add(b_intunion);
    println!("{} + 55 = {}", c_intunion, d_intunion);
    assert_eq!(d_intunion.get(), u32::MAX);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
    let b_longunion = LongUnion::new_with(55_u64);
    let c_longunion = a_longunion.saturating_add(b_longunion);
    println!("{} + 55 = {}", a_longunion, c_longunion);
    assert_eq!(c_longunion.get(), u64::MAX);

    let d_longunion = c_longunion.saturating_add(b_longunion);
    println!("{} + 55 = {}", c_longunion, d_longunion);
    assert_eq!(d_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let b_longerunion = LongerUnion::new_with(55_u128);
    let c_longerunion = a_longerunion.saturating_add(b_longerunion);
    println!("{} + 55 = {}", a_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    let d_longerunion = c_longerunion.saturating_add(b_longerunion);
    println!("{} + 55 = {}", c_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
    let b_sizeunion = SizeUnion::new_with(55_usize);
    let c_sizeunion = a_sizeunion.saturating_add(b_sizeunion);
    println!("{} + 55 = {}", a_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), usize::MAX);

    let d_sizeunion = c_sizeunion.saturating_add(b_sizeunion);
    println!("{} + 55 = {}", c_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), usize::MAX);
    println!("--------------------------------------");
}


fn unions_sub_main()
{
    unions_borrowing_sub();
    unions_wrapping_sub();
    unions_overflowing_sub();
    unions_checked_sub();
    unions_unchecked_sub();
    unions_saturating_sub();
    unions_abs_diff();
}

fn unions_borrowing_sub()
{
    println!("unions_borrowing_sub");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

    // Example for ShortUnion
    // a_u32: u32 === (a_high_shortunion, a_low_shortunion) == (50000_u16, 30000_u16) == 3276830000_u32
    let a_high_shortunion = ShortUnion::new_with(50000_u16);
    let a_low_shortunion = ShortUnion::new_with(30000_u16);
    // b_u32: u32 === (b_high_shortunion, b_low_shortunion) == (10000_u16, 10100_u16) == 655370100_u32
    let b_high_shortunion = ShortUnion::new_with(10000_u16);
    let b_low_shortunion = ShortUnion::new_with(10100_u16);

    // (50000_u16, 30000_u16) - (10000_u16, 10100_u16) == 3276830000_u32 - 655370100_u32 == 99_u16
    //   3276830000_u32 == (50000_u16, 30000_u16)
    // -  655370100_u32 == (10000_u16, 10100_u16)
    // ------------------------------------------
    //   2621459900_u32 == (40000_u16, 19900_u16)

    // c: u32 === (c_high_shortunion, c_low_shortunion)
    let (c_low_shortunion, borrow) = a_low_shortunion.borrowing_sub(b_low_shortunion, false);
    let (c_high_shortunion, borrow) = a_high_shortunion.borrowing_sub(b_high_shortunion, borrow);
    println!("{}-{}, {}", c_high_shortunion, c_low_shortunion, borrow);
    assert_eq!(c_high_shortunion.get(), 40000_u16);
    assert_eq!(c_low_shortunion.get(), 19900_u16);
    assert_eq!(borrow, false);

    // (10000_u16, 10100_u16) - (50000_u16, 30000_u16) == 655370100_u32 - 3276830000_u32 == 51501_u16
    //    655370100_u32 == (10000_u16, 10100_u16)
    // - 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   1673507396_u32 == (25535_u16, 45636_u16)

    // d: u32 === (d_high_shortunion, d_low_shortunion)
    let (d_low_shortunion, borrow) = b_low_shortunion.borrowing_sub(a_low_shortunion, false);
    let (d_high_shortunion, borrow) = b_high_shortunion.borrowing_sub(a_high_shortunion, borrow);
    println!("{}-{}, {}", d_high_shortunion, d_low_shortunion, borrow);
    assert_eq!(d_high_shortunion.get(), 25535_u16);
    assert_eq!(d_low_shortunion.get(), 45636_u16);
    assert_eq!(borrow, true);

    // Example for IntUnion
    // a_u64: u64 === (a_high_intunion, a_low_intunion) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_intunion = IntUnion::new_with(2299561912_u32);
    let a_low_intunion = IntUnion::new_with(2956226837_u32);
    // b_u64: u64 === (b_high_intunion, b_low_intunion) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_intunion = IntUnion::new_with(1782160508_u32);
    let b_low_intunion = IntUnion::new_with(682685733_u32);

    // (2299561912_u32, 2956226837_u32) - (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 - 7654321098765432101_u64 == 2222222111358024688_u64
    //   9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
    // -------------------------------------------------------------
    //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)

    // c: u64 === (c_high_intunion, c_low_intunion)
    let (c_low_intunion, borrow) = a_low_intunion.borrowing_sub(b_low_intunion, false);
    let (c_high_intunion, borrow) = a_high_intunion.borrowing_sub(b_high_intunion, borrow);
    println!("{}-{}, {}", c_high_intunion, c_low_intunion, borrow);
    assert_eq!(c_high_intunion.get(), 517401404_u32);
    assert_eq!(c_low_intunion.get(), 2273541104_u32);
    assert_eq!(borrow, false);

    // (517401404_u32, 2273541104_u32) - (1782160508_u32,  682685733_u32) == 2222222111358024688_u32 - 7654321098765432101_u32 == 13014645086302144203_u16
    //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)
    // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
    // -------------------------------------------------------------
    //  13014645086302144203_u64 == (3030208192_u32, 1590855371_u32)

    // d: u64 === (d_high_intunion, d_low_intunion)
    let (d_low_intunion, borrow) = c_low_intunion.borrowing_sub(b_low_intunion, false);
    let (d_high_intunion, borrow) = c_high_intunion.borrowing_sub(b_high_intunion, borrow);
    println!("{}-{}, {}", d_high_intunion, d_low_intunion, borrow);
    assert_eq!(d_high_intunion.get(), 3030208192_u32);
    assert_eq!(d_low_intunion.get(), 1590855371_u32);
    assert_eq!(borrow, true);

    // Example for LongUnion
    // a_u128: u128 === (a_high_longunion, a_low_longunion) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_longunion = LongUnion::new_with(10775095670246085798_u64);
    let a_low_longunion = LongUnion::new_with(7681743649119882630_u64);
    // b_u128: u128 === (b_high_longunion, b_low_longunion) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_longunion = LongUnion::new_with(6692605942763486917_u64);
    let b_low_longunion = LongUnion::new_with(12312739301371248917_u64);

    // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
    //   198765432198765432198765432198765432198_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // ------------------------------------------------------------------------------------------------------
    //    75308643186419753297530864308641975409_u128 == (10775095670246085798_u64,  7681743649119882630_u64)

    // c: u32 === (c_high_u16, c_low_u16)
    let (c_low_longunion, borrow) = a_low_longunion.borrowing_sub(b_low_longunion, false);
    let (c_high_longunion, borrow) = a_high_longunion.borrowing_sub(b_high_longunion, borrow);
    println!("{}-{}, {}", c_high_longunion, c_low_longunion, borrow);
    assert_eq!(c_high_longunion.get(), 4082489727482598880_u64);
    assert_eq!(c_low_longunion.get(), 13815748421458185329_u64);
    assert_eq!(borrow, false);

    // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
    //    75308643186419753297530864308641975409_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // ------------------------------------------------------------------------------------------------------
    //   292134221095012537859670903850286730076_u128 == (15836627858428663579_u64,  1503009120086936412_u64)

    // d: u128 === (d_high_u64, d_low_u64)
    let (d_low_longunion, borrow) = c_low_longunion.borrowing_sub(b_low_longunion, false);
    let (d_high_longunion, borrow) = c_high_longunion.borrowing_sub(b_high_longunion, borrow);
    println!("{}-{}, {}", d_high_longunion, d_low_longunion, borrow);
    assert_eq!(d_high_longunion.get(), 15836627858428663579_u64);
    assert_eq!(d_low_longunion.get(), 1503009120086936412_u64);
    assert_eq!(borrow, true);

    // Example for LongerUnion
    //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    // ---------------------------------------------------------------------------------------------------------------------
    //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)

    // a_u256: u256 === (a_high_longerunion, a_low_longerunion)
    let (a_low_longerunion, borrow) = LongerUnion::new_with(6789012345678912345_u128).borrowing_sub(LongerUnion::new_with(6789_u128), false);
    let (a_high_longerunion, borrow) = LongerUnion::new_with(12345678901234567890_u128).borrowing_sub(LongerUnion::new_with(1234_u128), borrow);
    println!("{}-{}, {}", a_low_longerunion, a_high_longerunion, borrow);
    assert_eq!(a_high_longerunion.get(), 12345678901234566656_u128);
    assert_eq!(a_low_longerunion.get(), 6789012345678905556_u128);
    assert_eq!(borrow, false);

    //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128,  12345678901234567890123456789012345678_u128)
    // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128,  56789012345678912345678901234567890123_u128)
    // --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   328077586172395887033451124191282405584107085763563507612853141042164389031555 == (283568639100782052886145506193140176212_u128, 295839033476494119007819162986212667011_u128)

    // b_u256: u256 === (b_high_longerunion, b_low_longerunion)
    let (b_low_longerunion, borrow) = LongerUnion::new_with(12345678901234567890123456789012345678_u128).borrowing_sub(LongerUnion::new_with(56789012345678912345678901234567890123_u128), false);
    let (b_high_longerunion, borrow) = LongerUnion::new_with(170141183460469231731687303715884105727_u128).borrowing_sub(LongerUnion::new_with(226854911280625642308916404954512140970_u128), borrow);
    println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, borrow);
    assert_eq!(b_high_longerunion.get(), 283568639100782052886145506193140176212_u128);
    assert_eq!(b_low_longerunion.get(), 295839033476494119007819162986212667011_u128);
    assert_eq!(borrow, true);

    // Example for SizeUnion for 64-bit CPU
    // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
    let a_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
    let a_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
    // b_u128: u128 === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
    let b_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
    let b_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);

    // (10775095670246085798_usize, 7681743649119882630_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
    //   198765432198765432198765432198765432198_u128 == (10775095670246085798_usize,  7681743649119882630_usize)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // ------------------------------------------------------------------------------------------------------
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)

    // c: u128 === (c_high_usize, c_low_usize)
    let (c_low_sizeunion, borrow) = a_low_sizeunion.borrowing_sub(b_low_sizeunion, false);
    let (c_high_sizeunion, borrow) = a_high_sizeunion.borrowing_sub(b_high_sizeunion, borrow);
    println!("{}-{}, {}", c_high_sizeunion, c_low_sizeunion, borrow);
    assert_eq!(c_high_sizeunion.get(), 4082489727482598880_usize);
    assert_eq!(c_low_sizeunion.get(), 13815748421458185329_usize);
    assert_eq!(borrow, false);

    // (4082489727482598880_usize, 13815748421458185329_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // ------------------------------------------------------------------------------------------------------
    //   292134221095012537859670903850286730076_u128 == (14364254346226952735_usize,  4630995652251366287_usize)

    // d: u128 === (d_high_usize, d_low_usize)
    let (d_low_sizeunion, borrow) = c_low_sizeunion.borrowing_sub(b_low_sizeunion, false);
    let (d_high_sizeunion, borrow) = c_high_sizeunion.borrowing_sub(b_high_sizeunion, borrow);
    println!("{}-{}, {}", d_high_sizeunion, d_low_sizeunion, borrow);
    assert_eq!(d_high_sizeunion.get(), 15836627858428663579_usize);
    assert_eq!(d_low_sizeunion.get(), 1503009120086936412_usize);
    assert_eq!(borrow, true);
    println!("--------------------------------------");
}

fn unions_wrapping_sub()
{
    println!("unions_wrapping_sub");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(55_u16).wrapping_sub(ShortUnion::new_with(55_u16));
    println!("55 - 55 = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 0_u16);

    let b_shortunion = a_shortunion.wrapping_sub(ShortUnion::new_with(1_u16));
    println!("{} - 1 = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(55_u32).wrapping_sub(IntUnion::new_with(55_u32));
    println!("55 - 55 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 0_u32);

    let b_intunion = a_intunion.wrapping_sub(IntUnion::new_with(1_u32));
    println!("{} - 1 = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), u32::MAX);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(55_u64).wrapping_sub(LongUnion::new_with(55_u64));
    println!("55 - 55 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 0_u64);

    let b_longunion = a_longunion.wrapping_sub(LongUnion::new_with(1_u64));
    println!("{} - 1 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(55_u128).wrapping_sub(LongerUnion::new_with(55_u128));
    println!("55 - 55 = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);

    let b_longerunion = a_longerunion.wrapping_sub(LongerUnion::new_with(1_u128));
    println!("{} - 1 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(55_usize).wrapping_sub(SizeUnion::new_with(55_usize));
    println!("55 - 55 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 0_usize);

    let b_sizeunion = a_sizeunion.wrapping_sub(SizeUnion::new_with(1_usize));
    println!("{} - 1 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    println!("--------------------------------------");
}

fn unions_overflowing_sub()
{
    println!("small_uint_overflowing_sub");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

    // Example for ShortUnion
    let (a_shortunion, overflow) = ShortUnion::new_with(55_u16).overflowing_sub(ShortUnion::new_with(55_u16));
    println!("55 - 55 = {}\nUnderflow = {}", a_shortunion, overflow);
    assert_eq!(a_shortunion.get(), 0_u16);
    assert_eq!(overflow, false);
 
    let (b_shortunion, overflow) = a_shortunion.overflowing_sub(ShortUnion::new_with(1_u16));
    println!("{} - 1 = {}\nUnderflow = {}", b_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), u16::MAX);
    assert_eq!(overflow, true);

    // Example for IntUnion
    let (a_intunion, overflow) = IntUnion::new_with(55_u32).overflowing_sub(IntUnion::new_with(55_u32));
    println!("55 - 55 = {}\nUnderflow = {}", a_intunion, overflow);
    assert_eq!(a_intunion.get(), 0_u32);
    assert_eq!(overflow, false);
 
    let (b_intunion, overflow) = a_intunion.overflowing_sub(IntUnion::new_with(1_u32));
    println!("{} - 1 = {}\nUnderflow = {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.get(), u32::MAX);
    assert_eq!(overflow, true);

    // Example for LongUnion
    let (a_longunion, overflow) = LongUnion::new_with(55_u64).overflowing_sub(LongUnion::new_with(55_u64));
    println!("55 - 55 = {}\nUnderflow = {}", a_longunion, overflow);
    assert_eq!(a_longunion.get(), 0_u64);
    assert_eq!(overflow, false);
 
    let (b_longunion, overflow) = a_longunion.overflowing_sub(LongUnion::new_with(1_u64));
    println!("{} - 1 = {}\nUnderflow = {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.get(), u64::MAX);
    assert_eq!(overflow, true);

    // Example for LongerUnion
    let (a_longerunion, overflow) = LongerUnion::new_with(55_u128).overflowing_sub(LongerUnion::new_with(55_u128));
    println!("55 - 55 = {}\nUnderflow = {}", a_longerunion, a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);
    assert_eq!(overflow, false);
 
    let (b_longerunion, overflow) = a_longerunion.overflowing_sub(LongerUnion::new_with(1_u128));
    println!("{} - 1 = {}\nUnderflow = {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), u128::MAX);
    assert_eq!(overflow, true);

    // Example for SizeUnion
    let (a_sizeunion, overflow) = SizeUnion::new_with(55_usize).overflowing_sub(SizeUnion::new_with(55_usize));
    println!("55 - 55 = {}\nUnderflow = {}", a_sizeunion, overflow);
    assert_eq!(a_sizeunion.get(), 0_usize);
    assert_eq!(overflow, false);
 
    let (b_sizeunion, overflow) = a_sizeunion.overflowing_sub(SizeUnion::new_with(1_usize));
    println!("{} - 1 = {}\nUnderflow = {}", a_sizeunion, b_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}

fn unions_checked_sub()
{
    println!("unions_checked_sub");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(55_u16).checked_sub(ShortUnion::new_with(55_u16));
    match a_shortunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u16);
            },
        None => { println!("Underflow happened."); },
    }

    let b_shortunion = a_shortunion.unwrap().checked_sub(ShortUnion::new_with(1_u16));
    match b_shortunion
    {
        Some(b) => { println!("{} - 1 = {}", a_shortunion.unwrap(), b); },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(55_u32).checked_sub(IntUnion::new_with(55_u32));
    match a_intunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u32);
            },
        None => { println!("Underflow happened."); },
    }

    let b_intunion = a_intunion.unwrap().checked_sub(IntUnion::new_with(1_u32));
    match b_intunion
    {
        Some(b) => { println!("{} - 1 = {}", a_intunion.unwrap(), b); },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(55_u64).checked_sub(LongUnion::new_with(55_u64));
    match a_longunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u64);
            },
        None => { println!("Underflow happened."); },
    }

    let b_longunion = a_longunion.unwrap().checked_sub(LongUnion::new_with(1_u64));
    match b_longunion
    {
        Some(b) => { println!("{} - 1 = {}", a_longunion.unwrap(), b); },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(55_u128).checked_sub(LongerUnion::new_with(55_u128));
    match a_longerunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u128);
            },
        None => { println!("Underflow happened."); },
    }

    let b_longerunion = a_longerunion.unwrap().checked_sub(LongerUnion::new_with(1_u128));
    match b_longerunion
    {
        Some(b) => { println!("{} - 1 = {}", a_longerunion.unwrap(), b); },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(55_usize).checked_sub(SizeUnion::new_with(55_usize));
    match a_sizeunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_usize);
            },
        None => { println!("Underflow happened."); },
    }

    let b_sizeunion = a_sizeunion.unwrap().checked_sub(SizeUnion::new_with(1_usize));
    match b_sizeunion
    {
        Some(b) => { println!("{} - 1 = {}", a_sizeunion.unwrap(), b); },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_sizeunion, None);
            },
    }
    println!("--------------------------------------");
}

fn unions_unchecked_sub()
{
    println!("unions_unchecked_sub");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    
    // Example for ShortUnion
    let a_ushortunion = ShortUnion::new_with(55_u16).unchecked_sub(ShortUnion::new_with(55_u16));
    println!("55 - 55 = {}", a_ushortunion);
    assert_eq!(a_ushortunion.get(), 0_u16);

    // It will panic
    // let b_ushortunion = ShortUnion::new_with(a_ushortunion.unchecked_sub(ShortUnion::new_with(1_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(55_u32).unchecked_sub(IntUnion::new_with(55_u32));
    println!("55 - 55 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 0_u32);

    // It will panic
    // let b_intunion = IntUnion::new_with(a_intunion.unchecked_sub(IntUnion::new_with(1_u32));

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(55_u64).unchecked_sub(LongUnion::new_with(55_u64));
    println!("55 - 55 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 0_u64);

    // It will panic
    // let b_u64 = LongUnion::new_with(a_longunion.unchecked_sub(LongUnion::new_with(1_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(55_u128).unchecked_sub(LongerUnion::new_with(55_u128));
    println!("55 - 55 = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);

    // It will panic
    // let b_longerunion = LongerUnion::new_with(a_longerunion.unchecked_sub(LongerUnion::new_with(1_u128));

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(55_usize).unchecked_sub(SizeUnion::new_with(55_usize));
    println!("55 - 55 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 0_usize);

    // It will panic
    // let b_sizeunion = SizeUnion::new_with(a_sizeunion.unchecked_sub(SizeUnion::new_with(1_usize));
    println!("--------------------------------------");
}

fn unions_saturating_sub()
{
    println!("unions_saturating_sub");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(55_u16).saturating_sub(ShortUnion::new_with(50_u16));
    println!("55 - 50 = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 5_u16);

    let b_u16 = a_shortunion.saturating_sub(ShortUnion::new_with(55_u16));
    println!("5 - 55 = {}", b_u16);
    assert_eq!(b_u16.get(), 0_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(55_u32).saturating_sub(IntUnion::new_with(50_u32));
    println!("55 - 50 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 5_u32);

    let b_intunion = a_intunion.saturating_sub(IntUnion::new_with(55_u32));
    println!("{} - 55 = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 0_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(55_u64).saturating_sub(LongUnion::new_with(50_u64));
    println!("55 - 50 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 5_u64);

    let b_longunion = a_longunion.saturating_sub(LongUnion::new_with(55_u64));
    println!("{} - 55 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(55_u128).saturating_sub(LongerUnion::new_with(50_u128));
    println!("55 - 50 = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 5_u128);

    let b_longerunion = a_longerunion.saturating_sub(LongerUnion::new_with(55_u128));
    println!("{} - 55 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(55_usize).saturating_sub(SizeUnion::new_with(50_usize));
    println!("55 - 50 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 5_usize);

    let b_sizeunion = a_sizeunion.saturating_sub(SizeUnion::new_with(55_usize));
    println!("{} - 55 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}


fn unions_abs_diff()
{
    println!("union_abs_diff");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(5050_u16);
    let b_shortunion = ShortUnion::new_with(5000_u16);
    let c_shortunion = a_shortunion.abs_diff(b_shortunion);
    let d_shortunion = b_shortunion.abs_diff(a_shortunion);
    println!("{} <-> {} = {}", a_shortunion, b_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), 50_u16);
    println!("{} <-> {} = {}", b_shortunion, a_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), 50_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(500500_u32);
    let b_intunion = IntUnion::new_with(500000_u32);
    let c_intunion = a_intunion.abs_diff(b_intunion);
    let d_intunion = b_intunion.abs_diff(a_intunion);
    println!("{} <-> {} = {}", a_intunion, b_intunion, c_intunion);
    assert_eq!(c_intunion.get(), 500_u32);
    println!("{} <-> {} = {}", b_intunion, a_intunion, d_intunion);
    assert_eq!(d_intunion.get(), 500_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(5000050000_u64);
    let b_longunion = LongUnion::new_with(5000000000_u64);
    let c_longunion = a_longunion.abs_diff(b_longunion);
    let d_longunion = b_longunion.abs_diff(a_longunion);
    println!("{} <-> {} = {}", a_longunion, b_longunion, c_longunion);
    assert_eq!(c_longunion.get(), 50000_u64);
    println!("{} <-> {} = {}", b_longunion, a_longunion, d_longunion);
    assert_eq!(d_longunion.get(), 50000_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(500000000500000000_u128);
    let b_longerunion = LongerUnion::new_with(500000000000000000_u128);
    let c_longerunion = a_longerunion.abs_diff(b_longerunion);
    let d_longerunion = b_longerunion.abs_diff(a_longerunion);
    println!("{} <-> {} = {}", a_longerunion, b_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), 500000000_u128);
    println!("{} <-> {} = {}", b_longerunion, a_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), 500000000_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(105_usize);
    let b_sizeunion = SizeUnion::new_with(100_usize);
    let c_sizeunion = a_sizeunion.abs_diff(b_sizeunion);
    let d_sizeunion = b_sizeunion.abs_diff(a_sizeunion);
    println!("{} <-> {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), 5_usize);
    println!("{} <-> {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 5_usize);
    println!("--------------------------------------");
}


fn unions_mul_main()
{
    unions_carrying_mul();
    unions_widening_mul();
    unions_wrapping_mul();
    unions_overflowing_mul();
    unions_checked_mul();
    unions_unchecked_mul();
    unions_saturating_mul();
}

fn unions_carrying_mul()
{
    println!("unions_carrying_mul");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

    // Example for ShortUnion for Little Endian
    // a_u64: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    let a_high_shortunion = ShortUnion::new_with(10000_u16);
    let a_low_shortunion = ShortUnion::new_with(10100_u16);
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    let b_high_shortunion = ShortUnion::new_with(10000_u16);
    let b_low_shortunion = ShortUnion::new_with(20000_u16);
    
    // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    //
    //                        (10000_u16, 10100_u16) == 655370100_u32
    // X                      (10000_u16, 20000_u16) == 655380000_u32
    // ---------------------------------------------
    //                       (  3082_u16, 18048_u16)
    //            (  3051_u16, 49664_u16)
    //            (  1541_u16,  9024_u16)
    // + (1525_u16, 57600_u16)
    // ---------------------------------
    //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    let zero_shortunion = ShortUnion::zero();
    let one_shortunion = ShortUnion::one();
    let (c_lower_shortunion, c_tmp_shortunion) = b_low_shortunion.carrying_mul(a_low_shortunion, zero_shortunion);
    let (d_low_shortunion, d_high_shortunion) = b_low_shortunion.carrying_mul(a_high_shortunion, c_tmp_shortunion);
    let (mut c_low_shortunion, e_high_shortunion) = b_high_shortunion.carrying_mul(a_low_shortunion, zero_shortunion);
    let (mut c_high_shortunion, mut c_higher_shortunion) = b_high_shortunion.carrying_mul(a_high_shortunion, e_high_shortunion);

    let mut overflow: bool;
    (c_low_shortunion, overflow) = c_low_shortunion.overflowing_add(d_low_shortunion);
    if overflow
        { (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(one_shortunion); }
    if overflow
        { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }

    (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(d_high_shortunion);
    if overflow
        { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
    println!("{}-{}-{}-{}", c_higher_shortunion, c_high_shortunion, c_low_shortunion, c_lower_shortunion);
    assert_eq!(c_higher_shortunion.get(), 1525_u16);
    assert_eq!(c_high_shortunion.get(), 62192_u16);
    assert_eq!(c_low_shortunion.get(), 61770_u16);
    assert_eq!(c_lower_shortunion.get(), 18048_u16);

    let a = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_shortunion.get(), c.get_ushort_(3));
    assert_eq!(c_high_shortunion.get(), c.get_ushort_(2));
    assert_eq!(c_low_shortunion.get(), c.get_ushort_(1));
    assert_eq!(c_lower_shortunion.get(), c.get_ushort_(0));

    // Example for IntUnion for Little Endian
    // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_intunion = IntUnion::new_with(2299561912_u32);
    let a_low_intunion = IntUnion::new_with(2956226837_u32);
    // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_intunion = IntUnion::new_with(1782160508_u32);
    let b_low_intunion = IntUnion::new_with(682685733_u32);

    // (2299561912_u32, 2956226837_u32) X (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 X 7654321098765432101_u64 == (4098188426859548455_u64, 17997868695111430409_u64) == 75598233076116445704676116321386983689_u128
    //
    //                                  (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    // X                                (1782160508_u32,  682685733_u32) == 7654321098765432101_u64
    // -----------------------------------------------------------------
    //                                  ( 469892724_u32, 2923262217_u32)
    //                  ( 365515730_u32, 2949035416_u32)
    //                  (1226661429_u32,  771527212_u32)
    // + (954183848_u32, 3735936288_u32)
    // -----------------------------------------------------------------
    //   (954183849_u32, 1033146151_u32, 4190455352_u32, 2923262217_u32) == 429516456138000000_u64
    let zero_intunion = IntUnion::zero();
    let one_intunion = IntUnion::one();
    let (c_lower_intunion, c_tmp_intunion) = b_low_intunion.carrying_mul(a_low_intunion, zero_intunion);
    let (d_low_intunion, d_high_intunion) = b_low_intunion.carrying_mul(a_high_intunion, c_tmp_intunion);
    let (mut c_low_intunion, e_high_intunion) = b_high_intunion.carrying_mul(a_low_intunion, zero_intunion);
    let (mut c_high_intunion, mut c_higher_intunion) = b_high_intunion.carrying_mul(a_high_intunion, e_high_intunion);

    let mut overflow: bool;
    (c_low_intunion, overflow) = c_low_intunion.overflowing_add(d_low_intunion);
    if overflow
        { (c_high_intunion, overflow) = c_high_intunion.overflowing_add(one_intunion); }
    if overflow
        { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }

    (c_high_intunion, overflow) = c_high_intunion.overflowing_add(d_high_intunion);
    if overflow
        { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
    println!("{}-{}-{}-{}", c_higher_intunion, c_high_intunion, c_low_intunion, c_lower_intunion);
    assert_eq!(c_higher_intunion.get(), 954183849_u32);
    assert_eq!(c_high_intunion.get(), 1033146151_u32);
    assert_eq!(c_low_intunion.get(), 4190455352_u32);
    assert_eq!(c_lower_intunion.get(), 2923262217_u32);

    let a = LongerUnion::new_with_uints([a_low_intunion.get(), a_high_intunion.get(), 0, 0]);
    let b = LongerUnion::new_with_uints([b_low_intunion.get(), b_high_intunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_intunion.get(), c.get_uint_(3));
    assert_eq!(c_high_intunion.get(), c.get_uint_(2));
    assert_eq!(c_low_intunion.get(), c.get_uint_(1));
    assert_eq!(c_lower_intunion.get(), c.get_uint_(0));

    // Example for LongUnion for Little Endian
    // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_longunion = LongUnion::new_with(10775095670246085798_u64);
    let a_low_longunion = LongUnion::new_with(7681743649119882630_u64);
    // b_u64: u64 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_longunion = LongUnion::new_with(6692605942763486917_u64);
    let b_low_longunion = LongUnion::new_with(12312739301371248917_u64);

    // (10775095670246085798_u64, 7681743649119882630_u64) X (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    //
    //                                                      (10775095670246085798_u64,  7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    // X                                                    ( 6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    // ---------------------------------------------------------------------------------------------------------
    //                                                      ( 5127371342803972846_u64,  9393535397455192574_u64)
    //                             (7192106282005498115_u64,  3473120370613376926_u64)
    //                             (2786989562573083321_u64,  6840685591062354974_u64)
    // + (3909279004922650219_u64,  1464703988338300862_u64)
    // ---------------------------------------------------------------------------------------------------------
    //   (3909279004922650219_u64, 11443799832916882298_u64, 15441177304479704746_u64,  9393535397455192574_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    let zero_longunion = LongUnion::zero();
    let one_longunion = LongUnion::one();
    let (c_lower_longunion, c_tmp_longunion) = b_low_longunion.carrying_mul(a_low_longunion, zero_longunion);
    let (d_low_longunion, d_high_longunion) = b_low_longunion.carrying_mul(a_high_longunion, c_tmp_longunion);
    let (mut c_low_longunion, e_high_longunion) = b_high_longunion.carrying_mul(a_low_longunion, zero_longunion);
    let (mut c_high_longunion, mut c_higher_longunion) = b_high_longunion.carrying_mul(a_high_longunion, e_high_longunion);

    let mut overflow: bool;
    (c_low_longunion, overflow) = c_low_longunion.overflowing_add(d_low_longunion);
    if overflow
        { (c_high_longunion, overflow) = c_high_longunion.overflowing_add(one_longunion); }
    if overflow
        { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }

    (c_high_longunion, overflow) = c_high_longunion.overflowing_add(d_high_longunion);
    if overflow
        { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
    println!("{}-{}-{}-{}", c_higher_longunion, c_high_longunion, c_low_longunion, c_lower_longunion);
    assert_eq!(c_higher_longunion.get(), 3909279004922650219_u64);
    assert_eq!(c_high_longunion.get(), 11443799832916882298_u64);
    assert_eq!(c_low_longunion.get(), 15441177304479704746_u64);
    assert_eq!(c_lower_longunion.get(), 9393535397455192574_u64);

    // Example for LongerUnion for Little Endian
    // a_u256: u256 === (a_high_u128, a_low_u128) == (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    let a_high_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
    let a_low_longerunion = LongerUnion::new_with(198765432198765432198765432198765432198_u128);
    // b_u256: u256 === (b_high_u128, b_low_u128) == (75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    let b_high_longerunion = LongerUnion::new_with(75318642097531864209753186420975318642_u128);
    let b_low_longerunion = LongerUnion::new_with(135792468013579246801357924680135792468_u128);

    // (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) X (75318642097531864209753186420975318642_u128 - 135792468013579246801357924680135792468_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256 X 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256 = 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    //
    //                                                                                              (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    // X                                                                                            ( 75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //                                                                                              ( 79318975115531594676802389315672824709_u128, 305933135181961371815664194362919418360_u128)
    //                                                ( 49266443702953415606417933871327680361_u128, 301235724958848324675382352967843249636_u128)
    //                                                ( 43995057941448862830514490586650222101_u128,  35386202970580104685103432753963846060_u128)
    // + (27326122685316262062508597076325453266_u128, 184240100967607654057575481238459345242_u128)
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   (27326122685316262062508597076325453266_u128, 277501602612009932494507905696437247705_u128,  75658536124021560573913567605711708949_u128, 305933135181961371815664194362919418360_u128) == 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    let zero_longerunion = LongerUnion::zero();
    let one_longerunion = LongerUnion::zero();
    let (c_lower_longerunion, c_tmp_longerunion) = b_low_longerunion.carrying_mul(a_low_longerunion, zero_longerunion);
    let (d_low_longerunion, d_high_longerunion) = b_low_longerunion.carrying_mul(a_high_longerunion, c_tmp_longerunion);
    let (mut c_low_longerunion, e_high_longerunion) = b_high_longerunion.carrying_mul(a_low_longerunion, zero_longerunion);
    let (mut c_high_longerunion, mut c_higher_longerunion) = b_high_longerunion.carrying_mul(a_high_longerunion, e_high_longerunion);

    let mut overflow: bool;
    (c_low_longerunion, overflow) = c_low_longerunion.overflowing_add(d_low_longerunion);
    if overflow
        { (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(one_longerunion); }
    if overflow
        { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }

    (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(d_high_longerunion);
    if overflow
        { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
    println!("{}-{}-{}-{}", c_higher_longerunion, c_high_longerunion, c_low_longerunion, c_lower_longerunion);
    assert_eq!(c_higher_longerunion.get(), 27326122685316262062508597076325453266_u128);
    assert_eq!(c_high_longerunion.get(), 277501602612009932494507905696437247705_u128);
    assert_eq!(c_low_longerunion.get(), 75658536124021560573913567605711708949_u128);
    assert_eq!(c_lower_longerunion.get(), 305933135181961371815664194362919418360_u128);

    // Example for SizeUnion for 64-bit CPUs for Little Endian
    #[cfg(target_pointer_width = "64")]
    {
        // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        let a_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
        let a_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
        // b_usize: usize === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        let b_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
        let b_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);

        // (10775095670246085798_usize, 7681743649119882630_usize) X (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        //
        //                                                          (10775095670246085798_usize,  7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        // X                                                        ( 6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        // -----------------------------------------------------------------------------------------------------------------
        //                                                          ( 5127371342803972846_usize,  9393535397455192574_usize)
        //                               (7192106282005498115_usize,  3473120370613376926_usize)
        //                               (2786989562573083321_usize,  6840685591062354974_usize)
        // + (3909279004922650219_usize,  1464703988338300862_usize)
        // -----------------------------------------------------------------------------------------------------------------
        //   (3909279004922650219_usize, 11443799832916882298_usize, 15441177304479704746_usize,  9393535397455192574_usize) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        let zero_sizeunion = SizeUnion::new_with(0);
        let one_sizeunion = SizeUnion::new_with(1);
        let (c_lower_sizeunion, c_tmp_sizeunion) = b_low_sizeunion.carrying_mul(a_low_sizeunion, zero_sizeunion);
        let (d_low_sizeunion, d_high_sizeunion) = b_low_sizeunion.carrying_mul(a_high_sizeunion, c_tmp_sizeunion);
        let (mut c_low_sizeunion, e_high_sizeunion) = b_high_sizeunion.carrying_mul(a_low_sizeunion, zero_sizeunion);
        let (mut c_high_sizeunion, mut c_higher_sizeunion) = b_high_sizeunion.carrying_mul(a_high_sizeunion, e_high_sizeunion);
    
        let mut overflow: bool;
        (c_low_sizeunion, overflow) = c_low_sizeunion.overflowing_add(d_low_sizeunion);
        if overflow
            { (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(one_sizeunion); }
        if overflow
            { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
    
        (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(d_high_sizeunion);
        if overflow
            { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
            println!("{}-{}-{}-{}", c_higher_sizeunion, c_high_sizeunion, c_low_sizeunion, c_lower_sizeunion);
        assert_eq!(c_higher_sizeunion.get(), 3909279004922650219_usize);
        assert_eq!(c_high_sizeunion.get(), 11443799832916882298_usize);
        assert_eq!(c_low_sizeunion.get(), 15441177304479704746_usize);
        assert_eq!(c_lower_sizeunion.get(), 9393535397455192574_usize);
    }
    println!("--------------------------------------");
}

fn unions_widening_mul()
{
    println!("unions_widening_mul");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

    // Example for ShortUnion for Little Endian
    // a_u64: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    let a_high_shortunion = ShortUnion::new_with(10000_u16);
    let a_low_shortunion = ShortUnion::new_with(10100_u16);
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    let b_high_shortunion = ShortUnion::new_with(10000_u16);
    let b_low_shortunion = ShortUnion::new_with(20000_u16);
    
    // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    //
    //                        (10000_u16, 10100_u16) == 655370100_u32
    // X                      (10000_u16, 20000_u16) == 655380000_u32
    // ---------------------------------------------
    //                       (  3082_u16, 18048_u16)
    //            (  3051_u16, 49664_u16)
    //            (  1541_u16,  9024_u16)
    // + (1525_u16, 57600_u16)
    // ---------------------------------
    //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    let one_shortunion = ShortUnion::one();
    let (c_lower_shortunion, c_temp_shortunion) = b_low_shortunion.widening_mul(a_low_shortunion);
    let (d_low_shortunion, d_high_shortunion) = b_low_shortunion.widening_mul(a_high_shortunion);
    let (mut c_low_shortunion, e_high_shortunion) = b_high_shortunion.widening_mul(a_low_shortunion);
    let (mut c_high_shortunion, mut c_higher_shortunion) = b_high_shortunion.widening_mul(a_high_shortunion);

    let mut overflow: bool;
    (c_low_shortunion, overflow) = c_low_shortunion.overflowing_add(d_low_shortunion);
    if overflow
        { (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(one_shortunion); }
    if overflow
        { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
    (c_low_shortunion, overflow) = c_low_shortunion.overflowing_add(c_temp_shortunion);
    if overflow
        { (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(one_shortunion); }
    if overflow
        { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }

    (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(d_high_shortunion);
    if overflow
        { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
    (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(e_high_shortunion);
    if overflow
        { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
    println!("{}-{}-{}-{}", c_higher_shortunion, c_high_shortunion, c_low_shortunion, c_lower_shortunion);
    assert_eq!(c_higher_shortunion.get(), 1525_u16);
    assert_eq!(c_high_shortunion.get(), 62192_u16);
    assert_eq!(c_low_shortunion.get(), 61770_u16);
    assert_eq!(c_lower_shortunion.get(), 18048_u16);

    let a = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_shortunion.get(), c.get_ushort_(3));
    assert_eq!(c_high_shortunion.get(), c.get_ushort_(2));
    assert_eq!(c_low_shortunion.get(), c.get_ushort_(1));
    assert_eq!(c_lower_shortunion.get(), c.get_ushort_(0));

    // Example for IntUnion for Little Endian
    // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_intunion = IntUnion::new_with(2299561912_u32);
    let a_low_intunion = IntUnion::new_with(2956226837_u32);
    // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_intunion = IntUnion::new_with(1782160508_u32);
    let b_low_intunion = IntUnion::new_with(682685733_u32);

    // (2299561912_u32, 2956226837_u32) X (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 X 7654321098765432101_u64 == (4098188426859548455_u64, 17997868695111430409_u64) == 75598233076116445704676116321386983689_u128
    //
    //                                  (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    // X                                (1782160508_u32,  682685733_u32) == 7654321098765432101_u64
    // -----------------------------------------------------------------
    //                                  ( 469892724_u32, 2923262217_u32)
    //                  ( 365515730_u32, 2949035416_u32)
    //                  (1226661429_u32,  771527212_u32)
    // + (954183848_u32, 3735936288_u32)
    // -----------------------------------------------------------------
    //   (954183849_u32, 1033146151_u32, 4190455352_u32, 2923262217_u32) == 429516456138000000_u64
    let one_intunion = IntUnion::one();
    let (c_lower_intunion, c_temp_intunion) = b_low_intunion.widening_mul(a_low_intunion);
    let (d_low_intunion, d_high_intunion) = b_low_intunion.widening_mul(a_high_intunion);
    let (mut c_low_intunion, e_high_intunion) = b_high_intunion.widening_mul(a_low_intunion);
    let (mut c_high_intunion, mut c_higher_intunion) = b_high_intunion.widening_mul(a_high_intunion);

    let mut overflow: bool;
    (c_low_intunion, overflow) = c_low_intunion.overflowing_add(d_low_intunion);
    if overflow
        { (c_high_intunion, overflow) = c_high_intunion.overflowing_add(one_intunion); }
    if overflow
        { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
    (c_low_intunion, overflow) = c_low_intunion.overflowing_add(c_temp_intunion);
    if overflow
        { (c_high_intunion, overflow) = c_high_intunion.overflowing_add(one_intunion); }
    if overflow
        { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }

    (c_high_intunion, overflow) = c_high_intunion.overflowing_add(d_high_intunion);
    if overflow
        { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
    (c_high_intunion, overflow) = c_high_intunion.overflowing_add(e_high_intunion);
    if overflow
        { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
    println!("{}-{}-{}-{}", c_higher_intunion, c_high_intunion, c_low_intunion, c_lower_intunion);
    assert_eq!(c_higher_intunion.get(), 954183849_u32);
    assert_eq!(c_high_intunion.get(), 1033146151_u32);
    assert_eq!(c_low_intunion.get(), 4190455352_u32);
    assert_eq!(c_lower_intunion.get(), 2923262217_u32);

    let a = LongerUnion::new_with_uints([a_low_intunion.get(), a_high_intunion.get(), 0, 0]);
    let b = LongerUnion::new_with_uints([b_low_intunion.get(), b_high_intunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_intunion.get(), c.get_uint_(3));
    assert_eq!(c_high_intunion.get(), c.get_uint_(2));
    assert_eq!(c_low_intunion.get(), c.get_uint_(1));
    assert_eq!(c_lower_intunion.get(), c.get_uint_(0));

    // Example for LongUnion for Little Endian
    // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_longunion = LongUnion::new_with(10775095670246085798_u64);
    let a_low_longunion = LongUnion::new_with(7681743649119882630_u64);
    // b_u64: u64 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_longunion = LongUnion::new_with(6692605942763486917_u64);
    let b_low_longunion = LongUnion::new_with(12312739301371248917_u64);

    // (10775095670246085798_u64, 7681743649119882630_u64) X (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    //
    //                                                      (10775095670246085798_u64,  7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    // X                                                    ( 6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    // ---------------------------------------------------------------------------------------------------------
    //                                                      ( 5127371342803972846_u64,  9393535397455192574_u64)
    //                             (7192106282005498115_u64,  3473120370613376926_u64)
    //                             (2786989562573083321_u64,  6840685591062354974_u64)
    // + (3909279004922650219_u64,  1464703988338300862_u64)
    // ---------------------------------------------------------------------------------------------------------
    //   (3909279004922650219_u64, 11443799832916882298_u64, 15441177304479704746_u64,  9393535397455192574_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    let one_longunion = LongUnion::one();
    let (c_lower_longunion, c_temp_longunion) = b_low_longunion.widening_mul(a_low_longunion);
    let (d_low_longunion, d_high_longunion) = b_low_longunion.widening_mul(a_high_longunion);
    let (mut c_low_longunion, e_high_longunion) = b_high_longunion.widening_mul(a_low_longunion);
    let (mut c_high_longunion, mut c_higher_longunion) = b_high_longunion.widening_mul(a_high_longunion);

    let mut overflow: bool;
    (c_low_longunion, overflow) = c_low_longunion.overflowing_add(d_low_longunion);
    if overflow
        { (c_high_longunion, overflow) = c_high_longunion.overflowing_add(one_longunion); }
    if overflow
        { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
    (c_low_longunion, overflow) = c_low_longunion.overflowing_add(c_temp_longunion);
    if overflow
        { (c_high_longunion, overflow) = c_high_longunion.overflowing_add(one_longunion); }
    if overflow
        { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }

    (c_high_longunion, overflow) = c_high_longunion.overflowing_add(d_high_longunion);
    if overflow
        { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
    (c_high_longunion, overflow) = c_high_longunion.overflowing_add(e_high_longunion);
    if overflow
        { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
    println!("{}-{}-{}-{}", c_higher_longunion, c_high_longunion, c_low_longunion, c_lower_longunion);
    assert_eq!(c_higher_longunion.get(), 3909279004922650219_u64);
    assert_eq!(c_high_longunion.get(), 11443799832916882298_u64);
    assert_eq!(c_low_longunion.get(), 15441177304479704746_u64);
    assert_eq!(c_lower_longunion.get(), 9393535397455192574_u64);

    // Example for LongerUnion for Little Endian
    // a_u256: u256 === (a_high_u128, a_low_u128) == (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    let a_high_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
    let a_low_longerunion = LongerUnion::new_with(198765432198765432198765432198765432198_u128);
    // b_u256: u256 === (b_high_u128, b_low_u128) == (75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    let b_high_longerunion = LongerUnion::new_with(75318642097531864209753186420975318642_u128);
    let b_low_longerunion = LongerUnion::new_with(135792468013579246801357924680135792468_u128);

    // (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) X (75318642097531864209753186420975318642_u128 - 135792468013579246801357924680135792468_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256 X 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256 = 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    //
    //                                                                                              (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    // X                                                                                            ( 75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //                                                                                              ( 79318975115531594676802389315672824709_u128, 305933135181961371815664194362919418360_u128)
    //                                                ( 49266443702953415606417933871327680361_u128, 301235724958848324675382352967843249636_u128)
    //                                                ( 43995057941448862830514490586650222101_u128,  35386202970580104685103432753963846060_u128)
    // + (27326122685316262062508597076325453266_u128, 184240100967607654057575481238459345242_u128)
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   (27326122685316262062508597076325453266_u128, 277501602612009932494507905696437247705_u128,  75658536124021560573913567605711708949_u128, 305933135181961371815664194362919418360_u128) == 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    let one_longerunion = LongerUnion::one();
    let (c_lower_longerunion, c_temp_longerunion) = b_low_longerunion.widening_mul(a_low_longerunion);
    let (d_low_longerunion, d_high_longerunion) = b_low_longerunion.widening_mul(a_high_longerunion);
    let (mut c_low_longerunion, e_high_longerunion) = b_high_longerunion.widening_mul(a_low_longerunion);
    let (mut c_high_longerunion, mut c_higher_longerunion) = b_high_longerunion.widening_mul(a_high_longerunion);

    let mut overflow: bool;
    (c_low_longerunion, overflow) = c_low_longerunion.overflowing_add(d_low_longerunion);
    if overflow
        { (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(one_longerunion); }
    if overflow
        { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
    (c_low_longerunion, overflow) = c_low_longerunion.overflowing_add(c_temp_longerunion);
    if overflow
        { (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(one_longerunion); }
    if overflow
        { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }

    (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(d_high_longerunion);
    if overflow
        { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
    (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(e_high_longerunion);
    if overflow
        { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
    println!("{}-{}-{}-{}", c_higher_longerunion, c_high_longerunion, c_low_longerunion, c_lower_longerunion);
    assert_eq!(c_higher_longerunion.get(), 27326122685316262062508597076325453266_u128);
    assert_eq!(c_high_longerunion.get(), 277501602612009932494507905696437247705_u128);
    assert_eq!(c_low_longerunion.get(), 75658536124021560573913567605711708949_u128);
    assert_eq!(c_lower_longerunion.get(), 305933135181961371815664194362919418360_u128);

    // Example for SizeUnion for 64-bit CPUs for Little Endian
    #[cfg(target_pointer_width = "64")]
    {
        // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        let a_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
        let a_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
        // b_usize: usize === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        let b_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
        let b_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);

        // (10775095670246085798_usize, 7681743649119882630_usize) X (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        //
        //                                                          (10775095670246085798_usize,  7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        // X                                                        ( 6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        // -----------------------------------------------------------------------------------------------------------------
        //                                                          ( 5127371342803972846_usize,  9393535397455192574_usize)
        //                               (7192106282005498115_usize,  3473120370613376926_usize)
        //                               (2786989562573083321_usize,  6840685591062354974_usize)
        // + (3909279004922650219_usize,  1464703988338300862_usize)
        // -----------------------------------------------------------------------------------------------------------------
        //   (3909279004922650219_usize, 11443799832916882298_usize, 15441177304479704746_usize,  9393535397455192574_usize) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        let one_sizeunion = SizeUnion::one();
        let (c_lower_sizeunion, c_temp_sizeunion) = b_low_sizeunion.widening_mul(a_low_sizeunion);
        let (d_low_sizeunion, d_high_sizeunion) = b_low_sizeunion.widening_mul(a_high_sizeunion);
        let (mut c_low_sizeunion, e_high_sizeunion) = b_high_sizeunion.widening_mul(a_low_sizeunion);
        let (mut c_high_sizeunion, mut c_higher_sizeunion) = b_high_sizeunion.widening_mul(a_high_sizeunion);
    
        let mut overflow: bool;
        (c_low_sizeunion, overflow) = c_low_sizeunion.overflowing_add(d_low_sizeunion);
        if overflow
            { (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(one_sizeunion); }
        if overflow
            { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        (c_low_sizeunion, overflow) = c_low_sizeunion.overflowing_add(c_temp_sizeunion);
        if overflow
            { (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(one_sizeunion); }
        if overflow
            { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
    
        (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(d_high_sizeunion);
        if overflow
            { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(e_high_sizeunion);
        if overflow
            { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        println!("{}-{}-{}-{}", c_higher_sizeunion, c_high_sizeunion, c_low_sizeunion, c_lower_sizeunion);
        assert_eq!(c_higher_sizeunion.get(), 3909279004922650219_usize);
        assert_eq!(c_high_sizeunion.get(), 11443799832916882298_usize);
        assert_eq!(c_low_sizeunion.get(), 15441177304479704746_usize);
        assert_eq!(c_lower_sizeunion.get(), 9393535397455192574_usize);
    }
    println!("--------------------------------------");
}

fn unions_wrapping_mul()
{
    println!("unions_wrapping_mul");
    println!("--------------------------------------");
}

fn unions_overflowing_mul()
{
    println!("unions_overflowing_mul");
    println!("--------------------------------------");
}

fn unions_checked_mul()
{
    println!("unions_checked_mul");
    println!("--------------------------------------");
}

fn unions_unchecked_mul()
{
    println!("unions_unchecked_mul");
    println!("--------------------------------------");
}

fn unions_saturating_mul()
{
    println!("unions_saturating_mul");
    println!("--------------------------------------");
}


fn unions_div_main(){}


fn unions_rem_main(){}


fn unions_neg_main()
{
    unions_wrapping_neg();
    unions_overflowing_neg();
}

fn unions_wrapping_neg()
{
    println!("unions_wrapping_neg");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(12345_u16);
    let b_shortunion = a_shortunion.wrapping_neg();
    println!("-{} = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 53191_u16);
    
    // Example for IntUnion
    let a_intunion = IntUnion::new_with(1234567890_u32);
    let b_intunion = a_intunion.wrapping_neg();
    println!("-{} = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 3060399406_u32);
    
    // Example for LongUnion
    let a_longunion = LongUnion::new_with(12345678901234567890_u64);
    let b_longunion = a_longunion.wrapping_neg();
    println!("-{} = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 6101065172474983726_u64);
    
    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
    let b_longerunion = a_longerunion.wrapping_neg();
    println!("-{} = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 216825577908592784562140039541644754667_u128);
    
    // Example for SizeUnion for 64-bit CPU
    let a_sizeunion = SizeUnion::new_with(1234567890123456789_usize);
    let b_sizeunion = a_sizeunion.wrapping_neg();
    println!("-{} = {}", a_sizeunion, a_sizeunion);
    assert_eq!(b_sizeunion.get(), 17212176183586094827_usize);
    println!("--------------------------------------");
}

fn unions_overflowing_neg()
{
    println!("unions_overflowing_neg");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(0_u16);
    let (b_shortunion, overflow) = a_shortunion.overflowing_neg();
    println!("-{} = {}, {}", a_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), 0_u16);
    assert_eq!(overflow, false);

    let c_shortunion = ShortUnion::new_with(12345_u16);
    let (d_shortunion, overflow) = c_shortunion.overflowing_neg();
    println!("-{} = {}, {}", c_shortunion, d_shortunion, overflow);
    assert_eq!(d_shortunion.get(), 53191_u16);
    assert_eq!(overflow, true);
    
    // Example for IntUnion
    let a_intunion = IntUnion::new_with(0_u32);
    let (b_intunion, overflow) = a_intunion.overflowing_neg();
    println!("-{} = {}, {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.get(), 0_u32);
    assert_eq!(overflow, false);

    let c_intunion = IntUnion::new_with(1234567890_u32);
    let (d_intunion, overflow) = c_intunion.overflowing_neg();
    println!("-{} = {}, {}", c_intunion, d_intunion, overflow);
    assert_eq!(d_intunion.get(), 3060399406_u32);
    assert_eq!(overflow, true);
    
    // Example for LongUnion
    let a_longunion = LongUnion::new_with(0_u64);
    let (b_longunion , overflow) = a_longunion.overflowing_neg();
    println!("-{} = {}, {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.get(), 0_u64);
    assert_eq!(overflow, false);

    let c_longunion = LongUnion::new_with(12345678901234567890_u64);
    let (d_longunion, overflow) = c_longunion.overflowing_neg();
    println!("-{} = {}, {}", c_longunion, d_longunion, overflow);
    assert_eq!(d_longunion.get(), 6101065172474983726_u64);
    assert_eq!(overflow, true);
    
    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(0_u128);
    let (b_longerunion, overflow) = a_longerunion.overflowing_neg();
    println!("-{} = {}, {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), 0_u128);
    assert_eq!(overflow, false);

    let c_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
    let (d_longerunion, overflow) = c_longerunion.overflowing_neg();
    println!("-{} = {}, {}", c_longerunion, d_longerunion, overflow);
    assert_eq!(d_longerunion.get(), 216825577908592784562140039541644754667_u128);
    assert_eq!(overflow, true);
    
    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(0_usize);
    let (b_sizeunion, overflow) = a_sizeunion.overflowing_neg();
    println!("-{} = {}, {}", a_sizeunion, a_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), 0_usize);
    assert_eq!(overflow, false);

    let c_sizeunion = SizeUnion::new_with(1234567890123456789_usize);
    let (d_sizeunion, overflow) = c_sizeunion.overflowing_neg();
    println!("-{} = {}, {}", c_sizeunion, d_sizeunion, overflow);
    assert_eq!(d_sizeunion.get(), 17212176183586094827_usize);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}


fn unions_pow_main(){}


fn unions_log_main(){}


fn unions_root_main(){}


fn unions_prime_main(){}


fn unions_bits_operation(){}


fn unions_bytes_operation(){}


fn unions_find_power(){}


fn unions_conversion(){}


fn unions_constants(){}


fn unions_size(){}


/*
fn short_union_debug_fmt_main()
{
    println!("short_union_main");
    use cryptocol::number::ShortUnion;
    let a_short = ShortUnion::new_with_signed(-12345_i16);
    println!("a_short = {:?}", a_short);
    println!("a_short = {:#?}", a_short);
    assert_eq!(format!("{a_short:?}"), "ShortUnion { this: 53191, that: -12345, ushort: 53191, sshort: -12345, ubyte: [199, 207], sbyte: [-57, -49] }");
    assert_eq!(format!("{a_short:#?}"), r#"ShortUnion {
    this: 53191,
    that: -12345,
    ushort: 53191,
    sshort: -12345,
    ubyte: [
        199,
        207,
    ],
    sbyte: [
        -57,
        -49,
    ],
}"#);
    println!("--------------------------------------");
}

fn int_union_debug_fmt_main()
{
    println!("int_union_main");
    use cryptocol::number::IntUnion;
    let a_int = IntUnion::new_with_signed(-1234567890_i32);
    println!("a_int = {:?}", a_int);
    println!("a_int = {:#?}", a_int);
    #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:?}"), "IntUnion { this: 3060399406, that: -1234567890, uint: 3060399406, sint: -1234567890, ushort: [64814, 46697], sshort: [-722, -18839], ubyte: [46, 253, 105, 182], sbyte: [46, -3, 105, -74] }");
    #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:#?}"), r#"IntUnion {
    this: 3060399406,
    that: -1234567890,
    uint: 3060399406,
    sint: -1234567890,
    ushort: [
        64814,
        46697,
    ],
    sshort: [
        -722,
        -18839,
    ],
    ubyte: [
        46,
        253,
        105,
        182,
    ],
    sbyte: [
        46,
        -3,
        105,
        -74,
    ],
}"#);
    println!("--------------------------------------");
}

fn long_union_debug_fmt_main()
{
    println!("long_union_main");
    use cryptocol::number::LongUnion;
    let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    println!("a_long = {:?}", a_long);
    println!("a_long = {:#?}", a_long);
    #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:?}"), "LongUnion { this: 17212176183586094827, that: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18], u_size: 17212176183586094827, s_size: -1234567890123456789 }");
    #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:#?}"), r#"LongUnion {
    this: 17212176183586094827,
    that: -1234567890123456789,
    ulong: 17212176183586094827,
    slong: -1234567890123456789,
    uint: [
        2182512363,
        4007522059,
    ],
    sint: [
        -2112454933,
        -287445237,
    ],
    ushort: [
        32491,
        33302,
        61195,
        61149,
    ],
    sshort: [
        32491,
        -32234,
        -4341,
        -4387,
    ],
    ubyte: [
        235,
        126,
        22,
        130,
        11,
        239,
        221,
        238,
    ],
    sbyte: [
        -21,
        126,
        22,
        -126,
        11,
        -17,
        -35,
        -18,
    ],
    u_size: 17212176183586094827,
    s_size: -1234567890123456789,
}"#);
    println!("--------------------------------------");
}

fn longer_union_debug_fmt_main()
{
    println!("longer_union_main");
    use cryptocol::number::LongerUnion;
    let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    println!("a_long = {:?}", a_longer);
    println!("a_long = {:#?}", a_longer);
    #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:?}"), "LongerUnion { this: 216825577908592784562140039541644754667, that: -123456789012345678901234567890123456789, ulonger: 216825577908592784562140039541644754667, slonger: -123456789012345678901234567890123456789, ulong: [6134004772338302699, 11754138130946064698], slong: [6134004772338302699, -6692605942763486918], uint: [1371963115, 1428184279, 2682913082, 2736723546], sint: [1371963115, 1428184279, -1612054214, -1558243750], ushort: [32491, 20934, 23767, 21792, 314, 40938, 5722, 41759], sshort: [32491, 20934, 23767, 21792, 314, -24598, 5722, -23777], ubyte: [235, 126, 198, 81, 215, 92, 32, 85, 58, 1, 234, 159, 90, 22, 31, 163], sbyte: [-21, 126, -58, 81, -41, 92, 32, 85, 58, 1, -22, -97, 90, 22, 31, -93], u_size: [6134004772338302699, 11754138130946064698], s_size: [6134004772338302699, -6692605942763486918] }");
    #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:#?}"), r#"LongerUnion {
    this: 216825577908592784562140039541644754667,
    that: -123456789012345678901234567890123456789,
    ulonger: 216825577908592784562140039541644754667,
    slonger: -123456789012345678901234567890123456789,
    ulong: [
        6134004772338302699,
        11754138130946064698,
    ],
    slong: [
        6134004772338302699,
        -6692605942763486918,
    ],
    uint: [
        1371963115,
        1428184279,
        2682913082,
        2736723546,
    ],
    sint: [
        1371963115,
        1428184279,
        -1612054214,
        -1558243750,
    ],
    ushort: [
        32491,
        20934,
        23767,
        21792,
        314,
        40938,
        5722,
        41759,
    ],
    sshort: [
        32491,
        20934,
        23767,
        21792,
        314,
        -24598,
        5722,
        -23777,
    ],
    ubyte: [
        235,
        126,
        198,
        81,
        215,
        92,
        32,
        85,
        58,
        1,
        234,
        159,
        90,
        22,
        31,
        163,
    ],
    sbyte: [
        -21,
        126,
        -58,
        81,
        -41,
        92,
        32,
        85,
        58,
        1,
        -22,
        -97,
        90,
        22,
        31,
        -93,
    ],
    u_size: [
        6134004772338302699,
        11754138130946064698,
    ],
    s_size: [
        6134004772338302699,
        -6692605942763486918,
    ],
}"#);
    println!("--------------------------------------");
}

#[cfg(target_pointer_width = "64")]
fn size_union_debug_fmt_main()
{
    println!("size_union_main");
    use cryptocol::number::SizeUnion;
    let a_size = SizeUnion::new_with_signed(-1234567890123456789_isize);
    println!("a_size = {:?}", a_size);
    println!("a_size = {:#?}", a_size);
    assert_eq!(format!("{a_size:?}"), "SizeUnion { this: 17212176183586094827, that: -1234567890123456789, u_size: 17212176183586094827, s_size: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18] }");
    assert_eq!(format!("{a_size:#?}"), r#"SizeUnion {
    this: 17212176183586094827,
    that: -1234567890123456789,
    u_size: 17212176183586094827,
    s_size: -1234567890123456789,
    ulong: 17212176183586094827,
    slong: -1234567890123456789,
    uint: [
        2182512363,
        4007522059,
    ],
    sint: [
        -2112454933,
        -287445237,
    ],
    ushort: [
        32491,
        33302,
        61195,
        61149,
    ],
    sshort: [
        32491,
        -32234,
        -4341,
        -4387,
    ],
    ubyte: [
        235,
        126,
        22,
        130,
        11,
        239,
        221,
        238,
    ],
    sbyte: [
        -21,
        126,
        22,
        -126,
        11,
        -17,
        -35,
        -18,
    ],
}"#);
    println!("--------------------------------------");
}
*/


fn end()
{
    calc();
}

fn calc()
{
    println!("For IntUnion");
    use cryptocol::number::{SmallUInt, LongUnion};
    let a = LongUnion::new_with(9876543210123456789_u64);
    let b = LongUnion::new_with(7654321098765432101_u64);
    let (c, d) = a.carrying_mul(b, 0_u64.into_longunion());
    println!("{}_u64 X {}_u64 = ({}_u64, {}_u64)", a, b, d, c);
    println!("a: {}_u32 - {}_u32 = {}_u64", a.get_uint_(1), a.get_uint_(0), a);
    println!("b: {}_u32 - {}_u32 = {}_u64", b.get_uint_(1), b.get_uint_(0), b);
    println!("c: {}_u32 - {}_u32 = {}_u64", c.get_uint_(1), c.get_uint_(0), c);
    println!("d: {}_u32 - {}_u32 = {}_u64", d.get_uint_(1), d.get_uint_(0), d);
    let z = LongerUnion::new_with_ulongs([c.get(), d.get()]);
    println!("z: {}_u128", z);
    for i in 0..4
        { print!("{}_u32 - ", z.get_uint_(3-i)); }
    println!();
    for i in 0..2
    {
        for j in 0..2
        {
            #[allow(unstable_name_collisions)]
            let (e, f) = a.get_uint_(j).carrying_mul(b.get_uint_(i), 0_u32);
            println!("{}_u32 X {}_u32 = ({}_u32, {}_u32)", a.get_uint_(j), b.get_uint_(i), f, e);
        }
    }

    println!("\nFor LongUnion");
    use cryptocol::number::{ LongerUnion, BigUInt };
    let a = LongerUnion::new_with(198765432198765432198765432198765432198_u128);
    let b = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
    let (c, d) = a.carrying_mul(b, 0_u128.into_longerunion());
    println!("{}_u128 X {}_u128 = ({}_u128, {}_u128)", a, b, d, c);
    println!("a: {}_u64 - {}_u64 = {}_u128", a.get_ulong_(1), a.get_ulong_(0), a);
    println!("b: {}_u64 - {}_u64 = {}_u128", b.get_ulong_(1), b.get_ulong_(0), b);
    println!("c: {}_u64 - {}_u64 = {}_u128", c.get_ulong_(1), c.get_ulong_(0), c);
    println!("d: {}_u64 - {}_u64 = {}_u128", d.get_ulong_(1), d.get_ulong_(0), d);
    let z = BigUInt::<u64, 4>::from_array([c.get_ulong_(0), c.get_ulong_(1), d.get_ulong_(0), d.get_ulong_(1)]);
    println!("z: {}_u256", z);
    for i in 0..4
        { print!("{}_u64 - ", z.get_num_(3-i)); }
    println!();
    for i in 0..2
    {
        for j in 0..2
        {
            #[allow(unstable_name_collisions)]
            let (e, f) = a.get_ulong_(j).carrying_mul(b.get_ulong_(i), 0_u64);
            println!("{}_u64 X {}_u64 = ({}_u64, {}_u64)", a.get_ulong_(j), b.get_ulong_(i), f, e);
        }
    }

    println!("\nFor LongerUnion");
    let a = BigUInt::<u128, 4>::from_array([198765432198765432198765432198765432198_u128, 123456789012345678901234567890123456789_u128, 0_u128, 0_u128]);
    let b = BigUInt::<u128, 4>::from_array([135792468013579246801357924680135792468_u128, 75318642097531864209753186420975318642_u128, 0_u128, 0_u128]);
    let c = a.wrapping_mul(&b);
    println!("{}_u256 X {}_u256 = ({}_u256, {}_u256)", a, b, c.get_num_(1), c.get_num_(0));
    println!("a: {}_u128 - {}_u128 = {}_u256", a.get_num_(1), a.get_num_(0), a);
    println!("b: {}_u128 - {}_u128 = {}_u256", b.get_num_(1), b.get_num_(0), b);
    println!("c: {}_u128 - {}_u128 = {}_u256", c.get_num_(1), c.get_num_(0), BigUInt::<u128, 2>::from_array([c.get_num_(0), c.get_num_(1)]));
    println!("d: {}_u128 - {}_u128 = {}_u256", c.get_num_(3), c.get_num_(2), BigUInt::<u128, 2>::from_array([c.get_num_(2), c.get_num_(3)]));
    println!("z: {}_u512", c);
    for i in 0..4
        { print!("{}_u128 - ", c.get_num_(3-i)); }
    println!();
    for i in 0..2
    {
        for j in 0..2
        {
            #[allow(unstable_name_collisions)]
            let (e, f) = a.get_num_(j).carrying_mul(b.get_num_(i), 0_u128);
            println!("{}_u128 X {}_u128 = ({}_u128, {}_u128)", a.get_num_(j), b.get_num_(i), f, e);
        }
    }
}
