// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

// On x86, the following registers are used for args 1-6:
// arg1: %ebx
// arg2: %ecx
// arg3: %edx
// arg4: %esi
// arg5: %edi
// arg6: %ebp
//
// eax is used for both the syscall number and the syscall return value.
//
// No other registers are clobbered. syscalls can also modify memory. With the
// `asm!()` macro, it is assumed that memory is clobbered unless the nomem
// option is specified.
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
        "int $$0x80",
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
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
        "xchg esi, {arg4}",
        "int $$0x80",
        "xchg esi, {arg4}",
        // Using esi is not allowed, so we need to use another register to
        // save/restore esi. Thus, we can say that esi is not clobbered.
        arg4 = in(reg) arg4,
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
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
        "int $$0x80",
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
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
        "xchg esi, {arg4}",
        "int $$0x80",
        "xchg esi, {arg4}",
        // Using esi is not allowed, so we need to use another register to
        // save/restore esi. Thus, we can say that esi is not clobbered.
        arg4 = in(reg) arg4,
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        options(nostack, preserves_flags)
    );
    ret
}
