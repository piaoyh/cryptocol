// Copyright 2023, 2024 PARK Youngho.hort_union_
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]

use cryptocol::number::SmallUInt;
#[allow(non_camel_case_types)]


pub fn main()
{
    short_union_quick_start();
    int_union_quick_start();
    long_union_quick_start();
    // longer_union_main();
    // size_union_main();

    // short_union_debug_fmt_main();
    // int_union_debug_fmt_main();
    // long_union_debug_fmt_main();
    // longer_union_debug_fmt_main();
    // size_union_debug_fmt_main();

    // short_union_get_ubyte_main();
    // short_union_get_sbyte_main();
    // short_union_set_ubyte_main();
    // short_union_set_sbyte_main();

    // longer_union_get_ubyte_main();
    // longer_union_get_sbyte_main();
    // longer_union_set_ubyte_main();
    // longer_union_set_sbyte_main();
}

fn short_union_quick_start()
{
    short_union_quick_start1();
    short_union_quick_start2();
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

fn int_union_quick_start()
{
    int_union_quick_start1();
    int_union_quick_start2();
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

fn long_union_quick_start()
{
    long_union_quick_start1();
    // long_union_quick_start2();
}

fn long_union_quick_start1()
{
    println!("long_union_quick_start1()");
    use cryptocol::number::LongUnion;

    let a = LongUnion::new_with_signed(-1234567890987645_i64);
    println!("a.get() = {}", a.get());
    println!("a.get_signed() = {}", a.get_signed());
    println!("a.get_ulong_() = {}", a.get_ulong());
    println!("a.get_slong_() = {}", a.get_slong());
    assert_eq!(a.get(), 18445509505818563971_u64);
    assert_eq!(a.get_signed(), -1234567890987645_i64);
    assert_eq!(a.get_ulong(), 18445509505818563971_u64);
    assert_eq!(a.get_slong(), -1234567890987645_i64);
    #[cfg(target_endian = "little")]
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
    }
    #[cfg(target_pointer_width = "64")]
    {
        println!("a.get_usize() = {}", a.get_usize());
        println!("a.get_ssize() = {}", a.get_ssize());
        assert_eq!(a.get_usize(), 18445509505818563971_usize);
        assert_eq!(a.get_ssize(), -1234567890987645_isize);
    }
    println!("--------------------------------------");
}

/*
fn longer_union_main()
{
    println!("longer_union_main()");
    use cryptocol::number::LongerUnion;
    let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);
    println!("a.this = {}, {}", unsafe { a.this }, a.get());
    println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
    println!("a.ulonger = {}", unsafe { a.ulonger });
    println!("a.slonger = {}", unsafe { a.slonger });
    #[cfg(target_endian = "little")]
    {
        for i in 0..2
            { println!("a.ulong[{}] = {}, {}", i, unsafe { a.ulong[i] }, a.get_ulong_(i)); }
        for i in 0..2
            { println!("a.slong[{}] = {}, {}", i, unsafe { a.slong[i] }, a.get_slong_(i)); }
        for i in 0..4
            { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
        for i in 0..4
            { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
        for i in 0..8
            { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
        for i in 0..8
            { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
        for i in 0..16
            { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
        for i in 0..16
            { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
        #[cfg(target_pointer_width = "64")]
        {
            const N: usize = 2;
            for i in 0..N
                { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
        }
        #[cfg(target_pointer_width = "32")]
        {
            const N: usize = 4;
            for i in 0..N
                { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
        }
        #[cfg(target_pointer_width = "16")]
        {
            const N: usize = 8;
            for i in 0..N
                { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
        }
        #[cfg(target_pointer_width = "8")]
        {
            const N: usize = 16;
            for i in 0..N
                { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
            for i in 0..N
                { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
        }
    }
    #[cfg(target_pointer_width = "128")]
    {
        println!("a.u_size = {}", unsafe { a.u_size });
        println!("a.s_size = {}", unsafe { a.s_size });
    }
    assert_eq!(unsafe { a.this }, 339047799029950809142362261752780557135_u128);
    assert_eq!(unsafe { a.that }, -1234567890987654321012345678987654321_i128);
    assert_eq!(unsafe { a.ulonger }, 339047799029950809142362261752780557135_u128);
    assert_eq!(unsafe { a.slonger }, -1234567890987654321012345678987654321_i128);
    #[cfg(target_endian = "little")]
    {
        assert_eq!(unsafe { a.ulong[0] }, 13664881099896654671_u64);
        assert_eq!(unsafe { a.ulong[1] }, 18379818014235068504_u64);
        assert_eq!(unsafe { a.slong[0] }, -4781862973812896945_i64);
        assert_eq!(unsafe { a.slong[1] }, -66926059474483112_i64);
        assert_eq!(unsafe { a.uint[0] }, 4048161615_u32);
        assert_eq!(unsafe { a.uint[1] }, 3181603061_u32);
        assert_eq!(unsafe { a.uint[2] }, 2127464536_u32);
        assert_eq!(unsafe { a.uint[3] }, 4279384858_u32);
        assert_eq!(unsafe { a.sint[0] }, -246805681_i32);
        assert_eq!(unsafe { a.sint[1] }, -1113364235_i32);
        assert_eq!(unsafe { a.sint[2] }, 2127464536_i32);
        assert_eq!(unsafe { a.sint[3] }, -15582438_i32);
        assert_eq!(unsafe { a.ushort[0] }, 2895_u16);
        assert_eq!(unsafe { a.ushort[1] }, 61770_u16);
        assert_eq!(unsafe { a.ushort[2] }, 26869_u16);
        assert_eq!(unsafe { a.ushort[3] }, 48547_u16);
        assert_eq!(unsafe { a.ushort[4] }, 34904_u16);
        assert_eq!(unsafe { a.ushort[5] }, 32462_u16);
        assert_eq!(unsafe { a.ushort[6] }, 15130_u16);
        assert_eq!(unsafe { a.ushort[7] }, 65298_u16);
        assert_eq!(unsafe { a.sshort[0] }, 2895_i16);
        assert_eq!(unsafe { a.sshort[1] }, -3766_i16);
        assert_eq!(unsafe { a.sshort[2] }, 26869_i16);
        assert_eq!(unsafe { a.sshort[3] }, -16989_i16);
        assert_eq!(unsafe { a.sshort[4] }, -30632_i16);
        assert_eq!(unsafe { a.sshort[5] }, 32462_i16);
        assert_eq!(unsafe { a.sshort[6] }, 15130_i16);
        assert_eq!(unsafe { a.sshort[7] }, -238_i16);
        assert_eq!(unsafe { a.ubyte[0] }, 79_u8);
        assert_eq!(unsafe { a.ubyte[1] }, 11_u8);
        assert_eq!(unsafe { a.ubyte[2] }, 74_u8);
        assert_eq!(unsafe { a.ubyte[3] }, 241_u8);
        assert_eq!(unsafe { a.ubyte[4] }, 245_u8);
        assert_eq!(unsafe { a.ubyte[5] }, 104_u8);
        assert_eq!(unsafe { a.ubyte[6] }, 163_u8);
        assert_eq!(unsafe { a.ubyte[7] }, 189_u8);
        assert_eq!(unsafe { a.ubyte[8] }, 88_u8);
        assert_eq!(unsafe { a.ubyte[9] }, 136_u8);
        assert_eq!(unsafe { a.ubyte[10] }, 206_u8);
        assert_eq!(unsafe { a.ubyte[11] }, 126_u8);
        assert_eq!(unsafe { a.ubyte[12] }, 26_u8);
        assert_eq!(unsafe { a.ubyte[13] }, 59_u8);
        assert_eq!(unsafe { a.ubyte[14] }, 18_u8);
        assert_eq!(unsafe { a.ubyte[15] }, 255_u8);
        assert_eq!(unsafe { a.sbyte[0] }, 79_i8);
        assert_eq!(unsafe { a.sbyte[1] }, 11_i8);
        assert_eq!(unsafe { a.sbyte[2] }, 74_i8);
        assert_eq!(unsafe { a.sbyte[3] }, -15_i8);
        assert_eq!(unsafe { a.sbyte[4] }, -11_i8);
        assert_eq!(unsafe { a.sbyte[5] }, 104_i8);
        assert_eq!(unsafe { a.sbyte[6] }, -93_i8);
        assert_eq!(unsafe { a.sbyte[7] }, -67_i8);
        assert_eq!(unsafe { a.sbyte[8] }, 88_i8);
        assert_eq!(unsafe { a.sbyte[9] }, -120_i8);
        assert_eq!(unsafe { a.sbyte[10] }, -50_i8);
        assert_eq!(unsafe { a.sbyte[11] }, 126_i8);
        assert_eq!(unsafe { a.sbyte[12] }, 26_i8);
        assert_eq!(unsafe { a.sbyte[13] }, 59_i8);
        assert_eq!(unsafe { a.sbyte[14] }, 18_i8);
        assert_eq!(unsafe { a.sbyte[15] }, -1_i8);
    }
    println!("--------------------------------------");
}


fn size_union_main()
{
    println!("size_union_main()");
    use cryptocol::number::SizeUnion;
    #[cfg(target_pointer_width = "128")]
    let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
    #[cfg(target_pointer_width = "64")]
    let a = SizeUnion::new_with_signed(-1234567890123456789_isize);
    #[cfg(target_pointer_width = "32")]
    let a = SizeUnion::new_with_signed(2112454933_isize);
    #[cfg(target_pointer_width = "16")]
    let a = SizeUnion::new_with_signed(32491_isize);
    #[cfg(target_pointer_width = "8")]
    let a = SizeUnion::new_with_signed(-21_isize);
    println!("a.this = {}, {}", unsafe { a.this }, a.get());
    println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
    println!("a.u_size = {}, {}", unsafe { a.u_size }, a.get());
    println!("a.s_size = {}, {}", unsafe { a.s_size }, a.get_signed());
    #[cfg(target_endian = "little")]
    {
        #[cfg(target_pointer_width = "128")]
        {
            println!("a.ulonger = {}, {}", unsafe { a.ulonger }, a.get());
            println!("a.slonger = {}, {}", unsafe { a.slonger }, a.get_signed());
            for i in 0..2
                { println!("a.ulong[{}] = {}, {}", i, unsafe { a.ulong[i] }, a.get_ulong_(i)); }
            for i in 0..2
                { println!("a.slong[{}] = {}, {}", i, unsafe { a.slong[i] }, a.get_slong_(i)); }
            for i in 0..4
                { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
            for i in 0..4
                { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
            for i in 0..8
                { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
            for i in 0..8
                { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
            for i in 0..16
                { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
            for i in 0..16
                { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
        }
        #[cfg(target_pointer_width = "64")]
        {
            println!("a.ulong = {}, {}", unsafe { a.ulong }, a.get());
            println!("a.slong = {}, {}", unsafe { a.slong }, a.get_signed());
            for i in 0..2
                { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
            for i in 0..2
                { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
            for i in 0..4
                { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
            for i in 0..4
                { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
            for i in 0..8
                { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
            for i in 0..8
                { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
        }
        #[cfg(target_pointer_width = "32")]
        {
            println!("a.uint = {}, {}", unsafe { a.uint }, a.get());
            println!("a.sint = {}, {}", unsafe { a.sint }, a.get_signed());
            for i in 0..2
                { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
            for i in 0..2
                { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
            for i in 0..4
                { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
            for i in 0..4
                { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
        }
        #[cfg(target_pointer_width = "16")]
        {
            println!("a.ushort = {}, {}", unsafe { a.ushort }, a.get());
            println!("a.sshort = {}, {}", unsafe { a.sshort }, a.get_signed());
            for i in 0..2
                { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
            for i in 0..2
                { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
        }
    }
    #[cfg(target_pointer_width = "8")]
    {
        println!("a.ubyte = {}, {}", unsafe { a.ubyte }, a.get());
        println!("a.sbyte = {}, {}", unsafe { a.sbyte }, a.get_signed());
    }

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(unsafe { a.this }, 17212176183586094827_usize);
        assert_eq!(unsafe { a.that }, -1234567890123456789_isize);
        assert_eq!(unsafe { a.u_size }, 17212176183586094827_usize);
        assert_eq!(unsafe { a.s_size }, -1234567890123456789_isize);
        assert_eq!(unsafe { a.ulong }, 17212176183586094827_u64);
        assert_eq!(unsafe { a.slong }, -1234567890123456789_i64);
    }

    #[cfg(target_endian = "little")]
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(unsafe { a.uint[0] }, 2182512363_u32);
        assert_eq!(unsafe { a.uint[1] }, 4007522059_u32);
        assert_eq!(unsafe { a.sint[0] }, -2112454933_i32);
        assert_eq!(unsafe { a.sint[1] }, -287445237_i32);
        assert_eq!(unsafe { a.ushort[0] }, 32491_u16);
        assert_eq!(unsafe { a.ushort[1] }, 33302_u16);
        assert_eq!(unsafe { a.ushort[2] }, 61195_u16);
        assert_eq!(unsafe { a.ushort[3] }, 61149_u16);
        assert_eq!(unsafe { a.sshort[0] }, 32491_i16);
        assert_eq!(unsafe { a.sshort[1] }, -32234_i16);
        assert_eq!(unsafe { a.sshort[2] }, -4341_i16);
        assert_eq!(unsafe { a.sshort[3] }, -4387_i16);
        assert_eq!(unsafe { a.ubyte[0] }, 235_u8);
        assert_eq!(unsafe { a.ubyte[1] }, 126_u8);
        assert_eq!(unsafe { a.ubyte[2] }, 22_u8);
        assert_eq!(unsafe { a.ubyte[3] }, 130_u8);
        assert_eq!(unsafe { a.ubyte[4] }, 11_u8);
        assert_eq!(unsafe { a.ubyte[5] }, 239_u8);
        assert_eq!(unsafe { a.ubyte[6] }, 221_u8);
        assert_eq!(unsafe { a.ubyte[7] }, 238_u8);
        assert_eq!(unsafe { a.sbyte[0] }, -21_i8);
        assert_eq!(unsafe { a.sbyte[1] }, 126_i8);
        assert_eq!(unsafe { a.sbyte[2] }, 22_i8);
        assert_eq!(unsafe { a.sbyte[3] }, -126_i8);
        assert_eq!(unsafe { a.sbyte[4] }, 11_i8);
        assert_eq!(unsafe { a.sbyte[5] }, -17_i8);
        assert_eq!(unsafe { a.sbyte[6] }, -35_i8);
        assert_eq!(unsafe { a.sbyte[7] }, -18_i8);
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


fn short_union_get_ubyte_main()
{
    println!("short_union_get_ubyte_main");
    use cryptocol::number::ShortUnion;
    let a_short = ShortUnion::new_with(2895_u16);
    let b_short_u8 = a_short.get_ubyte_(1);
    println!("a_short.get_ubyte_(1) = {}", b_short_u8);
    assert_eq!(b_short_u8, 11_u8);
    // It will panic.
    // let c_short = a_short.get_ubyte_(2);

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
*/