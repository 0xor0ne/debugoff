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

// use std::mem::MaybeUninit;
// use std::sync::Mutex;
use std::cell::{RefCell, RefMut};
// use std::sync::Once;
use const_random::const_random;
use crunchy::*;
#[cfg(feature = "obfuscate")]
use goldberg::*;
use unroll::*;

#[allow(non_camel_case_types)]
enum PtraceRequest {
    PTRACE_TRACEME = 0,
}

#[derive(Debug)]
struct PtraceState {
    traceme_done: bool,
    traceme_ctr: u64,
}

const SRSIZE: usize = 10;

#[derive(Debug)]
struct Aa {
    // ptrace_state: Mutex<PtraceState>,
    ptrace_state: PtraceState,
    r: Rand,
    sr: [u32; SRSIZE],
}

thread_local!(static AA: RefCell<Aa> = RefCell::new(Aa::new()));

impl Aa {
    // fn ptrace_singleton() -> &'static Aa {
    //     static mut SINGLETON: MaybeUninit<Aa> = MaybeUninit::uninit();
    //     static ONCE: Once = Once::new();

    //     unsafe {
    //         ONCE.call_once(|| {
    //             let singleton = Aa {
    //                 ptrace_state: Mutex::new(PtraceState {
    //                     traceme_done: false,
    //                     traceme_ctr: 0,
    //                 }),
    //             };
    //             SINGLETON.write(singleton);
    //         });

    //         SINGLETON.assume_init_ref()
    //     }
    // }

    fn new() -> Aa {
        let tmp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH);

        #[cfg(feature = "obfuscate")]
        goldberg_stmts! {
            let r: u128 = match tmp {
                Ok(x) => x.as_nanos(),
                _ => 0u128,
            };
        }
        #[cfg(not(feature = "obfuscate"))]
        let r: u128 = match tmp {
            Ok(x) => x.as_nanos(),
            _ => 0u128,
        };

        #[cfg(feature = "obfuscate")]
        goldberg_stmts! {
            Aa {
                ptrace_state : PtraceState {
                    traceme_done: false,
                    traceme_ctr: 0u64,
                },
                r: Rand::new(r as u32),
                sr: [
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                    const_random!(u32),
                ],
            }
        }
        #[cfg(not(feature = "obfuscate"))]
        Aa {
            ptrace_state: PtraceState {
                traceme_done: false,
                traceme_ctr: 0u64,
            },
            r: Rand::new(r as u32),
            sr: [
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
                const_random!(u32),
            ],
        }
    }
}

/// Sets the process as traceable, as with `ptrace(PTRACE_TRACEME, ...)`
#[inline(always)]
fn ptraceme() -> Result<(), crate::DebugOffErr> {
    #[cfg(feature = "obfuscate")]
    let res: usize = unsafe {
        crate::arch::syscall4(
            crate::arch::SysNo::SYS_PTRACE,
            PtraceRequest::PTRACE_TRACEME as usize,
            goldberg_int! {0usize},
            goldberg_int! {0usize},
            goldberg_int! {0usize},
        )
    };
    #[cfg(not(feature = "obfuscate"))]
    let res: usize = unsafe {
        crate::arch::syscall4(
            crate::arch::SysNo::SYS_PTRACE,
            PtraceRequest::PTRACE_TRACEME as usize,
            0,
            0,
            0,
        )
    };

    match res {
        0 => Ok(()),
        _ => Err(crate::DebugOffErr {
            err_type: crate::DebugOffErrType::AlreadyTraced,
        }),
    }
}

