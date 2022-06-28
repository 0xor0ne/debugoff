// BSD 2-Clause License
//
// Copyright (c) 2022, 0xor0ne
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Rust anti-analysis library for making static and dynamic (debugging) analysis more difficult.
//!
//! The library targets Linux environments.
//!
//! It is currently based on `ptrace` anti-analysis trick and has the following features:
//!
//! * Direct syscall invocation without relying on libc (this makes LD_PRELOAD bypass mechanism
//! ineffective);
//! * Multiple `ptrace` syscall invocations. Each call to `ptrace` must return the expected value
//! and contributes to the computation of an "`offset`" value that, at the end of the call chain,
//! must match an expected value (see [here](https://seblau.github.io/posts/linux-anti-debugging));
//! * 'ptrace' is called in nested loops. The loops are unrolled and the number of iterations is
//! randomized at each compilation. Also the `"offset`" value is radomized at each iteration;
//! * The produced code can be obfuscated even more by enabling the `obfuscate` feature which
//! relies on [goldberg crate](https://crates.io/crates/goldberg);
//!
//!
//! Overall, this is not the final solution against static and dynamic analysis but for sure it
//! is going to make the reverser/analyst life a little bit more difficult.
//!
//! to use the crate, add it to your dependencies:
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
//! cargo clean
//! cargo build --release
//! ```
//!
//! Also, it would be a good idea to build the project without symbols:
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
//! println!(
//!     "Time: {}",
//!     SystemTime::now()
//!         .duration_since(SystemTime::UNIX_EPOCH)
//!         .unwrap()
//!         .as_millis()
//! );
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
