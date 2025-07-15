use crate::GLOBAL_HOOK;

pub(crate) unsafe extern "C" fn 时间() {
    ::core::arch::asm!(
        "
        push rax
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        lea rax, [r9 + 0x4]
        mov [r15], rax
        ",
        in("r15") &raw const GLOBAL_HOOK.时间指针,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        pop rax
        pop r15
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        add edx, eax
        add [r9 + 0x4], edx
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nostack)
    );
}
