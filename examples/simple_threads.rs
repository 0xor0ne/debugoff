// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

#[cfg(target_os = "linux")]
#[cfg(not(debug_assertions))]
use debugoff;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

pub fn main() {
    #[cfg(target_os = "linux")]
    #[cfg(not(debug_assertions))]
    debugoff::ptraceme_or_die();

    let threads: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                #[cfg(target_os = "linux")]
                #[cfg(not(debug_assertions))]
                debugoff::ptraceme_or_die();

                thread::sleep(Duration::from_millis(i * 10));

                #[cfg(target_os = "linux")]
                #[cfg(not(debug_assertions))]
                debugoff::ptraceme_or_die();

                println!(
                    "Time in thread {}: {}",
                    i,
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                );
            })
        })
        .collect();

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }

    println!(
        "Time in main thread: {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
}
