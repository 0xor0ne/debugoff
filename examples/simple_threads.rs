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
