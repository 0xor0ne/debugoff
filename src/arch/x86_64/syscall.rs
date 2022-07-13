// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

// On x86-64, the following registers are used for args 1-6:
// arg1: %rdi
// arg2: %rsi
// arg3: %rdx
// arg4: %r10
// arg5: %r8
// arg6: %r9
//
// rax is used for both the syscall number and the syscall return value.
//
// rcx and r11 are always clobbered. syscalls can also modify memory. With the
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
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") arg1,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
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
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
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
    let key: usize = const_random!(usize);
    asm!(
        "xor r11, rcx",
        "mov rax, r11",
        "and rcx, 0xFF",
        "add rax, rcx",
        "2:",
        "sub rax, 1",
        "sub rcx, 1",
        "cmp rcx, 0",
        "jg 2b",
        "syscall",
        inout("rcx") ((key as u16) as usize) => _, // rcx is used to store old rip
        inout("r11") ((key as u16) as usize) ^ (n as usize) => _, // r11 is used to store old rflags
        out("rax") ret,
        in("rdi") arg1,
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
    let key: usize = const_random!(usize);
    asm!(
        "xor r11, rcx",
        "mov rax, r11",
        "and rcx, 0xFF",
        "add rax, rcx",
        "2:",
        "sub rax, 1",
        "sub rcx, 1",
        "cmp rcx, 0",
        "jg 2b",
        "syscall",
        inout("rcx") ((key as u16) as usize) => _, // rcx is used to store old rip
        inout("r11") ((key as u16) as usize) ^ (n as usize) => _, // r11 is used to store old rflags
        out("rax") ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        options(nostack, preserves_flags)
    );
    // asm!(
    //     "xor rax, {0}",
    //     "syscall",
    //     in(reg) ((key as u8) as usize),
    //     inlateout("rax") (n as usize) ^ ((key as u8) as usize) => ret,
    //     in("rdi") arg1,
    //     in("rsi") arg2,
    //     in("rdx") arg3,
    //     in("r10") arg4,
    //     out("rcx") _, // rcx is used to store old rip
    //     out("r11") _, // r11 is used to store old rflags
    //     options(nostack, preserves_flags)
    // );
    ret
}
