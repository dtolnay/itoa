#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

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
    ISIZE(isize),
    USIZE(usize),
}

fuzz_target!(|input: IntegerInput| {
    let mut buffer = itoa::Buffer::new();
    match input {
        IntegerInput::I8(val) => buffer.format(val),
        IntegerInput::U8(val) => buffer.format(val),
        IntegerInput::I16(val) => buffer.format(val),
        IntegerInput::U16(val) => buffer.format(val),
        IntegerInput::I32(val) => buffer.format(val),
        IntegerInput::U32(val) => buffer.format(val),
        IntegerInput::I64(val) => buffer.format(val),
        IntegerInput::U64(val) => buffer.format(val),
        IntegerInput::I128(val) => buffer.format(val),
        IntegerInput::U128(val) => buffer.format(val),
        IntegerInput::ISIZE(val) => buffer.format(val),
        IntegerInput::USIZE(val) => buffer.format(val),
    };
});
