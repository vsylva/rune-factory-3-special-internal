use crate::GLOBAL_HOOK;

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 居民友谊倍率x100() {
    ::core::arch::naked_asm!(
        "
        push rax

        mov eax, 0x64
        mov r9d, edx
        imul r9d, eax
        test r11, r11

        pop rax

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 技能经验倍率x100() {
    ::core::arch::naked_asm!(
        "
        push rax

        mov rax, 0x64
        imul rdx, rax
        movsxd  r8, edx
        movzx ecx, si

        pop rax

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 战斗经验倍率x100() {
    ::core::arch::naked_asm!(
        "
        push rax

        mov eax, 0x64
        and ecx, r11d
        imul r8d, eax
        add ecx, r8d

        pop rax

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 战斗伤害倍率x100() {
    ::core::arch::naked_asm!(
        "
        push r10

        mov r10d, 0x64
        imul eax, r10d
        mov esi, eax
        mov [rsp + 0x48], eax

        pop r10

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}

pub(crate) unsafe extern "C" fn 自动钓鱼() {
    ::core::arch::asm!(
        "
        movzx ecx, word ptr [rax + 0x18]
        cmp cx, 0x5
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        push rax
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        je 2f
        mov word ptr [rax], 0x1
        ",
        in("rax") &raw const GLOBAL_HOOK.自动钓鱼按键标签,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        jmp 3f
        2:
        mov word ptr [rax], 0x0
        ",
        in("rax") &raw const GLOBAL_HOOK.自动钓鱼按键标签,
        options(nostack));

    ::core::arch::asm!(
        "
        3:
        cmp cx, 0x3
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe extern "C" fn 自动钓鱼按键() {
    ::core::arch::asm!(
        "
        not dx
        and dx, ax
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        push rax
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        cmp rax, 0x1
        ",
        in("rax") GLOBAL_HOOK.自动钓鱼按键标签,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        jne 2f
        mov dx, 0x2

        2:
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe extern "C" fn 角色穿墙() {
    ::core::arch::asm!(
        "
        mov rsi, rdx
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        push rax
        push rcx
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        mov rax, [rax]
        mov rcx, [rax + 0x311F78]
        lea rax, [rcx]
        cmp rbx, rax
        ",
        in("rax") &raw const crate::GLOBAL_SANDLL_INFO.base,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        pop rcx
        pop rax
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        je 2f
        test rcx, rcx
        2:
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

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 禁止负面状态() {
    ::core::arch::naked_asm!(
        "
        push rax

        mov rax, 0x0
        mov ax, [rbx + 0x55]
        and ax, 0xFC0F
        mov [rbx + 0x55],ax

        pop rax

        mov ebp, 0x1000

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 无限接取委托() {
    ::core::arch::naked_asm!(
        "
        mov rbx, [rdx + 0x8]
        lea ecx, [r9 - 0x1]

        push rax

        mov rax, 0x0
        mov eax, [rbx + 0x4]
        or eax, 0x7000000
        xor eax, 0x7000000
        mov [rbx + 0x4], eax

        pop rax

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn 魔物必定驯服() {
    ::core::arch::naked_asm!(
        "
        shr rcx, 0x20
        and ecx, 0x7F
        mov r8w, cx

        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        "
    );
}
