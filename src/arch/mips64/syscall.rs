// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

// MIPS has the following registers:
//
// | Symbolic Name | Number          | Usage                          |
// | ============= | =============== | ============================== |
// | zero          | 0               | Constant 0.                    |
// | at            | 1               | Reserved for the assembler.    |
// | v0 - v1       | 2 - 3           | Result Registers.              |
// | a0 - a3       | 4 - 7           | Argument Registers 1 ·· · 4.   |
// | t0 - t9       | 8 - 15, 24 - 25 | Temporary Registers 0 · · · 9. |
// | s0 - s7       | 16 - 23         | Saved Registers 0 ·· · 7.      |
// | k0 - k1       | 26 - 27         | Kernel Registers 0 ·· · 1.     |
// | gp            | 28              | Global Data Pointer.           |
// | sp            | 29              | Stack Pointer.                 |
// | fp            | 30              | Frame Pointer.                 |
// | ra            | 31              | Return Address.                |
//
// The following registers are used for args 1-6:
//
// arg1: %a0 ($4)
// arg2: %a1 ($5)
// arg3: %a2 ($6)
// arg4: %a3 ($7)
// arg5: %t0 ($8)
// arg6: %t1 ($9)
//
// %v0 is the syscall number.
// %v0 is the return value.
// %a3 is a boolean indicating that an error occurred.
//
// All temporary registers are clobbered (8-15, 24-25).
//
// NOTE: The main difference between MIPS and MIPS64 is that MIPS64 doesn't use
// the stack to pass in args 5-6. Instead, it uses the temporary registers t0
// and t1, which still get clobbered.
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
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n as usize => ret,
        lateout("$7") err,
        in("$4") arg1,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n as usize => ret,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // $7 is now used for both input and output.
        inlateout("$7") arg4 => err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    let mut err: usize;
    let mut ret: usize;
    let _key: usize = const_random!(usize);
    asm!(
        "syscall",
        inlateout("$2") n as usize => ret,
        lateout("$7") err,
        in("$4") arg1,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    let mut err: usize;
    let mut ret: usize;
    let _key: usize = const_random!(usize);
    asm!(
        "syscall",
        inlateout("$2") n as usize => ret,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // $7 is now used for both input and output.
        inlateout("$7") arg4 => err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}
