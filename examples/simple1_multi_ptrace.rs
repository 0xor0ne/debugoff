// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

#[cfg(target_os = "linux")]
#[cfg(not(debug_assertions))]
use debugoff;
use std::time::SystemTime;

pub fn main() {
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