/// Call `ptrace(PTRACE_TRACEME, ...)` to detect the presence of a debugger.
///
/// This function can be called multiple times.
///
/// At the first invocation, the function expects a return value of 0 from `ptrace(PTRACE_TRACEME, ...)`.
/// In subsequent calls, `ptrace(PTRACE_TRACEME, ...)` should return -1.
///
/// If the above is not satisfied, the function calls `exit_group(0)`.
///
/// To be more effective, the function should be called at least once for each thread.
///
/// ## Examples
///
/// ```rust
/// // Import only on Linux and for "release builds"
/// #[cfg(target_os = "linux")]
/// #[cfg(not(debug_assertions))]
/// use debugoff;
///
/// // Call only on Linux and for "release" builds.
/// #[cfg(target_os = "linux")]
/// #[cfg(not(debug_assertions))]
/// debugoff::ptraceme_or_die();
/// ```
#[inline(always)]
pub fn ptraceme_or_die() {
    let res = ptraceme();
    // println!("{:?}", res);

    // The first time this function is called, res should be Ok(_). Subsequent calls should
    // return Err(_)
    #[cfg(feature = "obfuscate")]
    AA.with(|f| {
        let mut aa: RefMut<Aa> = f.borrow_mut();
        match aa.ptrace_state.traceme_done {
            false => match res {
                Ok(_) => aa.ptrace_state.traceme_done = true,
                Err(_) => the_end(),
            },
            true => if res.is_ok() { the_end() },
        }

        goldberg_stmts! {
            let mut tmp: u64 = aa.ptrace_state.traceme_ctr;
            tmp = tmp.saturating_add(1);
            aa.ptrace_state.traceme_ctr = tmp;
        }
    });
    #[cfg(not(feature = "obfuscate"))]
    AA.with(|f| {
        let mut aa: RefMut<Aa> = f.borrow_mut();
        match aa.ptrace_state.traceme_done {
            false => match res {
                Ok(_) => aa.ptrace_state.traceme_done = true,
                Err(_) => the_end(),
            },
            true => if res.is_ok() { the_end() },
        }

        aa.ptrace_state.traceme_ctr = aa.ptrace_state.traceme_ctr.saturating_add(1);
    });
}

/// Call `ptrace(PTRACE_TRACEME, ...)` multiple times in nested loops.
///
/// The loop iterations are unrolled and the number of iterations is randomized for each
/// compilation (just remember to clean you project before compiling again: `cargo clean`).
///
/// For each iteration, if the value returned by `ptrace` is not the expected one, the function
/// calls `exit_group(0)`. If the value returned by `ptrace` is the expected one (0 at the first
/// call in a thread and -1 thereafter) then a random value (sum of a dynamic random value and a
/// compilation time random value) is added to an `offset` value. At the end of all the iterations
/// the `offset` value is checked. If the check fails, the function calls
/// `exit_group(0)`
///
/// If `debugoff` is included as a dependency with `obfuscate` feature enabled, the code is even
/// more obfuscated by [goldberg](https://crates.io/crates/goldberg).
///
/// This function can be called multiple times.
/// To be more effective, the function should be called at least once for each thread.
///
/// ## Examples
///
/// ```rust
/// // Import only on Linux and for "release builds"
/// #[cfg(target_os = "linux")]
/// #[cfg(not(debug_assertions))]
/// use debugoff;
///
/// // Call only on Linux and for "release" builds.
/// #[cfg(target_os = "linux")]
/// #[cfg(not(debug_assertions))]
/// debugoff::multi_ptraceme_or_die();
/// ```
#[unroll_for_loops]
#[inline(always)]
pub fn multi_ptraceme_or_die() {
    unroll! {
        for j in 0..16 {
            let mut v: Vec<u32> = Vec::new();
            let mut offset: u32 = 0;
            for _i in 1..((const_random!(usize) % 4) + 2) {
                let res = ptraceme();
                #[cfg(feature="obfuscate")]
                AA.with (|f| {
                    let mut aa: RefMut<Aa> = f.borrow_mut();
                    goldberg_stmts! {
                        let r = aa.r.rand();
                        match aa.ptrace_state.traceme_done {
                            false => match res {
                                Ok(_) => {
                                    aa.ptrace_state.traceme_done = true;
                                    v.push(r);
                                    for idx in 0..SRSIZE {
                                        offset = offset.wrapping_add(r.wrapping_add(aa.sr[idx % SRSIZE]));
                                    }
                                },
                                Err(_) => the_end(),
                            },
                            true => match res {
                                Ok(_) => the_end(),
                                _ => {
                                    v.push(r);
                                    for idx in 0..SRSIZE {
                                        offset = offset.wrapping_add(r.wrapping_add(aa.sr[idx % SRSIZE]));
                                    }
                                }
                            },
                        }
                        aa.ptrace_state.traceme_ctr = aa.ptrace_state.traceme_ctr.saturating_add(1);
                    }
                });
                #[cfg(not(feature="obfuscate"))]
                AA.with (|f| {
                    let mut aa: RefMut<Aa> = f.borrow_mut();
                    let r = aa.r.rand();
                    match aa.ptrace_state.traceme_done {
                        false => match res {
                            Ok(_) => {
                                aa.ptrace_state.traceme_done = true;
                                v.push(r);
                                for idx in 0..SRSIZE {
                                    offset = offset.wrapping_add(r.wrapping_add(aa.sr[idx % SRSIZE]));
                                }
                            },
                            Err(_) => the_end(),
                        },
                        true => match res {
                            Ok(_) => the_end(),
                            _ => {
                                v.push(r);
                                for idx in 0..SRSIZE {
                                    offset = offset.wrapping_add(r.wrapping_add(aa.sr[idx % SRSIZE]));
                                }
                            }
                        },
                    }
                    aa.ptrace_state.traceme_ctr = aa.ptrace_state.traceme_ctr.saturating_add(1);
                });
            }

            let mut check: u32 = 0;
            AA.with (|f| {
                let aa: RefMut<Aa> = f.borrow_mut();
                for r in v {
                    for idx in 0..SRSIZE {
                        check = check.wrapping_add(r.wrapping_add(aa.sr[idx % SRSIZE]));
                    }
                }
            });

            if check != offset {
                the_end();
            }
        }
    }
}

