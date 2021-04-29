extern crate cc;

fn main() {
    cc::Build::new().file("src/capi.c").compile("libcapi.a");
}
