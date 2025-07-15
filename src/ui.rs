use crate::{Crop, GLOBAL_HOOK, TimeSpeed, Trainer};

impl hudhook::ImguiRenderLoop for Trainer {
    unsafe fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        crate::init::on_ui_start(self, _ctx, _render_context);
    }

    unsafe fn before_render<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        static mut 按键按下: bool = false;

        if (hudhook::windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(
            hudhook::windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_3.0 as i32,
        ) as u16
            & 0x8000)
            != 0
        {
            if !按键按下 {
                按键按下 = true;

                self.显示界面 = !self.显示界面;
            }
        } else if 按键按下 {
            按键按下 = false;
        }

        if !self.显示界面 {
            _ctx.io_mut().mouse_draw_cursor = false;

            return;
        }

        _ctx.io_mut().mouse_draw_cursor = true;
    }

    unsafe fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        if !self.显示界面 {
            return;
        }
        let window_width = ui.io().display_size[0];
        let window_height = ui.io().display_size[1];

        ui.window("[~]键 (在Esc键下面)")
            .size(
                [window_width / 3.0, window_height / 2.0],
                hudhook::imgui::Condition::FirstUseEver,
            )
            .position(
                [window_width / 2.0, window_height / 2.0],
                hudhook::imgui::Condition::FirstUseEver,
            )
            .position_pivot([0.5, 0.5])
            // .movable(false)
            // .resizable(false)
            .collapsible(false)
            .build(|| unsafe {
                if GLOBAL_HOOK.时间指针.is_null() {
                    // ui.text_colored([1.0, 0.0, 0.0, 1.0], "等待载入游戏存档......");

                    draw_centered_spinner(ui, 100.0, [1.0, 0.0, 0.0, 1.0]);

                    return;
                };

                draw_panel(self, ui)
            });
    }
}

/// 绘制一个旋转的加载圆弧
fn draw_centered_spinner(ui: &hudhook::imgui::Ui, radius: f32, color: [f32; 4]) {
    use hudhook::imgui::{ImColor32, sys::ImVec2};

    // 获取窗口左上角和大小
    let win_pos = ui.window_pos(); // [x, y]
    let win_size = ui.window_size(); // [w, h]

    // 计算窗口内容区域中心点
    let center = ImVec2::new(
        win_pos[0] + win_size[0] * 0.5,
        win_pos[1] + win_size[1] * 0.5,
    );

    // 动态控制旋转
    use std::f32::consts::PI;
    let num_segments = 30;
    let time = ui.time() as f32;
    let start_angle = time * 2.0;
    let arc_angle = PI * 1.5;

    let color_u32 = ImColor32::from_rgba_f32s(color[0], color[1], color[2], color[3]);
    let draw_list = ui.get_window_draw_list();

    let mut last_point: Option<ImVec2> = None;
    for i in 0..=num_segments {
        let a = start_angle + arc_angle * (i as f32) / (num_segments as f32);
        let x = center.x + a.cos() * radius;
        let y = center.y + a.sin() * radius;
        let point = ImVec2::new(x, y);

        if let Some(prev) = last_point {
            draw_list
                .add_line([prev.x, prev.y], [point.x, point.y], color_u32)
                .build();
        }
        last_point = Some(point);
    }

    // 在圆心绘制提示文字
    let text = "等待载入游戏存档";
    let text_size = ui.calc_text_size(text);
    let text_pos = [center.x - text_size[0] / 2.0, center.y - text_size[1] / 2.0];
    let old_pos = ui.cursor_screen_pos();
    ui.set_cursor_screen_pos(text_pos);
    ui.text(text);
    ui.set_cursor_screen_pos(old_pos);
}

