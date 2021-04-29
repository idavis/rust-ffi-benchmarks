#![feature(test)]
extern crate test;

extern "C" {
    fn add_one(x: i64) -> i64;
    fn get_one() -> i64;
    fn calc_sum(iterations: i64) -> i64;
    fn calc_sum_opt(iterations: i64) -> i64;
}

#[inline(never)]
fn add_one_rust(x: i64) -> i64 {
    x + 1
}

fn add_one_rust_opt(x: i64) -> i64 {
    x + 1
}

static ITERATIONS: i64 = 10_000_000;

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use test::Bencher;

    fn assert_sum(sum: i64) -> () {
        if ITERATIONS != sum {
            panic!("Broken sum code");
        }
    }

    #[bench]
    fn bench_add_one_capi(b: &mut Bencher) {
        let n: i64 = test::black_box(ITERATIONS);
        
        b.iter(|| {
            let sum = (0..n).fold(0, |a, _| unsafe { add_one(a) });
            assert_sum(sum);
        });
    }

    #[bench]
    fn bench_calc_sum_capi_opt(b: &mut Bencher) {
        let n: i64 = test::black_box(ITERATIONS);
        b.iter(|| {
            let sum = unsafe { calc_sum_opt(n) };
            assert_sum(sum);
        });
    }

    #[bench]
    fn bench_calc_sum_capi(b: &mut Bencher) {
        let n: i64 = test::black_box(ITERATIONS);
        b.iter(|| {
            let sum = unsafe { calc_sum(n) };
            assert_sum(sum);
        });
    }

    #[bench]
    fn bench_get_one_capi(b: &mut Bencher) {
        let n: i64 = test::black_box(ITERATIONS);
        b.iter(|| {
            let sum = (0..n).fold(0, |a, _b| unsafe { get_one() } + a);
            assert_sum(sum);
        });
    }

    #[bench]
    fn bench_add_one_rust(b: &mut Bencher) {
        let n: i64 = test::black_box(ITERATIONS);
        b.iter(|| {
            let sum = (0..n).fold(0, |a, _b| add_one_rust(a));
            assert_sum(sum);
        });
    }

    #[bench]
    fn bench_add_one_rust_opt(b: &mut Bencher) {
        let n: i64 = test::black_box(ITERATIONS);
        b.iter(|| {
            let sum = (0..n).fold(0, |a, _b| add_one_rust_opt(a));
            assert_sum(sum);
        });
    }
}
