#![allow(non_snake_case)]

macro_rules! test {
    ($($name:ident($value:expr, $expected:expr))*) => {
        $(
            #[test]
            fn $name() {
                let mut buffer = itoa::Buffer::new();
                let s = buffer.format($value);
                assert_eq!(s, $expected);
            }
        )*
    }
}

test! {
    test_u8_0(0u8, "0")
    test_u8_9(9u8, "9")
    test_u8_10(10u8, "10")
    test_u8_99(99u8, "99")
    test_u8_100(100u8, "100")
    test_u8_max(u8::MAX, "255")

    test_i8_0(0i8, "0")
    test_i8_9(9i8, "9")
    test_i8_10(10i8, "10")
    test_i8_99(99i8, "99")
    test_i8_min(i8::MIN, "-128")
    test_i8_max(i8::MAX, "127")

    test_u16_0(0u16, "0")
    test_u16_9(9u16, "9")
    test_u16_10(10u16, "10")
    test_u16_99(99u16, "99")
    test_u16_100(100u16, "100")
    test_u16_max(u16::MAX, "65535")

    test_i16_0(0i16, "0")
    test_i16_9(9i16, "9")
    test_i16_10(10i16, "10")
    test_i16_99(99i16, "99")
    test_i16_100(100i16, "100")
    test_i16_min(i16::MIN, "-32768")
    test_i16_max(i16::MAX, "32767")

    test_u32_0(0u32, "0")
    test_u32_9(9u32, "9")
    test_u32_10(10u32, "10")
    test_u32_99(99u32, "99")
    test_u32_100(100u32, "100")
    test_u32_max(u32::MAX, "4294967295")

    test_i32_0(0i32, "0")
    test_i32_9(9i32, "9")
    test_i32_10(10i32, "10")
    test_i32_99(99i32, "99")
    test_i32_100(100i32, "100")
    test_i32_min(i32::MIN, "-2147483648")
    test_i32_max(i32::MAX, "2147483647")

    test_u64_0(0u64, "0")
    test_u64_9(9u64, "9")
    test_u64_10(10u64, "10")
    test_u64_99(99u64, "99")
    test_u64_100(100u64, "100")
    test_u64_half(u64::from(u32::MAX), "4294967295")
    test_u64_max(u64::MAX, "18446744073709551615")

    test_i64_0(0i64, "0")
    test_i64_9(9i64, "9")
    test_i64_10(10i64, "10")
    test_i64_99(99i64, "99")
    test_i64_100(100i64, "100")
    test_i64_min(i64::MIN, "-9223372036854775808")
    test_i64_max(i64::MAX, "9223372036854775807")

    test_u128_0(0u128, "0")
    test_u128_9(9u128, "9")
    test_u128_10(10u128, "10")
    test_u128_99(99u128, "99")
    test_u128_100(100u128, "100")
    test_u128_max(u128::MAX, "340282366920938463463374607431768211455")

    test_i128_0(0i128, "0")
    test_i128_9(9i128, "9")
    test_i128_10(10i128, "10")
    test_i128_99(99i128, "99")
    test_i128_100(100i128, "100")
    test_i128_min(i128::MIN, "-170141183460469231731687303715884105728")
    test_i128_max(i128::MAX, "170141183460469231731687303715884105727")
}

#[test]
fn test_max_str_len() {
    use itoa::Integer as _;

    assert_eq!(i8::MAX_STR_LEN, 4);
    assert_eq!(u8::MAX_STR_LEN, 3);
    assert_eq!(i16::MAX_STR_LEN, 6);
    assert_eq!(u16::MAX_STR_LEN, 5);
    assert_eq!(i32::MAX_STR_LEN, 11);
    assert_eq!(u32::MAX_STR_LEN, 10);
    assert_eq!(i64::MAX_STR_LEN, 20);
    assert_eq!(u64::MAX_STR_LEN, 20);
    assert_eq!(i128::MAX_STR_LEN, 40);
    assert_eq!(u128::MAX_STR_LEN, 39);
}
