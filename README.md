# Size optimization of Rust binaries
This repository shows some examples of binary size optimization in Rust as part of our Newsletter see
[Rust-Trends.com](https://Rust-Trends.com)

Rust optimizes for speed of execution and compilation instead of binary size. But in some cases 
optimization for size is preferred, e.g. when you have limited space (Embedded Systems) or when hackers
want to stay unnoticed on a system, i.e. larger binaries are more suspicious and easier to detect.

# Build with custom profiles
![Minimum Rust: 1.57](https://img.shields.io/badge/Minimum%20Rust%20Version-1.57-brightgreen.svg)

When executing `cargo build` the Rust binary is built in Debug mode. Debug mode comes with overhead
to provide extra information to the IDE (debuggers). This overhead is stripped when you build with
the release flag. `cargo build --release`.

To minimize binary size, we build with custom profiles that inherit from this release mode. Note 
that custom profiles are stable since Rust 1.57.

Ran below on a MacBook Pro (13-inch, 2018) Intel
```bash
$ cargo build --release
    Compiling size-optimization-rust-binaries v0.1.0
    Finished release [optimized] target(s) in 26.27s

$ cargo build --profile=size-opt1
    Compiling size-optimization-rust-binaries v0.1.0 
    Finished size-opt1 [optimized] target(s) in 22.77s

$ cargo build --profile=size-opt2
    Compiling size-optimization-rust-binaries v0.1.0
    Finished size-opt2 [optimized] target(s) in 25.31s

$ cargo build --profile=size-opt3
    Compiling size-optimization-rust-binaries v0.1.0
    Finished size-opt3 [optimized] target(s) in 25.71s

$ du -h target/release/size-optimization-rust-binaries
1.2M    target/release/size-optimization-rust-binaries

$ du -h target/size-opt1 size-optimization-rust-binaries
804K    target/size-opt1/size-optimization-rust-binaries

$ du -h target/size-opt2 size-optimization-rust-binaries
668K    target/size-opt2/size-optimization-rust-binaries

$ du -h target/size-opt3 size-optimization-rust-binaries
652K    target/size-opt3/size-optimization-rust-binaries
```

Note the reduction in size and the difference in compile time.

# The custom profiles explained
There are three custom profiles, you can also find them in the `Cargo.toml` file.

## size-opt1
```toml
[profile.size-opt1]
inherits = "release"
strip = true # strip symbols from the binary
opt-level = 'z' # Optimize for size
```
`strip` strips the symbols from the binary. With symbols, also called function symbols, a debugger can show what part of code belongs to what function. Easy when you are debugging but not necessary in a release. [The Cargo Book](https://doc.rust-lang.org/cargo/reference/profiles.html#strip) 

The optimization level `opt-level` is default set to `3` in release mode and includes all speed optimizations. Changing it to `z` sets it to optimize for size instead. For more information see opt-level in [The Cargo Book](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level)

## size-opt2
```toml
[profile.size-opt2]
inherits = "release"
strip = true # strip symbols from the binary
opt-level = 'z' # Optimize for size
lto = true # Link Time Optimization (LTO)
```
Link Time Optimization will instruct the compiler toolchain before linking the final binary, to look one last time for possible optimizations.
[The rustc Book](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto)

## size-opt3
```toml
[profile.size-opt3]
inherits = "release"
strip = true # strip symbols from the binary
opt-level = 'z' # Optimize for size
lto = true # Link Time Optimization (LTO)
codegen-units = 1 # Parallel Code Generation Units
```
Code generation units greater than 1 specify that the compiler toolchain is allowed to process our
code in parallel. This most of the time leaves us with unused optimization potential, in this
case for size, since it will look at our code in parallel. By default when not specified it is set
to 16, which increases compilation speed.

# Links
- For more info look at the [min-sized-rust - GitHub repository](https://github.com/johnthagen/min-sized-rust). Do not forget to look at the links at the bottom of their `README.md` file for more reading material.
- Blog post about [Optimizing Rust Binary Size](https://arusahni.net/blog/2020/03/optimizing-rust-binary-size.html)

- Most programs are built on top of other crates, if you want to analyze them use `cargo-bloat`. You can install it with `$ cargo install cargo-bloat` and run it with `cargo bloat` in your project folder.

<br />
<br />

**Join our Newsletter at [Rust-Trends.com](https://Rust-Trends.com)**, for your biweekly dose of Rust Trends.