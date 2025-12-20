#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/// Multiply unsigned 128 bit integers, return upper 128 bits of the result
#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
fn u128_mulhi(x: u128, y: u128) -> u128 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let y_lo = y as u64;
    let y_hi = (y >> 64) as u64;

    // handle possibility of overflow
    let carry = (u128::from(x_lo) * u128::from(y_lo)) >> 64;
    let m = u128::from(x_lo) * u128::from(y_hi) + carry;
    let high1 = m >> 64;

    let m_lo = m as u64;
    let high2 = (u128::from(x_hi) * u128::from(y_lo) + u128::from(m_lo)) >> 64;

    u128::from(x_hi) * u128::from(y_hi) + high1 + high2
}

/// Divide `n` by 1e19 and return quotient and remainder
///
/// Integer division algorithm is based on the following paper:
///
///   T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”
///   in Proc. of the SIGPLAN94 Conference on Programming Language Design and
///   Implementation, 1994, pp. 61–72
///
#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
pub fn udivmod_1e19(n: u128) -> (u128, u64) {
    let d = 10_000_000_000_000_000_000_u64; // 10^19

    let quot = if n < 1 << 83 {
        u128::from((n >> 19) as u64 / (d >> 19))
    } else {
        u128_mulhi(n, 156927543384667019095894735580191660403) >> 62
    };

    let rem = (n - quot * u128::from(d)) as u64;
    debug_assert_eq!(quot, n / u128::from(d));
    debug_assert_eq!(u128::from(rem), n % u128::from(d));

    (quot, rem)
}
