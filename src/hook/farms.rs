use crate::GLOBAL_HOOK;

pub(crate) unsafe extern "C" fn 农田() {
    ::core::arch::asm!(
        "
        push rax
        push r11
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        cmp r11,0
        je 2f

        mov rax, 0x0
        mov eax, dword ptr [rbx + 0x4]
        or eax, 0x38003E
        xor eax, 0x38000E
        mov [rbx + 0x4], eax

        2:
        ",
        in("r11") GLOBAL_HOOK.土壤质量标签,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        cmp r12, 0x0
        je 3f

        mov rax, 0x0
        mov al, byte ptr[rbx + 0x3]
        or al, 0x10
        and al, 0xFB
        mov byte ptr [rbx + 0x3], al

        3:
        ",
        in("r12") GLOBAL_HOOK.自动浇水标签 ,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        cmp r13, 0x0
        je 4f

        mov rax, 0x0
        mov al, byte ptr [rbx + 0x3]
        and al, 0x8
        cmp al, 0x8
        je 4f

        mov al, byte ptr[rbx + 0x3]
        or al, 0x8
        mov byte ptr [rbx + 0x3], al
        mov byte ptr [rbx], 0x0

        4:
        ",
        in("r13") GLOBAL_HOOK.自动耕作标签,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        cmp r14, 0x0
        je 5f

        mov rax, 0x0
        mov al, byte ptr [rbx + 0x3]
        and al, 0x8
        cmp al, 0x8
       ",


        in("r14") GLOBAL_HOOK.自动种植标签,

        options(nostack)
    );

    ::core::arch::asm!(
        "
        jne 5f

        mov rax, 0x0
        mov ax, word ptr [r15]
        mov word ptr [rbx + 0x0], ax

        5:
        ",
        in("r15") &raw const GLOBAL_HOOK.作物属性,
        options(nostack)
    );

    ::core::arch::asm!(
        "
        pop r11
        pop rax
        pop r12
        pop r13
        pop r14
        pop r15
        ",
        options(nostack)
    );

    ::core::arch::asm!(
        "
        add rbx, 0x8
        cmp di, r15w
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
pub(crate) unsafe extern "C" fn 作物立即成熟() {
    ::core::arch::naked_asm!(
        "
        mov edx, [rax]
        or edx, 0x5000
        and edx, 0xFFFFDFFF
        mov [rax], edx
        shr edx, 0x1
        and edx, 0x7F

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
