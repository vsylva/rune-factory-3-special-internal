use crate::GLOBAL_HOOK;

pub(crate) unsafe extern "C" fn 物品编辑() {
    ::core::arch::asm!(
        "
        push rax
        push r10
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        mov [r10], rax

        push rcx
        push rdx

        mov rdx,0
        mov rcx,0
        mov cl,byte ptr [rax+3]
        shr cl, 4
        or dx,cx

        mov rcx,0
        mov cl,byte ptr [rax+7]
        shr cl, 6
        shl cl, 4
        or dx,cx

        mov rcx,0
        mov cl,byte ptr [rax+0xB]
        shr cl, 6
        shl cl, 6
        or dx,cx

        mov rcx,0
        mov cl,byte ptr [rax+0xF]
        shr cx, 6
        shl cx, 8
        or dx,cx

        pop rdx
        pop rcx

        mov ecx,[rax]
        test ecx,0x00003C00
        ",
        in("r10") &raw const GLOBAL_HOOK.物品指针,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        pop r10
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
