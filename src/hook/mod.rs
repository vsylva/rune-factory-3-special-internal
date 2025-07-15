pub(crate) mod farms;
pub(crate) mod features;
pub(crate) mod items;
pub(crate) mod time;

pub(crate) struct AsmHook {
    pub(crate) 目标地址: usize,
    pub(crate) 开关: bool,
}

impl AsmHook {
    pub(crate) unsafe fn 创建(
        模块地址: usize,
        模块大小: usize,
        特征码: &str,
        特征码地址偏移: usize,
        原指令占用字节: usize,
        跳转地址: *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<Self> {
        let mut hook = Self {
            目标地址: 0,
            开关: false,
        };

        hook.目标地址 = libmem::sig_scan(特征码, 模块地址, 模块大小)? + 特征码地址偏移;

        let 原指令的下一指令地址 = hook.目标地址 + 原指令占用字节;

        let mut 扫描结束的偏移 = 0;

        for i in 0..0xFF {
            let ptr = 跳转地址.cast::<u8>().byte_add(i);

            if ptr.read() == 0x90 {
                let parts = ::core::slice::from_raw_parts(ptr, 4);

                if parts.iter().all(|nop| *nop == 0x90) {
                    扫描结束的偏移 = i;
                    break;
                }
            }
        }

        let mut 远跳转指令 = Vec::new();

        远跳转指令.push(0xFF);
        远跳转指令.push(0x25);
        远跳转指令.push(0x0);
        远跳转指令.push(0x0);
        远跳转指令.push(0x0);
        远跳转指令.push(0x0);

        远跳转指令.extend_from_slice((原指令的下一指令地址 as isize).to_le_bytes().as_ref());

        libmem::write_memory_ex(
            &libmem::get_process().unwrap(),
            跳转地址.byte_add(扫描结束的偏移) as usize,
            远跳转指令.as_slice(),
        )?;

        if hudhook::mh::MH_CreateHook(
            hook.目标地址 as *mut ::core::ffi::c_void,
            跳转地址,
            ::core::ptr::null_mut(),
        ) != hudhook::mh::MH_STATUS::MH_OK
        {
            return None;
        }

        Some(hook)
    }

    pub(crate) unsafe fn 打开(&mut self) {
        let _ = hudhook::mh::MH_EnableHook(self.目标地址 as *mut ::core::ffi::c_void);
    }

    pub(crate) unsafe fn 切换开关(&mut self) {
        if !self.开关 {
            let _ = hudhook::mh::MH_DisableHook(self.目标地址 as *mut ::core::ffi::c_void);
            return;
        }

        let _ = hudhook::mh::MH_EnableHook(self.目标地址 as *mut ::core::ffi::c_void);
    }
}
