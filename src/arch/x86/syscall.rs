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
use core::arch::asm;

/// Issues a raw system call with 1 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
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