unsafe fn draw_panel(trainer: &mut Trainer, ui: &hudhook::imgui::Ui) {
    if let Some(tab_bar) = ui.tab_bar("#tab_bar") {
        if let Some(tab_item) = ui.tab_item("功能") {
            if ui.checkbox("最高金币", &mut GLOBAL_HOOK.金币最大开关) {
                if !hudhook::windows::Win32::System::Memory::IsBadReadPtr(
                    Some(GLOBAL_HOOK.金币地址.cast()),
                    4,
                )
                .as_bool()
                {
                    if GLOBAL_HOOK.金币最大开关 {
                        GLOBAL_HOOK.金币旧值 = GLOBAL_HOOK.金币地址.read();

                        GLOBAL_HOOK.金币地址.write(9999999);
                    } else {
                        GLOBAL_HOOK.金币地址.write(GLOBAL_HOOK.金币旧值);
                    }
                }
            }

            if ui.checkbox("最高木材", &mut GLOBAL_HOOK.木头最大开关) {
                if !hudhook::windows::Win32::System::Memory::IsBadReadPtr(
                    Some(GLOBAL_HOOK.木头地址.cast()),
                    2,
                )
                .as_bool()
                {
                    if GLOBAL_HOOK.木头最大开关 {
                        GLOBAL_HOOK.木头旧值 = GLOBAL_HOOK.木头地址.read();

                        GLOBAL_HOOK.木头地址.write(0x3FFF);
                    } else {
                        GLOBAL_HOOK.木头地址.write(GLOBAL_HOOK.木头旧值);
                    }
                }
            }

            if ui.checkbox("自动钓鱼", &mut GLOBAL_HOOK.自动钓鱼.开关) {
                GLOBAL_HOOK.自动钓鱼.切换开关();

                GLOBAL_HOOK.自动钓鱼按键.开关 = GLOBAL_HOOK.自动钓鱼.开关;

                GLOBAL_HOOK.自动钓鱼按键.切换开关();
            }

            if ui.checkbox("角色穿墙", &mut GLOBAL_HOOK.角色穿墙.开关) {
                GLOBAL_HOOK.角色穿墙.切换开关();
            }

            if ui.checkbox("居民友谊倍率*100", &mut GLOBAL_HOOK.居民友谊倍率x100.开关)
            {
                GLOBAL_HOOK.居民友谊倍率x100.切换开关()
            }

            if ui.checkbox("技能经验倍率*100", &mut GLOBAL_HOOK.技能经验倍率x100.开关)
            {
                GLOBAL_HOOK.技能经验倍率x100.切换开关()
            }

            if ui.checkbox("战斗经验倍率*100", &mut GLOBAL_HOOK.战斗经验倍率x100.开关)
            {
                GLOBAL_HOOK.战斗经验倍率x100.切换开关()
            }

            if ui.checkbox("战斗伤害倍率*100", &mut GLOBAL_HOOK.战斗伤害倍率x100.开关)
            {
                GLOBAL_HOOK.战斗伤害倍率x100.切换开关()
            }

            if ui.checkbox("魔物必定驯服", &mut GLOBAL_HOOK.魔物必定驯服.开关) {
                GLOBAL_HOOK.魔物必定驯服.切换开关()
            }

            if ui.checkbox("无限接取委托", &mut GLOBAL_HOOK.无限委托.开关) {
                GLOBAL_HOOK.无限委托.切换开关()
            }

            if ui.checkbox("禁止负面状态", &mut GLOBAL_HOOK.禁止负面状态.开关) {
                GLOBAL_HOOK.禁止负面状态.切换开关()
            }

            // if ui.checkbox("百倍伤害", &mut GLOBAL_HOOK.伤害倍率.开关) {
            //     GLOBAL_HOOK.伤害倍率.切换开关()
            // }

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("农田") {
            if ui.checkbox("作物即时成熟", &mut GLOBAL_HOOK.作物立即成熟.开关) {
                GLOBAL_HOOK.作物立即成熟.切换开关()
            }

            if ui.checkbox("耕作所有土地", &mut GLOBAL_HOOK.自动耕作开关) {
                if GLOBAL_HOOK.自动耕作开关 {
                    GLOBAL_HOOK.自动耕作标签 = 1;
                } else {
                    GLOBAL_HOOK.自动耕作标签 = 0;
                }
            }

            if ui.checkbox("土地状态最优", &mut GLOBAL_HOOK.土壤质量开关) {
                if GLOBAL_HOOK.土壤质量开关 {
                    GLOBAL_HOOK.土壤质量标签 = 1;
                } else {
                    GLOBAL_HOOK.土壤质量标签 = 0;
                }
            }

            if ui.checkbox("自动浇水", &mut GLOBAL_HOOK.自动浇水开关) {
                if GLOBAL_HOOK.自动浇水开关 {
                    GLOBAL_HOOK.自动浇水标签 = 1;
                } else {
                    GLOBAL_HOOK.自动浇水标签 = 0;
                }
            }

            if ui.checkbox("自动种植", &mut GLOBAL_HOOK.自动种植开关) {
                if GLOBAL_HOOK.自动种植开关 {
                    GLOBAL_HOOK.自动种植标签 = 1;
                } else {
                    GLOBAL_HOOK.自动种植标签 = 0;
                }
            }

            if GLOBAL_HOOK.自动种植开关 {
                if let Some(cb) = ui.begin_combo("种子类型", trainer.选择的作物.to_string())
                {
                    for current in trainer.作物列表 {
                        if trainer.选择的作物 == *current {
                            ui.set_item_default_focus();
                        }

                        if ui
                            .selectable_config(current.to_string())
                            .selected(trainer.选择的作物 == *current)
                            .build()
                        {
                            trainer.选择的作物 = *current;
                        }
                    }

                    cb.end();
                }

                ui.same_line();

                if ui.button("设置##类型") {
                    GLOBAL_HOOK.作物属性.设置作物类型(trainer.选择的作物);
                }

                if let Some(cb) = ui.begin_combo("种子等级", trainer.选择的作物等级.to_string())
                {
                    for current in trainer.作物等级列表 {
                        if trainer.选择的作物等级 == *current {
                            ui.set_item_default_focus();
                        }

                        if ui
                            .selectable_config(current.to_string())
                            .selected(trainer.选择的作物等级 == *current)
                            .build()
                        {
                            trainer.选择的作物等级 = *current;
                        }
                    }

                    cb.end();
                }

                ui.same_line();

                if ui.button("设置##等级") {
                    GLOBAL_HOOK.作物属性.设置作物等级(trainer.选择的作物等级);
                }

                if let Some(cb) = ui.begin_combo("成长阶段", trainer.选择的作物阶段.to_string())
                {
                    for current in trainer.作物阶段列表 {
                        if trainer.选择的作物阶段 == *current {
                            ui.set_item_default_focus();
                        }

                        if ui
                            .selectable_config(current.to_string())
                            .selected(trainer.选择的作物阶段 == *current)
                            .build()
                        {
                            trainer.选择的作物阶段 = *current;
                        }
                    }

                    cb.end();
                }

                ui.same_line();

                if ui.button("设置##阶段") {
                    GLOBAL_HOOK.作物属性.设置作物阶段(trainer.选择的作物阶段);
                }

                if ui.button("清除农田作物") {
                    trainer.选择的作物 = Crop::无_0;

                    GLOBAL_HOOK.作物属性.设置作物类型(Crop::无_0);
                }
            }

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("时间") {
            if let Some(cb) = ui.begin_combo("秒", trainer.选择的秒.to_string()) {
                for current in &trainer.秒列表 {
                    if trainer.选择的秒 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的秒 == *current)
                        .build()
                    {
                        trainer.选择的秒 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##秒") {
                (*GLOBAL_HOOK.时间指针).秒 = trainer.选择的秒;
            }

            if let Some(cb) = ui.begin_combo("小时", trainer.选择的时.to_string()) {
                for current in &trainer.时列表 {
                    if trainer.选择的时 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的时 == *current)
                        .build()
                    {
                        trainer.选择的时 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##时") {
                (*GLOBAL_HOOK.时间指针).时 = trainer.选择的时;
            }

            if let Some(cb) = ui.begin_combo("天", trainer.选择的天.to_string()) {
                for current in &trainer.天列表 {
                    if trainer.选择的天 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的天 == *current)
                        .build()
                    {
                        trainer.选择的天 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##天") {
                (*GLOBAL_HOOK.时间指针).天 = trainer.选择的天;
            }

            if let Some(cb) = ui.begin_combo("季节", trainer.选择的季节.to_string()) {
                for current in trainer.季节列表 {
                    if trainer.选择的季节 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的季节 == *current)
                        .build()
                    {
                        trainer.选择的季节 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##季节") {
                (*GLOBAL_HOOK.时间指针).季节 = trainer.选择的季节 as u8;
            }

            if let Some(cb) = ui.begin_combo("年", trainer.选择的年.to_string()) {
                for current in &trainer.年列表 {
                    if trainer.选择的年 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的年 == *current)
                        .build()
                    {
                        trainer.选择的年 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##年") {
                (*GLOBAL_HOOK.时间指针).年 = trainer.选择的年;
            }

            if let Some(cb) = ui.begin_combo("流速", trainer.选择的流速.to_string()) {
                for current in trainer.时间流速列表 {
                    if trainer.选择的流速 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的流速 == *current)
                        .build()
                    {
                        trainer.选择的流速 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##流速") {
                (*GLOBAL_HOOK.时间指针).流速 = trainer.选择的流速 as u32;
            }

            if ui.button("暂停时间") {
                trainer.选择的流速 = TimeSpeed::Puase;

                (*GLOBAL_HOOK.时间指针).流速 = trainer.选择的流速 as u32;
            }

            ui.same_line();

            if ui.button("恢复时间") {
                trainer.选择的流速 = TimeSpeed::Normal;

                (*GLOBAL_HOOK.时间指针).流速 = trainer.选择的流速 as u32;
            }

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("物品##物品栏") {
            if let Some(cb) = ui.begin_combo("类型##物品", trainer.选择的物品.to_string())
            {
                for current in trainer.物品列表 {
                    if trainer.选择的物品 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的物品 == *current)
                        .build()
                    {
                        trainer.选择的物品 = *current;
                    }
                }

                cb.end();
            }

            if let Some(cb) = ui.begin_combo("数量", trainer.选择的物品数量.to_string()) {
                for current in &trainer.物品数量 {
                    if trainer.选择的物品数量 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(trainer.选择的物品数量 == *current)
                        .build()
                    {
                        trainer.选择的物品数量 = *current;
                    }
                }

                cb.end();
            }

            if ui.button("设置##物品") {
                if !hudhook::windows::Win32::System::Memory::IsBadReadPtr(
                    Some(GLOBAL_HOOK.物品指针.cast()),
                    2,
                )
                .as_bool()
                {
                    (*GLOBAL_HOOK.物品指针)
                        .设置(trainer.选择的物品 as u16, trainer.选择的物品数量);
                }
            }

            // ui.text(format!("{:p}", GLOBAL_HOOK.物品指针));

            tab_item.end();
        }

        tab_bar.end();
    };
}
