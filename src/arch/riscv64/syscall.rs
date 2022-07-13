// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

// On riscv64, the following registers are used for args 1-4:
// arg1: %a0
// arg2: %a1
// arg3: %a2
// arg4: %a3
//
// %a7 is used for the syscall number.
//
// %a0 is reused for the syscall return value.
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
        "ecall 0",
        in("a7") n as usize,
        inlateout("a0") arg1 => ret,
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
        "ecall 0",
        in("a7") n as usize,
        inlateout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a3") arg4,
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
        "ecall 0",
        in("a7") n as usize,
        inlateout("a0") arg1 => ret,
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
        "ecall 0",
        in("a7") n as usize,
        inlateout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}
