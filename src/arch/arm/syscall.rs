// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

// On arm, the following registers are used for args 1-6:
// arg1: %r0
// arg2: %r1
// arg3: %r2
// arg4: %r3
// arg5: %r4
// arg6: %r5
//
// %r7 is used for the syscall number.
//
// %r0 is reused for the syscall return value.
//
// No other registers are clobbered.
use super::syscalls::SysNo;
#[cfg(feature = "syscallobf")]
use const_random::const_random;
use core::arch::asm;

/// Issues a raw system call with 1 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[cfg(not(feature = "syscallobf"))]
#[inline(always)]
pub unsafe fn syscall1(n: SysNo, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[cfg(not(feature = "syscallobf"))]
#[inline(always)]
pub unsafe fn syscall4(n: SysNo, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        in("r3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw obfuscated system call with 1 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[cfg(feature = "syscallobf")]
#[inline(always)]
pub unsafe fn syscall1(n: SysNo, arg1: usize) -> usize {
    let mut ret: usize;
    let _key: usize = const_random!(usize);
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw obfuscated system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[cfg(feature = "syscallobf")]
#[inline(always)]
pub unsafe fn syscall4(n: SysNo, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let mut ret: usize;
    let _key: usize = const_random!(usize);
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        in("r3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}
