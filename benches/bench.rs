#![feature(test)]
#![allow(non_snake_case)]
#![allow(clippy::cast_lossless)]

extern crate test;

macro_rules! benches {
    ($($name:ident($value:expr))*) => {
        mod bench_itoa_format {
            use std::hint;
            use test::Bencher;

            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    let mut buffer = itoa::Buffer::new();

                    b.iter(|| {
                        let printed = buffer.format(hint::black_box($value));
                        hint::black_box(printed);
                    });
                }
            )*
        }

        mod bench_std_fmt {
            use std::hint;
            use std::io::Write;
            use test::Bencher;

            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    let mut buf = Vec::with_capacity(40);

                    b.iter(|| {
                        buf.clear();
                        write!(&mut buf, "{}", hint::black_box($value)).unwrap();
                        hint::black_box(&buf);
                    });
                }
            )*
        }
    }
}

benches! {
    bench_u64_0(0u64)
    bench_u64_half(u32::MAX as u64)
    bench_u64_max(u64::MAX)

    bench_i16_0(0i16)
    bench_i16_min(i16::MIN)

    bench_u128_0(0u128)
    bench_u128_max(u128::MAX)
}
