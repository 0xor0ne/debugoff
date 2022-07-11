// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

//! ## Linux anti-analysis Rust library
//!
//! The goal of this library is to make both static and dynamic (debugging) analysis more
//! difficult.
//!
//! **The library targets Linux environments.**
//!
//! It is currently based on `ptrace` anti-analysis trick and provides the following main features:
//!
//! * Direct syscall invocation without relying on libc (this makes LD_PRELOAD bypass mechanism
//! ineffective);
//!
//! * Multiple `ptrace` syscall invocations. Each call to `ptrace` must return the expected value
//! (i.e., 0 at the first invocation and -1 thereafter) and contributes to the computation of an
//! "`offset`" value that, at the end of the `ptrace` call chain, must match an expected value (see
//! [here](https://seblau.github.io/posts/linux-anti-debugging)). If ptrace returns an unexpcted
//! value or the "`offset`" value does not match, the process is terminated;
//!
//! * 'ptrace' is called in nested loops. The loops are unrolled and the number of iterations is
//! randomized at each compilation. Moreover, also the "`offset`" value is radomized at each
//! iteration;
//!
//! * The generated code can be obfuscated even more by enabling the `obfuscate` feature which
//! relies on [goldberg crate](https://crates.io/crates/goldberg);
//!
//!
//! To use the crate, add it to your dependencies:
//!
//! ```text
//! [dependencies]
//! debugoff = { version = "0.1.0, features = ["obfuscate"] }
//! ```
//!
//! Given that the library generates random code at each compilation, be sure to rebuild everything
//! each time. Something like this:
//!
//! ```text
//! cargo clean && cargo build --release
//! ```
//!
//! Stripping symbols from the release build is also a good idea:
//!
//! ```text
//! [profile.release]
//! debug = false
//! strip = "symbols"
//! panic = "abort"
//! ```
//!
//! ## Usage Example
//!
//! In the example below, `debugoff` is used only when the target OS is Linux  and only for release
//! builds (in this way when the code is compiled in debug mode it can be debugged without the need
//! to bypass `debugoff`).
//!
//! ```rust
//! // Include only for Linux and when building in release mode
//! #[cfg(target_os = "linux")]
//! #[cfg(not(debug_assertions))] 
//! use debugoff;
//! use std::time::SystemTime;
//!
//!
//! // Call only for Linux and when building in release mode
//! #[cfg(target_os = "linux")]
//! #[cfg(not(debug_assertions))]
//! debugoff::multi_ptraceme_or_die();
//!
//! println!( "Time: {}", SystemTime::now() .duration_since(SystemTime::UNIX_EPOCH)
//!     .unwrap().as_millis());
//!
//! // Call only for Linux and when building in release mode
//! #[cfg(target_os = "linux")]
//! #[cfg(not(debug_assertions))]
//! debugoff::multi_ptraceme_or_die();
//!
//! println!("Example complete!");
//! ```
//!

use std::{error::Error, fmt};

mod aa;
mod arch;

pub use crate::aa::multi_ptraceme_or_die;
pub use crate::aa::ptraceme_or_die;

#[derive(Debug, Clone)]
enum DebugOffErrType {
    // InternalError,
    AlreadyTraced,
    // ValError,
}

#[derive(Debug, Clone)]
struct DebugOffErr {
    err_type: DebugOffErrType,
}

impl Error for DebugOffErr {}

impl fmt::Display for DebugOffErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self.err_type)
    }
}
