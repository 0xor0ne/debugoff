# DebugOff Library

> WORK IN PROGRESS

Rust anti-analysis library for making static and dynamic (debugging) analysis
more difficult.

The library targets Linux environments.

It is currently based on `ptrace` anti-analysis trick and has the following
features:

* Direct syscall invocation without relying on libc (this makes LD_PRELOAD
  bypass mechanism ineffective);

* Multiple `ptrace` syscall invocations. Each call to `ptrace` must return the
  expected value and contributes to the computation of an "`offset`" value that,
  at the end of the call chain, must match an expected value (see
  [here](https://seblau.github.io/posts/linux-anti-debugging));

* `ptrace` is called in nested loops. The loops are unrolled and the number of
  iterations is randomized at each compilation. Also the `"offset`" value is
  radomized at each iteration;

* The produced code can be obfuscated even more by enabling the `obfuscate`
  feature which relies on [goldberg crate](https://crates.io/crates/goldberg);

Overall, this is not the final solution against static and dynamic analysis
but for sure it is going to make the reverser/analyst life a little bit more
difficult.

to use the crate, add it to your dependencies:

```text
[dependencies]
debugoff = { version = "0.1.0, features = ["obfuscate"] }
```

Given that the library generates random code at each compilation, be sure to
rebuild everything each time. Something like this:

```text
cargo clean
cargo build --release
```

Also, it would be a good idea to build the project without symbols:

```text
[profile.release]
debug = false
strip = "symbols"
panic = "abort"
```

## Usage Example

```rust
// Include only for Linux and when building in release mode
#[cfg(target_os = "linux")]
#[cfg(not(debug_assertions))]
use debugoff;
use std::time::SystemTime;

fn main() {
  // Call only for Linux and when building in release mode
  #[cfg(target_os = "linux")]
  #[cfg(not(debug_assertions))]
  debugoff::multi_ptraceme_or_die();

  println!(
      "Time: {}",
      SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .unwrap()
          .as_millis()
  );

  // Call only for Linux and when building in release mode
  #[cfg(target_os = "linux")]
  #[cfg(not(debug_assertions))]
  debugoff::multi_ptraceme_or_die();

  println!("Example complete!");
}
```

See other examples in [examples directory](./examples).

## Obfuscation example

If we build the following code (which does not use `DebugOff`) in release mode:

```rust
use std::time::SystemTime;

fn main() {
  println!(
      "Time: {}",
      SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .unwrap()
          .as_millis()
  );

  println!("Example complete!");
}
```

This is the corresponding `main` function graph:

![Executable build without
DebugOff](./docs/images/function_graph_no_debugoff.png).

If we build the same code but using `DebugOff` this time:

```rust
#[cfg(target_os = "linux")]
#[cfg(not(debug_assertions))]
use debugoff;
use std::time::SystemTime;

fn main() {
  #[cfg(target_os = "linux")]
  #[cfg(not(debug_assertions))]
  debugoff::multi_ptraceme_or_die();

  println!(
      "Time: {}",
      SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .unwrap()
          .as_millis()
  );

  #[cfg(target_os = "linux")]
  #[cfg(not(debug_assertions))]
  debugoff::multi_ptraceme_or_die();

  println!("Example complete!");
}
```

This is the obfuscated `main` function graph obtained:

![Executable build with
DebugOff](./docs/images/function_graph_obfuscate.png).

In this particular example, all the code produced by `DebugOff` was inlided in
the `main` function. this is not always the case, it can depends on many factors
like the points where `DebugOff` is called and the toolchain version. In other
cases the resulting function graph could be simpler than the one reported in the
example but `DebugOff` always offer some degree of obfuscation.

## TODOs

* Syscall obfuscation;
