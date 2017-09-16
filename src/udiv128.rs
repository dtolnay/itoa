// Copyright 2009-2016 compiler-builtins Developers
//
// The compiler-builtins crate is dual licensed under both the University of
// Illinois "BSD-Like" license and the MIT license.  As a user of this code you may
// choose to use it under either license. As a contributor, you agree to allow
// your code to be used under both.
//
// Full text of the relevant licenses is found here:
// https://github.com/rust-lang-nursery/compiler-builtins/blob/master/LICENSE.TXT
//
//
//
// The following code is based on Rust’s [compiler-builtins crate]
// (https://github.com/rust-lang-nursery/compiler-builtins) which
// provides runtime functions for the Rust programs. The Rust
// compiler will automatically link your programs against this crate.
//
// We copied the implementation of '__udivmodti4()' which is an intrinsic
// implementing division with remainder for architectures without 128-bit integer support.
// We have done this two reasons, to work around [bad optimization by LLVM]
// (https://github.com/rust-lang/rust/issues/44545) and to allow function
// inlining which doesn’t happen with the intrinsic.

pub fn udivmod_10000(n: u128) -> (u128, isize) {
    let high = (n >> 64) as u64;
    if high == 0 {
        let low = n as u64;
        return ((low / 10000) as u128, (low % 10000) as isize);
    }

    let leading_zeros_10000 = 114;
    debug_assert_eq!(leading_zeros_10000, 10000u128.leading_zeros());
    let sr = 1 + leading_zeros_10000 - high.leading_zeros();

    // 52 <= sr <= 115
    let mut q: u128 = n << (128 - sr);
    let mut r: u128 = n >> sr;
    let mut carry: u64 = 0;

    // Don't use a range because they may generate references to memcpy in unoptimized code
    //
    // Loop invariants:  r < 10000; carry is 0 or 1
    let mut i = 0;
    while i < sr {
        i += 1;

        // r:q = ((r:q) << 1) | carry
        r = (r << 1) | (q >> 127);
        q = (q << 1) | carry as u128;

        // carry = 0
        // if r >= 10000 {
        //     r -= 10000;
        //     carry = 1;
        // }
        let s = 10000u128.wrapping_sub(r).wrapping_sub(1) as i128 >> 127;
        carry = (s & 1) as u64;
        r -= 10000u128 & s as u128;
    }

    ((q << 1) | carry as u128, r as isize)
}
