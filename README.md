
# C FFI Benchmarks

## src/capi.c
The target C API being consumed by Rust. The methods have inlining 
disabled to try to mimic the extern calls coming from rust.

## src/lib.rs
The Rust code defining the consumed extern functions along with the benchmarck suite.

## ./build.rs
Automates the linking and compilation of the `src/capi.c` code. The debug/release compilation
options are automatically configured by Rust.


## Running benchmarks

```bash
cargo test --release -- -Zunstable-options --format json --report-time --test-threads 1
```

or

```bash
cargo bench
```

### Sample Output for bench 100MM Calls/Iteration
The `*_opt` tests are letting the compiler do its work withing each language to do the all of the work
instead of crossing language boundaries.

```json
running 6 tests
test tests::bench_add_one_capi      ... bench: 138,008,320 ns/iter (+/- 7,195,146)
test tests::bench_add_one_rust      ... bench: 137,977,280 ns/iter (+/- 5,712,915)
test tests::bench_add_one_rust_opt  ... bench:           0 ns/iter (+/- 0)
test tests::bench_calc_sum_capi     ... bench: 138,190,020 ns/iter (+/- 7,506,302)
test tests::bench_calc_sum_capi_opt ... bench:           1 ns/iter (+/- 0)
test tests::bench_get_one_capi      ... bench: 137,848,420 ns/iter (+/- 5,755,599)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out; finished in 169.69s
```

# Setup on my machine

- gcc version 7.5.0
- rustc version 1.53.0-nightly (42816d61e 2021-04-24)

Running benchmarks in release mode compiles with:

```bash
"cc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-Wall" "-Wextra"
```

Running benchmarks in debug mode compiles with:

```bash
"cc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-Wall" "-Wextra"
```