#[cfg(feature = "obfuscate")]
#[inline(always)]
fn the_end() {
    // Be carefull, optimizer in release mode can decide to remove the following code.
    // let p: *mut u32 = core::ptr::null_mut();
    // unsafe {
    //     *p = 0xFF;
    // }

    let _res =
        unsafe { crate::arch::syscall1(crate::arch::SysNo::SYS_EXIT_GROUP, goldberg_int!(0usize)) };
}
#[cfg(not(feature = "obfuscate"))]
#[inline(always)]
fn the_end() {
    // Be carefull, optimizer in release mode can decide to remove the following code.
    // let p: *mut u32 = core::ptr::null_mut();
    // unsafe {
    //     *p = 0xFF;
    // }

    let _res = unsafe { crate::arch::syscall1(crate::arch::SysNo::SYS_EXIT_GROUP, 0) };
}

#[derive(Debug)]
pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    #[cfg(feature = "obfuscate")]
    fn new(seed: u32) -> Rand {
        goldberg_stmts! {
            let kx: u32 = 123456789u32;
            let ky: u32 = 362436069u32;
            let kz: u32 = 521288629u32;
            let kw: u32 = 88675123u32;
            Rand{
                x: kx^seed, y: ky^seed,
                z: kz, w: kw
            }
        }
    }
    #[cfg(not(feature = "obfuscate"))]
    fn new(seed: u32) -> Rand {
        let kx: u32 = 123456789u32;
        let ky: u32 = 362436069u32;
        let kz: u32 = 521288629u32;
        let kw: u32 = 88675123u32;
        Rand {
            x: kx ^ seed,
            y: ky ^ seed,
            z: kz,
            w: kw,
        }
    }

    #[cfg(feature = "obfuscate")]
    fn rand(&mut self) -> u32 {
        goldberg_stmts! {
            let t = self.x^self.x.wrapping_shl(11);
            self.x = self.y; self.y = self.z; self.z = self.w;
            self.w ^= self.w.wrapping_shr(19)^t^t.wrapping_shr(8);
            return self.w;
        }
    }
    #[cfg(not(feature = "obfuscate"))]
    fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        self.w
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod test {

    use std::thread;
    use std::time::Duration;

    #[test]
    fn multiple_ptraceme_or_die() {
        for i in 0..10 {
            super::Aa::ptraceme_or_die();
            println!("{}", i);
        }

        super::AA.with(|f| {
            assert_eq!(10, f.borrow().ptrace_state.traceme_ctr);
        });
    }

    #[test]
    fn multiple_threads_ptraceme_or_die() {
        let tmp = super::AA.with(|f| f.borrow().ptrace_state.traceme_ctr);

        let threads: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    super::Aa::ptraceme_or_die();
                    thread::sleep(Duration::from_millis(i * 10));
                    println!("Thread #{}", i);
                    super::Aa::ptraceme_or_die();
                    super::AA.with(|f| {
                        assert_eq!(2, f.borrow().ptrace_state.traceme_ctr);
                    });
                })
            })
            .collect();

        for thread in threads.into_iter() {
            thread.join().unwrap();
        }

        super::AA.with(|f| {
            assert_eq!(tmp, f.borrow().ptrace_state.traceme_ctr);
        });

        println!("END");
        std::process::exit(0);
    }
}
