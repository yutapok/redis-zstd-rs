extern crate gcc;

fn main() {
    // Build a Redis pseudo-library so that we have symbols that we can link
    // against while building Rust code.
    //
    // include/redismodule.h is just vendored in from the Redis project and
    // src/redismodule.c is just a stub that includes it and plays a few other
    // tricks that we need to complete the build.
    gcc::Build::new()
        .file("src/redismodule.c")
        .include("include/")
        .compile("libredismodule.a");

    gcc::Build::new()
        .file("src/rmod_custom.c")
        .include("include/")
        .flag("-lzstd")
        .compile("librmod_custom.a");

    // The GCC module emits `rustc-link-lib=static=redismodule` for us.
}
