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

const BITS: u32 = 128;
const BITS_HALF: u32 = 64;

trait LargeInt {
    fn low(self) -> u64;
    fn high(self) -> u64;
    fn from_parts(low: u64, high: u64) -> Self;
}

trait Int {
    fn aborting_div(self, other: Self) -> Self;
    fn aborting_rem(self, other: Self) -> Self;
}

impl LargeInt for u128 {
    fn low(self) -> u64 {
        self as u64
    }

    fn high(self) -> u64 {
        (self >> 64) as u64
    }

    fn from_parts(low: u64, high: u64) -> u128 {
        low as u128 | ((high as u128) << 64)
    }
}

impl Int for u64 {
    fn aborting_div(self, other: u64) -> u64 {
        <u64>::checked_div(self, other).unwrap()
    }

    fn aborting_rem(self, other: u64) -> u64 {
        <u64>::checked_rem(self, other).unwrap()
    }
}

pub fn udivmodti4(n: u128, d: u128, rem: Option<&mut u128>) -> u128 {
    // NOTE X is unknown, K != 0
    if n.high() == 0 {
        if d.high() == 0 {
            // 0 X
            // ---
            // 0 X

            if let Some(rem) = rem {
                *rem = <u128>::from(n.low().aborting_rem(d.low()));
            }
            return <u128>::from(n.low().aborting_div(d.low()))
        } else {
            // 0 X
            // ---
            // K X
            if let Some(rem) = rem {
                *rem = n;
            }
            return 0;
        };
    }

    let mut sr;
    let mut q;
    let mut r;

    if d.low() == 0 {
        if d.high() == 0 {
            // K X
            // ---
            // 0 0
            // NOTE This should be unreachable in safe Rust because the program will panic before
            // this intrinsic is called
            unreachable!();
        }

        if n.low() == 0 {
            // K 0
            // ---
            // K 0
            if let Some(rem) = rem {
                *rem = <u128>::from_parts(0, n.high().aborting_rem(d.high()));
            }
            return <u128>::from(n.high().aborting_div(d.high()))
        }

        // K K
        // ---
        // K 0

        if d.high().is_power_of_two() {
            if let Some(rem) = rem {
                *rem = <u128>::from_parts(n.low(), n.high() & (d.high() - 1));
            }
            return <u128>::from(n.high() >> d.high().trailing_zeros());
        }

        sr = d.high().leading_zeros().wrapping_sub(n.high().leading_zeros());

        // D > N
        if sr > BITS_HALF - 2 {
            if let Some(rem) = rem {
                *rem = n;
            }
            return 0;
        }

        sr += 1;

        // 1 <= sr <= BITS_HALF - 1
        q = n << (BITS - sr);
        r = n >> sr;
    } else if d.high() == 0 {
        // K X
        // ---
        // 0 K
        if d.low().is_power_of_two() {
            if let Some(rem) = rem {
                *rem = <u128>::from(n.low() & (d.low() - 1));
            }

            if d.low() == 1 {
                return n;
            } else {
                let sr = d.low().trailing_zeros();
                return n >> sr;
            };
        }

        sr = 1 + BITS_HALF + d.low().leading_zeros() - n.high().leading_zeros();

        // 2 <= sr <= u64::BITS - 1
        q = n << (BITS - sr);
        r = n >> sr;
    } else {
        // K X
        // ---
        // K K
        sr = d.high().leading_zeros().wrapping_sub(n.high().leading_zeros());

        // D > N
        if sr > BITS_HALF - 1 {
            if let Some(rem) = rem {
                *rem = n;
            }
            return 0;
        }

        sr += 1;

        // 1 <= sr <= BITS_HALF
        q = n << (BITS - sr);
        r = n >> sr;
    }

    // Not a special case
    // q and r are initialized with
    // q = n << (u64::BITS - sr)
    // r = n >> sr
    // 1 <= sr <= u64::BITS - 1
    let mut carry = 0;

    // Don't use a range because they may generate references to memcpy in unoptimized code
    let mut i = 0;
    while i < sr {
        i += 1;

        // r:q = ((r:q) << 1) | carry
        r = (r << 1) | (q >> (BITS - 1));
        q = (q << 1) | carry as u128;

        // carry = 0
        // if r >= d {
        //     r -= d;
        //     carry = 1;
        // }
        let s = (d.wrapping_sub(r).wrapping_sub(1)) as i128 >> (BITS - 1);
        carry = (s & 1) as u64;
        r -= d & s as u128;
    }

    if let Some(rem) = rem {
        *rem = r;
    }
    (q << 1) | carry as u128
}

#[cfg(test)]
#[test]
fn test_udivmodti4() {
    let primes = [
          3,   7,  31,  73, 127, 179, 233, 283, 353,
        419, 467, 547, 607, 661, 739, 811, 877, 947,
    ];

    for (i, d) in (0..128).cycle().zip(primes.iter().cycle()).take(1_000) {
        let n = 1u128 << i;
        let mut rem = 0;
        let q = udivmodti4(n, *d, Some(&mut rem));
        assert_eq!(q, n / d);
        assert_eq!(rem, n % d);
    }
}
