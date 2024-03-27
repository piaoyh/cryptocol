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
    size_union_new_with_u128();
    size_union_new_with_bool();
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
    unions_get_ubyte();
    unions_set_ubyte();
    unions_get_sbyte_();
    unions_set_sbyte_();
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

fn unions_set_sbyte_() {}
fn unions_get_sbyte() {}
fn unions_set_sbyte() {}

/*
    match a_short.get_ubyte(1)
    {
        Some(b) =>  {
                println!("a_short.get_ubyte(1) = {}", b);
                assert_eq!(b, 11_u8);
            },
        None =>     { println!("Out of range"); },
    }
    
    match a_short.get_ubyte(2)
    {
        Some(b) =>  { println!("a_short.get_ubyte(2) = {}", b); },
        None =>     {
                println!("Out of range");
                assert_eq!(a_short.get_ubyte(2), None);
            },
    }
    println!("--------------------------------------");
}


fn short_union_get_sbyte_main()
{
    println!("short_union_get_sbyte_main");
    use cryptocol::number::ShortUnion;
    let a_short = ShortUnion::new_with(2895_u16);
    let b_short_i8 = a_short.get_sbyte_(1);
    println!("a_short.get_sbyte_(1) = {}", b_short_i8);
    assert_eq!(b_short_i8, 11_i8);
    // It will panic.
    // let c_short = a_short.get_sbyte_(2);

    match a_short.get_sbyte(1)
    {
        Some(b) =>  {
                println!("a_short.get_sbyte(1) = {}", b);
                assert_eq!(b, 11_i8);
            },
        None =>     { println!("Out of range"); },
    }
    
    match a_short.get_sbyte(2)
    {
        Some(b) =>  { println!("a_short.get_sbyte(2) = {}", b); },
        None =>     {
                println!("Out of range");
                assert_eq!(a_short.get_sbyte(2), None);
            },
    }
    println!("--------------------------------------");
}


fn short_union_set_ubyte_main()
{
    println!("short_union_set_ubyte_main");
    use cryptocol::number::ShortUnion;
    let mut a_short = ShortUnion::new_with(2895_u16);
    let mut b_short_u8 = a_short.get_ubyte_(1);
    println!("a_short.get_ubyte_(1) = {}", b_short_u8);
    a_short.set_ubyte_(1, 0);
    b_short_u8 = a_short.get_ubyte_(1);
    println!("a_short.get() = {}, a_short.get_ubyte_(1) = {}", a_short, b_short_u8);
    assert_eq!(a_short.get(), 79_u16);
    assert_eq!(b_short_u8, 0_u8);
    // It will panic.
    // let c_short = a_short.set_ubyte_(2, 0);

    let mut succ = a_short.set_ubyte(1, 11);
    let mut ubyte = a_short.get_ubyte(1);
    if succ
    {
        println!("a_short.get() = {}, a_short.get_ubyte(1).unwrap() = {}", a_short, ubyte.unwrap());
        assert_eq!(ubyte.unwrap(), 11_u8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(ubyte, None);
    }

    succ = a_short.set_ubyte(2, 11);
    ubyte = a_short.get_ubyte(2);
    if succ
    {
        println!("a_short.get() = {}, a_short.get_ubyte(2).unwrap() = {}", a_short, ubyte.unwrap());
        assert_eq!(ubyte.unwrap(), 11_u8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(ubyte, None);
    }
    println!("--------------------------------------");
}


fn short_union_set_sbyte_main()
{
    println!("short_union_set_sbyte_main");
    use cryptocol::number::ShortUnion;
    let mut a_short = ShortUnion::new_with_signed(79_i16);
    let mut b_short_i8 = a_short.get_sbyte_(1);
    println!("a_short.get_sbyte_(1) = {}", b_short_i8);
    a_short.set_sbyte_(1, 0);
    b_short_i8 = a_short.get_sbyte_(1);
    println!("a_short.get_signed() = {}, a_short.get_sbyte_(1) = {}", a_short.get_signed(), b_short_i8);
    assert_eq!(a_short.get_signed(), 79_i16);
    assert_eq!(b_short_i8, 0_i8);

    // It will panic.
    // let c_short = a_short.set_sbyte_(2, 0);

    let mut succ = a_short.set_sbyte(1, 11);
    let mut sbyte = a_short.get_sbyte(1);
    if succ
    {
        println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        assert_eq!(sbyte.unwrap(), 11_i8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(sbyte, None);
    }

    succ = a_short.set_sbyte(2, 11);
    sbyte = a_short.get_sbyte(2);
    if succ
    {
        println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        assert_eq!(sbyte.unwrap(), 11_i8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(sbyte, None);
    }
    println!("--------------------------------------");
}


fn longer_union_get_ubyte_main()
{
    println!("longer_union_get_sbyte_main");
    use cryptocol::number::LongerUnion;
    let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    let b_longer_u8 = a_longer.get_ubyte_(3);
    println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
    assert_eq!(b_longer_u8, 241_u8);
    // It will panic.
    // let c_longer = a_longer.get_ubyte_(16);

    match a_longer.get_ubyte(3)
    {
        Some(b) =>  {
                println!("a_longer.get_ubyte(3) = {}", b);
                assert_eq!(b, 241_u8);
            },
        None =>     { println!("Out of range"); },
    }
    
    match a_longer.get_ubyte(16)
    {
        Some(b) =>  { println!("a_short.get_ubyte(16) = {}", b); },
        None =>     {
                println!("Out of range");
                assert_eq!(a_longer.get_ubyte(16), None);
            },
    }
    println!("--------------------------------------");
}

fn longer_union_get_sbyte_main()
{
    println!("longer_union_get_sbyte_main");
    use cryptocol::number::LongerUnion;
    let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    let b_longer_i8 = a_longer.get_sbyte_(3);
    println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
    assert_eq!(b_longer_i8, 81_i8);

    // It will panic.
    // let c_longer = a_longer.get_sbyte_(16);

    match a_longer.get_sbyte(3)
    {
        Some(b) =>  {
                println!("a_longer.get_sbyte(3) = {}", b);
                assert_eq!(b, 81_i8);
            },
        None =>     { println!("Out of range"); },
    }
    
    match a_longer.get_sbyte(16)
    {
        Some(b) =>  { println!("a_short.get_sbyte(16) = {}", b); },
        None =>     {
                println!("Out of range");
                assert_eq!(a_longer.get_sbyte(16), None);
            },
    }
    println!("--------------------------------------");
}


fn longer_union_set_ubyte_main()
{
    println!("longer_union_set_ubyte_main");
    use cryptocol::number::LongerUnion;
    let mut a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
    let mut b_longer_u8 = a_longer.get_ubyte_(3);
    println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
    a_longer.set_ubyte_(3, 0);
    b_longer_u8 = a_longer.get_ubyte_(3);
    println!("a_longer.get() = {}, a_longer.get_ubyte_(3) = {}", a_longer, b_longer_u8);
    assert_eq!(a_longer.get(), 339047799029950809142362261748737248079_u128);
    assert_eq!(a_longer.get_ubyte_(3), 0_u8);
    
    // It will panic.
    // let c_longer = a_longer.get_ubyte_(16);

    let mut succ = a_longer.set_ubyte(3, 241_u8);
    let mut ubyte = a_longer.get_ubyte(3);
    if succ
    {
        println!("a_longer.get() = {}, a_longer.get_ubyte(3).unwrap() = {}", a_longer, ubyte.unwrap());
        assert_eq!(ubyte.unwrap(), 241_u8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(ubyte, None);
    }

    succ = a_longer.set_ubyte(16, 241_u8);
    ubyte = a_longer.get_ubyte(16);
    if succ
    {
        println!("a_longer.get() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer, ubyte.unwrap());
        assert_eq!(ubyte.unwrap(), 241_u8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(ubyte, None);
    }
    println!("--------------------------------------");
}


fn longer_union_set_sbyte_main()
{
    println!("longer_union_set_sbyte_main");
    use cryptocol::number::LongerUnion;
    let mut a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    let mut b_longer_i8 = a_longer.get_sbyte_(3);
    println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
    a_longer.set_sbyte_(3, 0);
    b_longer_i8 = a_longer.get_sbyte_(3);
    println!("a_longer.get_signed() = {}, a_longer.get_sbyte_(3) = {}", a_longer.get_signed(), b_longer_i8);
    assert_eq!(a_longer.get_signed(), -123456789012345678901234567891482411285_i128);
    assert_eq!(a_longer.get_ubyte_(3), 0_u8);
    
    // It will panic.
    // let c_longer = a_longer.get_sbyte_(16);

    let mut succ = a_longer.set_sbyte(3, 81_i8);
    let mut sbyte = a_longer.get_sbyte(3);
    if succ
    {
        println!("a_longer.get_signed() = {}, a_longer.get_sbyte(3).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        assert_eq!(sbyte.unwrap(), 81_i8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(sbyte, None);
    }

    succ = a_longer.set_sbyte(16, 81_i8);
    sbyte = a_longer.get_sbyte(16);
    if succ
    {
        println!("a_longer.get_signed() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        assert_eq!(sbyte.unwrap(), 81_i8);
    }
    else
    {
        println!("Out of range");
        assert_eq!(sbyte, None);
    }
    println!("--------------------------------------");
}


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