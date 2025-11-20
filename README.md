Turns out this is a dup of rust-lang/rust issues #129904.

<details>
<summary>example</summary>

This works:

```bash
cargo build --bin mwe
```

but this doesn't:

```bash
mv src/lib_muted.rs src/lib.rs
cargo build --lib  # ok
cargo build --bin mwe  # failed: link error
```

```
ld: Undefined symbols:
            _cxxbridge1$print_int, referenced from:
                mwe::cpp::ffi::print_int::h77d72e315e7e8095 in mwe-166a17518e1a13b5.4cbwrp1o0lewj4p3jxqrj1fnk.0ik3nh9.rcgu.o
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

With `-v` specified, we can see that no `-l` flag is passed to the linker, causing the failure of symbol resolution. Adding `println!("cargo:rustc-link-lib=static=cpp");` doesn't change this behavior.

```
~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc --crate-name mwe --edition=2024 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=172 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C split-debuginfo=unpacked --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=ec4a1eb35136f0a2 -C extra-filename=-6b0dcd384c59add2 --out-dir ~/Code/rust/mwe/target/debug/deps -C incremental=~/Code/rust/mwe/target/debug/incremental -L dependency=~/Code/rust/mwe/target/debug/deps --extern cxx=~/Code/rust/mwe/target/debug/deps/libcxx-5731d5c0f01a0c2b.rlib --extern mwe=~/Code/rust/mwe/target/debug/deps/libmwe-4e1250ced3c7189f.rlib -L/opt/homebrew/lib/ -L/opt/homebrew/lib/ -L native=~/Code/rust/mwe/target/debug/build/mwe-852e505a1646d4ef/out -L native=~/Code/rust/mwe/target/debug/build/cxx-20452fd8b085ab95/out -L native=~/Code/rust/mwe/target/debug/build/link-cplusplus-6ed42d3163ab44af/out
```

However, for `cargo build --lib -v`, we can see that the `-l static=cpp` flag is passed to the linker.

Other information:

```
$ rustc --version
rustc 1.91.1 (ed61e7d7e 2025-11-07)
$ cargo --version
cargo 1.91.1 (ea2d97820 2025-10-10)
```

</details>
