#![allow(non_snake_case)]
#![allow(clippy::cast_lossless)]

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
    test_u64_0(0u64, "0")
    test_u64_half(u32::MAX as u64, "4294967295")
    test_u64_max(u64::MAX, "18446744073709551615")
    test_i64_min(i64::MIN, "-9223372036854775808")

    test_i16_0(0i16, "0")
    test_i16_min(i16::MIN, "-32768")

    test_u128_0(0u128, "0")
    test_u128_max(u128::MAX, "340282366920938463463374607431768211455")
    test_i128_min(i128::MIN, "-170141183460469231731687303715884105728")
    test_i128_max(i128::MAX, "170141183460469231731687303715884105727")
}
