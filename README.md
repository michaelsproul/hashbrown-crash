# Steps to reproduce

1. Build `libdiffuzz.so` following instructions here: https://github.com/Shnatsel/libdiffuzz#tldr-usage
2. Build this crate with: `env RUSTFLAGS="-g" cargo build --release`
3. Run with `env LD_PRELOAD=/your/libdiffuzz.so target/release/hashbrown-crash`
4. The binary segfaults 💥

Or with GDB:

```
$ gdb target/release/hashbrown-crash
(gdb) set environment LD_PRELOAD /your/libdiffuzz.so
(gdb) start
(gdb) bt
```

See [./backtrace.txt](https://github.com/michaelsproul/hashbrown-crash/blob/main/backtrace.txt) for the crash.
