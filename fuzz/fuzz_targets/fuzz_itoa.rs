#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::mem;

#[derive(Arbitrary, Debug)]
enum IntegerInput {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Isize(isize),
    Usize(usize),
}

macro_rules! test_itoa {
    ($val:expr) => {
        match $val {
            val => {
                let mut buffer = itoa::Buffer::new();
                let string = buffer.format(val);
                assert!(string.len() <= mem::size_of::<itoa::Buffer>());
                assert_eq!(val, string.parse().unwrap());
            }
        }
    };
}

fuzz_target!(|input: IntegerInput| {
    match input {
        IntegerInput::I8(val) => test_itoa!(val),
        IntegerInput::U8(val) => test_itoa!(val),
        IntegerInput::I16(val) => test_itoa!(val),
        IntegerInput::U16(val) => test_itoa!(val),
        IntegerInput::I32(val) => test_itoa!(val),
        IntegerInput::U32(val) => test_itoa!(val),
        IntegerInput::I64(val) => test_itoa!(val),
        IntegerInput::U64(val) => test_itoa!(val),
        IntegerInput::I128(val) => test_itoa!(val),
        IntegerInput::U128(val) => test_itoa!(val),
        IntegerInput::Isize(val) => test_itoa!(val),
        IntegerInput::Usize(val) => test_itoa!(val),
    }
});
