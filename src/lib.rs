#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]

use crate::hook::AsmHook;

mod dllmain;
mod hook;
mod init;
mod ui;

static mut GLOBAL_SANDLL_INFO: libmem::Module = libmem::Module {
    base: 0,
    end: 0,
    size: 0,
    path: String::new(),
    name: String::new(),
};

static mut GLOBAL_HOOK: TrainerHook = TrainerHook {
    金币地址: ::core::ptr::null_mut(),
    金币旧值: 0,
    金币最大开关: false,

    木头地址: ::core::ptr::null_mut(),
    木头旧值: 0,
    木头最大开关: false,

    自动钓鱼: AsmHook {
        目标地址: 0,
        开关: false,
    },
    自动钓鱼按键: AsmHook {
        目标地址: 0,
        开关: false,
    },
    自动钓鱼按键标签: 0,

    角色穿墙: AsmHook {
        目标地址: 0,
        开关: false,
    },

    战斗伤害倍率x100: AsmHook {
        目标地址: 0,
        开关: false,
    },

    居民友谊倍率x100: AsmHook {
        目标地址: 0,
        开关: false,
    },

    作物立即成熟: AsmHook {
        目标地址: 0,
        开关: false,
    },

    技能经验倍率x100: AsmHook {
        目标地址: 0,
        开关: false,
    },

    无限委托: AsmHook {
        目标地址: 0,
        开关: false,
    },

    战斗经验倍率x100: AsmHook {
        目标地址: 0,
        开关: false,
    },

    魔物必定驯服: AsmHook {
        目标地址: 0,
        开关: false,
    },

    禁止负面状态: AsmHook {
        目标地址: 0,
        开关: false,
    },

    农田: AsmHook {
        目标地址: 0,
        开关: false,
    },

    土壤质量开关: false,
    土壤质量标签: 0,

    自动耕作开关: false,
    自动耕作标签: 0,

    自动浇水开关: false,
    自动浇水标签: 0,

    自动种植开关: false,
    自动种植标签: 0,

    作物属性: crate::CropProperties {
        类型: 0,
        状态: crate::CropStates { 阶段: 0 },
    },

    时间: AsmHook {
        目标地址: 0,
        开关: false,
    },
    时间指针: ::core::ptr::null_mut(),

    物品: AsmHook {
        目标地址: 0,
        开关: false,
    },
    物品指针: ::core::ptr::null_mut(),
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
// 季节
enum Season {
    // 春
    Spring = 0,

    //夏
    Summer = 1,

    // 秋
    Autumn = 2,

    // 冬
    Winter = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
// 时间速度
enum TimeSpeed {
    // 暂停
    Puase = 0,

    // 1%
    Percent1 = 0x3D,

    // 10%
    Percent10 = 0x266,

    // 25%
    Percent25 = 0x600,

    // 50%
    Percent50 = 0xC00,

    // 默认
    Normal = 0x1800,

    // 150%
    Percent = 0x2400,

    // 200%
    Percent200 = 0x3000,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Crop {
    无_0 = 0,

    石头_1 = 1, // 可捡
    岩石_2 = 2, // 可砸
    树枝_3 = 3, // 可捡
    树桩_4 = 4, // 可劈
    木材_5 = 5, // 可砸，什么都不会出
    毒沼_6 = 6, // 可砸，什么都不会出

    // 矿石_7, // 砸会闪退
    药草_8 = 8,     // 可捡
    解毒草_9 = 9,   // 可捡
    黑草_10 = 10,   // 可捡
    枯草_11 = 11,   // 可捡
    黄草_14 = 14,   // 可捡
    苦橙草_15 = 15, // 可捡

    // 种子_16, // 不可捡，名字就叫 “种子
    杂草_17 = 17,   // 可捡
    季节岩_18 = 18, // 可砸
    花卉_19 = 19,   // 可摧毁

    水晶_20 = 20, // 可砸，出的不知道是不是buff

    // 苹果_21, //  可砸，什么都不会出
    // 苹果_22    同上
    // 苹果_23    同上
    草莓_24 = 24,
    卷心菜_25 = 25,
    樱芜菁_26 = 26,
    洋葱_27 = 27,
    托伊药草_28 = 28,
    月落草_29 = 29,
    樱草_30 = 30,
    灯草_31 = 31,
    金刚花_32 = 32,
    青水晶_33 = 33,
    金卷心菜_34 = 34,
    少女蜜瓜_35 = 35,

    竹笋_36 = 36, // 可割

    南瓜_37 = 37,
    黄瓜_38 = 38,
    玉米_39 = 39,
    番茄_40 = 40,
    茄子_41 = 41,
    菠萝_42 = 42,
    粉红猫_43 = 43,
    铁千轮_44 = 44,
    四叶草_45 = 45,
    原之焰火_46 = 46,
    绿水晶_47 = 47,
    金南瓜_48 = 48,

    蓝草_49 = 49, // 可捡
    绿草_50 = 50, // 可捡
    紫草_51 = 51, // 可捡
    靛草_52 = 52, // 可捡

    番薯_53 = 53,
    马铃薯_54 = 54,
    胡萝卜_55 = 55,
    青椒_56 = 56,
    菠菜_57 = 57,
    魅蓝草_58 = 58,
    红叶花_59 = 59,
    剧毒蒲公英_60 = 60,
    红水晶_61 = 61,
    金马铃薯_62 = 62,
    芜菁_63 = 63,
    白萝卜_64 = 64,
    葱_65 = 65,
    白菜_66 = 66,
    树形草_67 = 67,
    白水晶_68 = 68,
    金芜青_69 = 69,
    火热果实_70 = 70,

    白草_71 = 71, // 可捡

                  // 72以后无效
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum CropLevel {
    LV1 = 0,
    LV2 = 1,
    LV3 = 2,
    LV4 = 3,
    LV5 = 4,
    LV6 = 5,
    LV7 = 6,
    LV8 = 7,
    LV9 = 8,
    LV10 = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum CropStage {
    // 无 = 0,
    一阶段 = 0x1,
    二阶段 = 0x2,
    三阶段 = 0x3,
    四阶段 = 0x4,
    五阶段 = 0x5,
}

#[test]
fn a() {
    println!("{}", std::mem::size_of::<Item>());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Item {
    // Cabbage
    卷心菜_1 = 1,
    // Pink Turnip
    樱芜菁_2 = 2,
    // Pink Melon
    少女蜜瓜_3 = 3,
    // Onion
    洋葱_4 = 4,
    // Pumpkin
    南瓜_5 = 5,
    // Cucumber
    黄瓜_6 = 6,
    // Corn
    玉米_7 = 7,
    // Tomato
    番茄_8 = 8,
    // Eggplant
    茄子_9 = 9,
    // Yam
    番薯_10 = 10,
    // Potato
    马铃薯_11 = 11,
    // Carrot
    胡萝卜_12 = 12,
    // Green Pepper
    青椒_13 = 13,
    // Spinach
    菠菜_14 = 14,
    // Turnip
    芜菁_15 = 15,
    // Radish
    白萝卜_16 = 16,
    // Leek
    葱_17 = 17,
    // Napa Cabbage
    白菜_18 = 18,
    // Hot-Hot Fruit
    火热果实_19 = 19,
    // Bamboo Shoot
    竹笋_20 = 20,
    // Golden King Cabbage
    金卷心菜_21 = 21,
    // Golden Pumpkin
    金南瓜_22 = 22,
    // Golden Potato
    金马铃薯_23 = 23,
    // Golden Turnip
    金芜菁_24 = 24,
    // Mushroom
    蘑菇A_25 = 25,
    // Mushroom
    蘑菇B_26 = 26,
    // Mushroom
    蘑菇C_27 = 27,
    // Mushroom
    蘑菇D_28 = 28,
    // Mushroom
    蘑菇E_29 = 29,
    // Mushroom
    蘑菇F_30 = 30,
    // Strawberry
    草莓_31 = 31,
    // Pineapple
    菠萝_32 = 32,
    // Grapes
    葡萄_33 = 33,
    // Apple
    苹果_34 = 34,
    // Orange
    橙子_35 = 35,
    // Toyherb
    托伊药草_36 = 36,
    // Moondrop Flower
    月落花_37 = 37,
    // Pink Cat
    粉红猫_38 = 38,
    // Charm Blue
    魅蓝草_39 = 39,
    // Medicinal Herb
    药草_40 = 40,
    // Antidote Grass
    解毒草_41 = 41,
    // Black Grass
    黑草_42 = 42,
    // Orange Grass
    苦橙草_43 = 43,
    // Red Grass
    红草_44 = 44,
    // Yellow Grass
    黄草_45 = 45,
    // Blue Grass
    蓝草_46 = 46,
    // Green Grass
    绿草_47 = 47,
    // Purple Grass
    紫草_48 = 48,
    // Indigo Grass
    靛草_49 = 49,
    // White Grass
    白草_50 = 50,
    // Weeds
    杂草_51 = 51,
    // Withered Grass
    枯草_52 = 52,
    // Cherry Grass
    樱草_53 = 53,
    // Lamp Grass
    灯草_54 = 54,
    // Blue Crystal Flower
    青水花_55 = 55,
    // Emery Flower
    金刚花_56 = 56,
    // Ironleaf
    铁千轮_57 = 57,
    // 4-Leaf Clover
    四叶草_58 = 58,
    // Fireflower
    原之焰火_59 = 59,
    // Green Crystal Flower
    绿水晶_60 = 60,
    // Noel Grass
    树形草_61 = 61,
    // Autumn Grass
    红叶花_62 = 62,
    // Pom-Pom Grass
    剧毒蒲公英_63 = 63,
    // Red Crystal Flower
    红水晶_64 = 64,
    // White Crystal Flower
    白水晶_65 = 65,
    // The Protein
    蛋白质_66 = 66,
    // Intelligencer
    智力补品_67 = 67,
    // Vital Gummi
    生命果冻_68 = 68,
    // Heart Drink
    心灵饮料_69 = 69,
    // Antidote Potion
    解毒药_70 = 70,
    // Para-Gone
    解麻药_71 = 71,
    // Roundoff
    解封药_72 = 72,
    // Cold Medicine
    感冒药_73 = 73,
    // Formuade
    超级营养药_74 = 74,
    // Love Potion
    爱情饮料_75 = 75,
    // Invinciroid
    无敌秘药_76 = 76,
    // Leveliser
    符文药水_77 = 77,
    // Heavy Spice
    地狱辣椒_78 = 78,
    // Sweet Powder
    甘甜粉_79 = 79,
    // Sour Drop
    酸味饮料_80 = 80,
    // Mixed Herbs
    混合香料_81 = 81,
    // Failed Dish
    失败之作_82 = 82,
    // Disastrous Dish
    超级失败之作_83 = 83,
    // Masu Trout Sashimi
    石川鲑鱼生鱼片_84 = 84,
    // Char Sashimi
    红点鲑生鱼片_85 = 85,
    // Rainbow Trout Sashimi
    剧毒虹鳟生鱼片_86 = 86,
    // Salmon Sashimi
    钩吻鲑生鱼片_87 = 87,
    // Cherry Salmon Sashimi
    山女鳟生鱼片_88 = 88,
    // Taimen Sashimi
    伊富鱼生鱼片_89 = 89,
    // Snapper Sashimi
    鲷鱼生鱼片_90 = 90,
    // Glitter Snapper Sashimi
    闪光鲷鱼生鱼片_91 = 91,
    // Throb. Snapper Sashimi
    心动鲷生鱼片_92 = 92,
    // Girella Sashimi
    黑毛鱼生鱼片_93 = 93,
    // Skipjack Sashimi
    鲣鱼生鱼片_94 = 94,
    // Yellowtail Sashimi
    青甘鱼生鱼片_95 = 95,
    // Fatty Tuna Sashimi
    金枪鱼大脂_96 = 96,
    // Sardine Sashimi
    沙丁鱼生鱼片_97 = 97,
    // Needlefish Sashimi
    针鱼生鱼片_98 = 98,
    // Pike Sashimi
    秋刀鱼生鱼片_99 = 99,
    // Flounder Sashimi
    比目鱼生鱼片_100 = 100,
    // Turbot Sashimi
    鲽鱼鲆生鱼片_101 = 101,
    // Aut. Flounder Sashimi
    红叶鱼生鱼片_102 = 102,
    // Squid Sashimi
    乌贼刺身_103 = 103,
    // Sunsquid Sashimi
    斑乌贼刺身_104 = 104,
    // Lamp Squid Sashimi
    封印乌贼刺身_105 = 105,
    // Blowfish Sashimi
    河豚生鱼片_106 = 106,
    // Lobster Sashimi
    螯龙虾刺身_107 = 107,
    // Shrimp Sashimi
    虾刺身_108 = 108,
    // Stir-Fried Veggies
    炒蔬菜_109 = 109,
    // Fried Rice
    炒饭_110 = 110,
    // Cabbage Cakes
    什锦烧_111 = 111,
    // French Fries
    炸薯条_112 = 112,
    // Croquettes
    可乐饼_113 = 113,
    // Popcorn
    爆米花_114 = 114,
    // Corn Cereal
    玉米片_115 = 115,
    // Miso Eggplant
    茄子田乐烧_116 = 116,
    // Rolled Eggs
    玉子烧_117 = 117,
    // Omelet
    欧姆蛋_118 = 118,
    // Omelet Rice
    蛋包饭_119 = 119,
    // Baked Apple
    烤苹果_120 = 120,
    // Curry Bread
    咖喱面包_121 = 121,
    // French Toast
    法式吐司_122 = 122,
    // Donuts
    甜甜圈_123 = 123,
    // Fried Udon
    炒乌冬面_124 = 124,
    // Tempura
    天妇罗_125 = 125,
    // Pancakes
    美式松饼_126 = 126,
    // Gyoza
    煎饺_127 = 127,
    // Risotto
    烩饭_128 = 128,
    // Dry Curry
    干咖喱料理_129 = 129,
    // Salted Masu Trout
    盐烤石川鲑鱼_130 = 130,
    // Salted Char
    盐烤红点鲑_131 = 131,
    // Salted Rainbow Trout
    烤剧毒虹鳟_132 = 132,
    // Salted Cherry Salmon
    盐烤山女鳟_133 = 133,
    // Salted Chub
    盐烤珠星三块鱼_134 = 134,
    // Salted Salmon
    盐烤鲑鱼_135 = 135,
    // Salted Taimen
    伊富鱼烤全鱼_136 = 136,
    // Grilled Crucian Carp
    高身鲫烤全鱼_137 = 137,
    // Grilled Gibelio
    鲋鱼烤全鱼_138 = 138,
    // Grilled Snapper
    烤鲷鱼料理_139 = 139,
    // Grilled Girella
    盐烤黑毛鱼_140 = 140,
    // Grilled Glitter Snapper
    烤闪光鲷鱼_141 = 141,
    // Grilled Throb. Snapper
    烤心动鲷鱼_142 = 142,
    // Grilled Skipjack
    土佐风烤鲣鱼_143 = 143,
    // Grilled Mackerel
    烤青花鱼_144 = 144,
    // Yellowtail Teriyaki
    照烧青甘鱼_145 = 145,
    // Salted Pond Smelt
    盐烤西太公鱼_146 = 146,
    // Tuna Teriyaki
    照烧金枪鱼_147 = 147,
    // Dried Sardines
    沙丁鱼串_148 = 148,
    // Grilled Needlefish
    盐烤针鱼_149 = 149,
    // Salted Pike
    盐烤秋刀鱼_150 = 150,
    // Grilled Flounder
    烤比目鱼_151 = 151,
    // Grilled Turbot
    烤鲽鱼_152 = 152,
    // Grilled Aut. Flounder
    烤红叶比目鱼_153 = 153,
    // Grilled Squid
    烤乌贼_154 = 154,
    // Grilled Sunsquid
    烤斑乌贼_155 = 155,
    // Grilled Lamp Squid
    烤封印乌贼_156 = 156,
    // Grilled Blowfish
    烤河豚_157 = 157,
    // Grilled Lobster
    螯龙虾烤全虾_158 = 158,
    // Grilled Shrimp
    烤全虾_159 = 159,
    // Grilled Sand Flounder
    烤沙比目鱼_160 = 160,
    // Hot Milk
    热牛奶_161 = 161,
    // Hot Chocolate
    热可可_162 = 162,
    // Grape "Liqueur"
    葡萄酒_163 = 163,
    // Boiled Pumpkin
    炖南瓜_164 = 164,
    // Boiled Spinach
    凉拌菠菜_165 = 165,
    // Boiled Egg
    水煮蛋_166 = 166,
    // Glazed Yam
    拔丝地瓜_167 = 167,
    // Boiled Gyoza
    水饺_168 = 168,
    // Strawberry Jam
    草莓酱_169 = 169,
    // Apple Jam
    苹果酱_170 = 170,
    // Grape Jam
    葡萄酱_171 = 171,
    // Marmalade
    柑橘酱_172 = 172,
    // Cheese Fondue
    奶酪火锅_173 = 173,
    // Udon
    乌冬面_174 = 174,
    // Curry Udon
    咖喱乌冬面_175 = 175,
    // Tempura Udon
    天妇罗乌冬面_176 = 176,
    // Rice Porridge
    粥_177 = 177,
    // Milk Porridge
    牛奶粥_178 = 178,
    // Tempura Bowl
    天妇罗盖饭_179 = 179,
    // Egg Bowl
    鸡蛋盖饭_180 = 180,
    // Stew
    炖菜_181 = 181,
    // Curry Rice
    咖喱饭_182 = 182,
    // Ultimate Curry
    究极咖喱_183 = 183,
    // Royal Curry
    至尊咖喱_184 = 184,
    // Relax Tea
    放松茶_185 = 185,
    // Grilled Miso
    味增田乐烧_186 = 186,
    // Union Stew
    什锦火锅_187 = 187,
    // Stewed Rockfish
    炖煮岩石鱼_188 = 188,
    // Corn on the Cob
    烤玉米_189 = 189,
    // Baked Onigiri
    烤饭团_190 = 190,
    // Sweet Potato
    烤番薯_191 = 191,
    // Toast
    吐司_192 = 192,
    // Jam Roll
    果酱面包_193 = 193,
    // Butter Roll
    黄油面包卷_194 = 194,
    // Pizza
    披萨_195 = 195,
    // Seafood Pizza
    海鲜披萨_196 = 196,
    // Doria
    多利亚饭_197 = 197,
    // Seafood Doria
    海鲜多利亚饭_198 = 198,
    // Gratin
    奶油烤菜_199 = 199,
    // Seafood Gratin
    海鲜奶油烤菜_200 = 200,
    // Yam of the Ages
    蜜番薯_201 = 201,
    // Cookie
    曲奇饼干_202 = 202,
    // Chocolate Cookie
    巧克力曲奇_203 = 203,
    // Cake
    蛋糕_204 = 204,
    // Chocolate Cake
    巧克力蛋糕_205 = 205,
    // Cheesecake
    奶酪蛋糕_206 = 206,
    // Apple Pie
    苹果派_207 = 207,
    // Pineapple Juice
    菠萝汁_208 = 208,
    // Tomato Juice
    番茄汁_209 = 209,
    // Grape Juice
    葡萄汁_210 = 210,
    // Orange Juice
    橘子汁_211 = 211,
    // Apple Juice
    苹果汁_212 = 212,
    // Strawberry Milk
    草莓牛奶_213 = 213,
    // Fruit Juice
    复合果汁_214 = 214,
    // Fruit Smoothie
    水果欧蕾_215 = 215,
    // Vegetable Juice
    蔬菜汁_216 = 216,
    // Veggie Smoothie
    蔬菜欧蕾_217 = 217,
    // Mixed Juice
    混合蔬果汁_218 = 218,
    // Mixed Smoothie
    混合蔬果欧蕾_219 = 219,
    // Ketchup
    番茄酱_220 = 220,
    // Butter
    黄油_221 = 221,
    // Gold Juice
    黄金果汁_222 = 222,
    // Prelude to Love
    恋爱的预感_223 = 223,
    // Hot Juice
    热果汁_224 = 224,
    // Steamed Bread
    蒸面包_225 = 225,
    // Cheese Bread
    奶酪蒸面包_226 = 226,
    // Meat Dumpling
    烧麦_227 = 227,
    // Manju
    包子_228 = 228,
    // Curry Manju
    咖喱包子_229 = 229,
    // Steamed Gyoza
    蒸饺_230 = 230,
    // Pound Cake
    蒸卡斯提拉_231 = 231,
    // Sponge Cake
    蒸蛋糕_232 = 232,
    // Flan
    布丁_233 = 233,
    // Pumpkin Flan
    南瓜布丁_234 = 234,
    // Dumplings
    团子_235 = 235,
    // Salad
    色拉_236 = 236,
    // Sandwich
    三明治_237 = 237,
    // Fruit Sandwich
    水果三明治_238 = 238,
    // Pickled Turnip
    腌芜菁_239 = 239,
    // Pickles
    腌黄瓜_240 = 240,
    // Bamboo Rice
    竹笋饭_241 = 241,
    // Raisin Bread
    葡萄干面包_242 = 242,
    // Ice Cream
    冰淇淋_243 = 243,
    // Relax Tea Leaves
    放松茶的茶叶_244 = 244,
    // Onigiri
    饭团_245 = 245,
    // Bread
    面包_246 = 246,
    // Salmon Onigiri
    鲑鱼饭团_247 = 247,
    // Pickle Mix
    腌红白芜菁_248 = 248,
    // Turnip Heaven
    芜菁天堂_249 = 249,
    // Rice
    米饭_250 = 250,
    // Chocolate
    巧克力_251 = 251,
    // Wine
    红酒_252 = 252,
    // Elli Leaves
    艾丽草_253 = 253,
    // Milk (S)
    牛奶_254 = 254,
    // Milk (M)
    优质牛奶_255 = 255,
    // Milk (L)
    钙质满分牛奶_256 = 256,
    // Egg (S)
    鸡蛋_257 = 257,
    // Egg (M)
    优质鸡蛋_258 = 258,
    // Egg (L)
    每日健康蛋_259 = 259,
    // Mayonnaise (S)
    蛋黄酱S_260 = 260,
    // Mayonnaise (M)
    蛋黄酱_261 = 261,
    // Mayonnaise (L)
    蛋黄酱L_262 = 262,
    // Cheese (S)
    奶酪S_263 = 263,
    // Cheese (M)
    奶酪_264 = 264,
    // Cheese (L)
    奶酪L_265 = 265,
    // Yogurt (S)
    酸奶S_266 = 266,
    // Yogurt (M)
    酸奶_267 = 267,
    // Yogurt (L)
    酸奶L_268 = 268,
    // Honey
    蜂蜜_269 = 269,
    // Flour
    小麦粉_270 = 270,
    // Oil
    油_271 = 271,
    // Curry Powder
    咖喱粉_272 = 272,
    // Rice Flour
    团子粉_273 = 273,
    // Medicine Bread
    药学低级配方面包_274 = 274,
    // Medicine Bread+
    药学高级配方面包_275 = 275,
    // Cooking Bread
    料理低级配方面包_276 = 276,
    // Cooking Bread+
    料理高级配方面包_277 = 277,
    // Weapon Bread
    武器低级配方面包_278 = 278,
    // Weapon Bread+
    武器高级配方面包_279 = 279,
    // Accessory Bread
    装饰低级配方面包_280 = 280,
    // Accessory Bread+
    装饰高级配方面包_281 = 281,
    // Farming Bread
    农具低级配方面包_282 = 282,
    // Farming Bread+
    农具高级配方面包_283 = 283,
    // Broadsword
    阔剑_284 = 284,
    // Steel Sword
    矿石剑_285 = 285,
    // Wind Sword
    风之剑_286 = 286,
    // Aqua Sword
    水星剑_287 = 287,
    // Defender
    防卫者_288 = 288,
    // Aerial Blade
    大气锋刃_289 = 289,
    // Burning Sword
    烈焰剑_290 = 290,
    // Sakura
    红樱_291 = 291,
    // Luck Blade
    幸运剑_292 = 292,
    // Platinum Sword
    白金剑_293 = 293,
    // Raventine
    胜利之剑_294 = 294,
    // Icifier
    死亡之剑_295 = 295,
    // Soul Eater
    噬魂剑_296 = 296,
    // Smash Blade
    粉碎之剑_297 = 297,
    // Dragon Slayer
    屠龙剑_298 = 298,
    // Sunspot
    太阳之剑_299 = 299,
    // Star Saber
    星辰军刀_300 = 300,
    // Gaia Sword
    盖亚之剑_301 = 301,
    // Grantale
    格兰泰尔_302 = 302,
    // Chaos Blade
    混沌剑_303 = 303,
    // Rune Blade
    符文剑_304 = 304,
    // Steel Sword+
    矿石剑改_305 = 305,
    // Platinum Sword+
    白金剑改_306 = 306,
    // Cutlass
    弯刀_307 = 307,
    // Back Scratcher
    痒痒挠_308 = 308,
    // Durendal
    迪朗达尔_309 = 309,
    // Gladius
    古拉迪乌斯_310 = 310,
    // Gorgeous Sword
    奢华之剑_311 = 311,
    // Spoon
    勺子_312 = 312,
    // Rune Legend
    符文传说_313 = 313,
    // Snakesword
    蛇腹剑_314 = 314,
    // Veggieblade
    蔬菜剑_315 = 315,
    // Invisiblade
    隐形剑_316 = 316,
    // Claymore
    大剑_317 = 317,
    // Zweihaender
    双手剑_318 = 318,
    // Flame Saber
    烈焰大剑_319 = 319,
    // Cyclone Blade
    气旋剑_320 = 320,
    // Dancing Dicer
    斩斩舞_321 = 321,
    // Great Sword
    巨剑_322 = 322,
    // Heaven Asunder
    天之村云之剑_323 = 323,
    // Grand Smasher
    大地粉碎者_324 = 324,
    // Blue-Eyed Blade
    苍眼太刀_325 = 325,
    // Poison Blade
    毒素剑_326 = 326,
    // Steel Slicer
    名刀达忍_327 = 327,
    // Flamberge
    焰形剑_328 = 328,
    // Shine Blade
    闪亮剑_329 = 329,
    // Earth Shade
    大地之影_330 = 330,
    // Bio Smasher
    生物粉碎者_331 = 331,
    // Punisher
    制裁者_332 = 332,
    // Sea Cutter
    海洋切割者_333 = 333,
    // Volcanon
    波尔卡农_334 = 334,
    // Snow Crown
    白雪皇冠_335 = 335,
    // Moon Shadow
    月影_336 = 336,
    // Force Element
    四元素剑_337 = 337,
    // Zweihaender+
    双手剑改_338 = 338,
    // Flamberge+
    焰形剑改_339 = 339,
    // Katzbalger
    斗剑_340 = 340,
    // Big Knife
    巨大匕首_341 = 341,
    // Katana
    太刀_342 = 342,
    // Balmung
    巴尔蒙克_343 = 343,
    // Psycho
    狂乱刃_344 = 344,
    // Dekash+
    大鱼_345 = 345,
    // Braveheart
    勇敢之心_346 = 346,
    // Belzebuth
    别西卜_347 = 347,
    // Daicone
    白萝卜剑_348 = 348,
    // Caliburn
    王者之剑_349 = 349,
    // Orochi
    八岐大蛇_350 = 350,
    // Spear
    长枪_351 = 351,
    // Lance
    骑士枪_352 = 352,
    // Needle Spear
    针枪_353 = 353,
    // Halberd
    斧枪_354 = 354,
    // Water Spear
    水之枪_355 = 355,
    // Blood Lance
    血枪_356 = 356,
    // Wood Staff
    棍棒_357 = 357,
    // Poison Spear
    毒枪_358 = 358,
    // Corsesca
    科西嘉枪_359 = 359,
    // Silent Grave
    沉默戟刀_360 = 360,
    // Flare Lance
    烈焰长枪_361 = 361,
    // Heavy Lance
    重枪_362 = 362,
    // Iseberk
    冰山枪_363 = 363,
    // Metus
    恐惧_364 = 364,
    // Monk Staff
    金箍棒_365 = 365,
    // Overbreak
    骤变者_366 = 366,
    // Brionac
    贯魔神枪_367 = 367,
    // Feather Lance
    羽毛枪_368 = 368,
    // Belvarose
    贝鲁伐洛斯_369 = 369,
    // Bjor
    剧毒枪_370 = 370,
    // Gungnir
    永恒之枪_371 = 371,
    // Lance+
    骑士枪改_372 = 372,
    // Corsesca+
    科西嘉枪改_373 = 373,
    // Trident
    三叉戟_374 = 374,
    // Pitchfork
    草叉_375 = 375,
    // Dragon's Fang
    龙之牙_376 = 376,
    // Gae Bolg
    盖伯尔加之枪_377 = 377,
    // Magical Lance
    魔法枪_378 = 378,
    // Safety Lance
    感觉不怎么痛的枪_379 = 379,
    // Legion
    灵枪_380 = 380,
    // Poison Queen
    毒后_381 = 381,
    // Pine Club
    菠萝棒_382 = 382,
    // Fivestaff
    五节棍_383 = 383,
    // Cheap Hammer
    破锤子_384 = 384,
    // Bronze Hammer
    青铜锤_385 = 385,
    // Silver Hammer
    银锤_386 = 386,
    // Golden Hammer
    金锤_387 = 387,
    // Platinum Hammer
    白金锤_388 = 388,
    // Battle Hammer
    战斗锤_389 = 389,
    // War Hammer
    战锤_390 = 390,
    // Great Hammer
    大锤_391 = 391,
    // Schnabel
    施纳贝尔_392 = 392,
    // Gigant Hammer
    巨锤_393 = 393,
    // Mjolnir
    雷神之锤_394 = 394,
    // Spiked Hammer
    钉锤_395 = 395,
    // Flame Hammer
    烈焰锤_396 = 396,
    // Ice Hammer
    寒冰锤_397 = 397,
    // Sky Hammer
    天空锤_398 = 398,
    // Graviton Hammer
    大地锤_399 = 399,
    // Bone Hammer
    骨锤_400 = 400,
    // Crystal Hammer
    水晶锤_401 = 401,
    // War Hammer+
    战锤改_402 = 402,
    // Gigant Hammer+
    巨锤改_403 = 403,
    // Hammer
    觉醒的破锤子_404 = 404,
    // Toy Hammer
    哔哔锤_405 = 405,
    // Fatal Crush
    致命冲击_406 = 406,
    // Strong Stone
    试力石_407 = 407,
    // Kongo
    金刚_408 = 408,
    // Bat
    球棒_409 = 409,
    // Iron Bat
    金属球棒_410 = 410,
    // Splash Star
    飞溅之星_411 = 411,
    // Cheap Axe
    破斧_412 = 412,
    // Chopping Axe
    砍柴用斧_413 = 413,
    // Lumber Axe
    砍伐用斧_414 = 414,
    // Mountain Axe
    巡山用斧_415 = 415,
    // Miracle Axe
    奇迹斧_416 = 416,
    // Battle Axe
    战斧_417 = 417,
    // Pole Axe
    枪斧_418 = 418,
    // Alldale
    欧鲁帝鲁_419 = 419,
    // Great Axe
    巨斧_420 = 420,
    // Demon Axe
    恶魔斧_421 = 421,
    // Crescent Axe
    偃月斧_422 = 422,
    // Executioner
    行刑者_423 = 423,
    // Heat Axe
    高热斧_424 = 424,
    // Frost Axe
    冰霜斧_425 = 425,
    // Tomahawk
    投掷战斧_426 = 426,
    // Rock Axe
    洛克斧_427 = 427,
    // Double Edge
    双刃斧_428 = 428,
    // Saint Axe
    圣斧_429 = 429,
    // Pole Axe+
    枪斧改_430 = 430,
    // Crescent Axe+
    偃月斧改_431 = 431,
    // Axe
    认真起来的破斧头_432 = 432,
    // Lollipop
    棒棒糖_433 = 433,
    // Battle Scythe
    战斗镰刀_434 = 434,
    // Basilisk Fang
    翼蜥之牙_435 = 435,
    // Devil Finger
    恶魔之爪_436 = 436,
    // Cheap Sickle
    破镰刀_437 = 437,
    // Iron Sickle
    铁镰刀_438 = 438,
    // Quality Sickle
    优质镰刀_439 = 439,
    // Super Sickle
    利刃镰刀_440 = 440,
    // Legendary Sickle
    名匠镰鼬_441 = 441,
    // Cheap Waterpot
    破洒水壶_443 = 443,
    // Tin Waterpot
    马口铁洒水壶_444 = 444,
    // Lion Waterpot
    狮子洒水壶_445 = 445,
    // Rainbow Waterpot
    彩虹洒水壶_446 = 446,
    // Joy Waterpot
    幸福的洒水壶_447 = 447,
    // Cheap Hoe
    破锄头_449 = 449,
    // Sturdy Hoe
    坚固的锄头_450 = 450,
    // Seasoned Hoe
    熟练的锄头_451 = 451,
    // Shiny Hoe
    闪耀的锄头_452 = 452,
    // Blessed Hoe
    喜悦的锄头_453 = 453,
    // Cheap Pole
    破钓竿_455 = 455,
    // Beginner's Pole
    初学者钓竿_456 = 456,
    // Skilled Pole
    中级者钓竿_457 = 457,
    // Famous Pole
    名人钓竿_458 = 458,
    // Sacred Pole
    神木钓竿_459 = 459,
    // Rod
    魔杖_461 = 461,
    // Staff
    魔法棒_462 = 462,
    // Silver Staff
    银之杖_463 = 463,
    // Flare Staff
    烈焰之杖_464 = 464,
    // Ice Staff
    寒冰之杖_465 = 465,
    // Lightning Wand
    闪电之杖_466 = 466,
    // Earth Staff
    大地之杖_467 = 467,
    // Wizard's Staff
    法师之杖_468 = 468,
    // Mage's Staff
    魔术手杖_469 = 469,
    // Rune Staff
    符文之杖_470 = 470,
    // Mage's Staff+
    魔法手仗_471 = 471,
    // Magic Broom
    魔法扫把_472 = 472,
    // Basket
    篮子_473 = 473,
    // Magic Shot
    魔法注射器_474 = 474,
    // Short Daggers
    短剑_475 = 475,
    // Steel Edge
    钢化刃_476 = 476,
    // Wind Edge
    风之刃_477 = 477,
    // Frost Edge
    冰霜之刃_478 = 478,
    // Steel Blades
    斩钢刃_479 = 479,
    // Sonic Daggers
    音速短剑_480 = 480,
    // Salamander
    沙罗曼达_481 = 481,
    // Twin Blades
    双鬼丸_482 = 482,
    // Rampage
    狂暴_483 = 483,
    // Platinum Edge
    白金之刃_484 = 484,
    // Efreet
    伊弗利特_485 = 485,
    // Deep Blizzard
    终极暴雪_486 = 486,
    // Dark Invitation
    暗黑邀约_487 = 487,
    // Force Divide
    力量切割_488 = 488,
    // Dragon Claws
    龙骑兵之爪_489 = 489,
    // Heart Fire
    火焰之心_490 = 490,
    // Desert Wind
    沙漠之风_491 = 491,
    // Broken Wall
    崩坏之墙_492 = 492,
    // Orcus Swords
    亡神剑_493 = 493,
    // Chaos Edge
    混沌之刃_494 = 494,
    // Rune Edge
    符文之刃_495 = 495,
    // Iron Edge
    钢铁之刃_496 = 496,
    // Emerald Edge
    绿宝石之刃_497 = 497,
    // Thief Knives
    盗贼匕首_498 = 498,
    // Double Scratch
    双重痒痒挠_499 = 499,
    // Priest Sabers
    神官之剑_500 = 500,
    // Earnest Edge
    诚挚之刃_501 = 501,
    // Gorgeous Lx
    超级奢华之剑_502 = 502,
    // Acutorimass
    双过滤勺_503 = 503,
    // Twin Leeks
    双葱_504 = 504,
    // Twin Justice
    双重正义_505 = 505,
    // Small Shield
    迷你盾_506 = 506,
    // Bronze Shield
    铜盾_507 = 507,
    // Round Shield
    圆盾_508 = 508,
    // Platinum Shield
    白金盾_509 = 509,
    // Heavy Shield
    重盾_510 = 510,
    // Knight Shield
    骑士盾_511 = 511,
    // Rune Shield
    符文盾_512 = 512,
    // Magic Shield
    魔法盾_513 = 513,
    // Prism Shield
    棱镜盾_514 = 514,
    // Element Shield
    元素盾_515 = 515,
    // Chaos Shield
    混沌盾_516 = 516,
    // Turtle Shield
    乌龟盾_517 = 517,
    // Bone Shield
    骨盾_518 = 518,
    // Kite Shield
    鸢盾_519 = 519,
    // Magical Shield
    魔术盾_520 = 520,
    // Monkey Plush
    猴子布娃娃_521 = 521,
    // Umbrella
    伞_522 = 522,
    // Aquamarine Ring
    海蓝宝石戒指_523 = 523,
    // Amethyst Ring
    紫水晶戒指_524 = 524,
    // Emerald Ring
    绿宝石戒指_525 = 525,
    // Sapphire Ring
    蓝宝石戒指_526 = 526,
    // Diamond Ring
    钻石戒指_527 = 527,
    // Ruby Ring
    红宝石戒指_528 = 528,
    // Happy Ring
    幸福戒指_529 = 529,
    // Cursed Ring
    暗之戒指_530 = 530,
    // Fire Ring
    火之戒指_531 = 531,
    // Wind Ring
    风之戒指_532 = 532,
    // Water Ring
    水之戒指_533 = 533,
    // Earth Ring
    地之戒指_534 = 534,
    // Silver Ring
    银戒指_535 = 535,
    // Gold Ring
    金戒指_536 = 536,
    // Platinum Ring
    白金戒指_537 = 537,
    // Critical Ring
    暴击戒指_538 = 538,
    // Silent Ring
    静默戒指_539 = 539,
    // Paralysis Ring
    瘫痪戒指_540 = 540,
    // Poison Ring
    毒素戒指_541 = 541,
    // Magic Ring
    魔法戒指_542 = 542,
    // Cheap Bracelet
    廉价手镯_543 = 543,
    // Bronze Bracelet
    铜手镯_544 = 544,
    // Silver Bracelet
    银手镯_545 = 545,
    // Gold Bracelet
    金手镯_546 = 546,
    // Platinum Bracelet
    白金手镯_547 = 547,
    // Aquamarine Brooch
    海蓝宝石胸针_548 = 548,
    // Amethyst Brooch
    紫水晶胸针_549 = 549,
    // Emerald Brooch
    绿宝石胸针_550 = 550,
    // Sapphire Brooch
    蓝宝石胸针_551 = 551,
    // Diamond Brooch
    钻石胸针_552 = 552,
    // Ruby Brooch
    红宝石胸针_553 = 553,
    // Silver Pendant
    银坠饰_554 = 554,
    // Heart Pendant
    心形坠饰_555 = 555,
    // Star Pendant
    星形坠饰_556 = 556,
    // Sun Pendant
    太阳坠饰_557 = 557,
    // Field Pendant
    草原坠饰_558 = 558,
    // Dew Pendant
    水滴坠饰_559 = 559,
    // Earth Pendant
    大地坠饰_560 = 560,
    // Holy Amulet
    神圣坠饰_561 = 561,
    // Charm
    护符_562 = 562,
    // Leather Belt
    皮腰带_563 = 563,
    // Lucky Strike
    暴击之七_564 = 564,
    // Talisman
    辟邪符_565 = 565,
    // Champ Belt
    冠军腰带_566 = 566,
    // Sturdy Gloves
    结实的手套_567 = 567,
    // Work Gloves
    拼命三郎的劳作手套_568 = 568,
    // Badge
    赠品徽章_569 = 569,
    // Power Gloves
    力量手套_570 = 570,
    // Magic Charm
    魔斗的咒符_571 = 571,
    // Shield Ring
    盾戒_572 = 572,
    // Rosary
    停战的祈祷_573 = 573,
    // Courage Badge
    勇气徽章_574 = 574,
    // Hero's Proof
    豪杰之证_575 = 575,
    // Proof of Wisdom
    贤者之证_576 = 576,
    // Hand-Knit Scarf
    手织围巾_577 = 577,
    // Fluffy Scarf
    毛茸茸围巾_578 = 578,
    // Art of Attack
    必杀的秘诀_579 = 579,
    // Art of Defense
    钢铁的秘诀_580 = 580,
    // Art of Magic
    魔法之道的秘诀_581 = 581,
    // Silver Hairpin
    银发簪_582 = 582,
    // Gold Hairpin
    金发簪_583 = 583,
    // Focus Earrings
    专心耳饰_584 = 584,
    // Fancy Hat
    时尚帽子_585 = 585,
    // Witch Earrings
    魔女耳饰_586 = 586,
    // Headband
    胜负头巾_587 = 587,
    // Feathered Hat
    羽毛帽子_588 = 588,
    // Brand Glasses
    名牌眼镜_589 = 589,
    // Fireproof Hood
    防灾头巾_590 = 590,
    // Magic Earrings
    魔法石耳饰_591 = 591,
    // Cute Knitting
    手工针织帽_592 = 592,
    // Feather Boots
    羽毛长靴_593 = 593,
    // Heavy Boots
    重靴_594 = 594,
    // Leather Boots
    皮长靴_595 = 595,
    // Knight Boots
    骑士靴_596 = 596,
    // Snow Boots
    雪靴_597 = 597,
    // Ice Skates
    溜冰鞋_598 = 598,
    // Cheep-Cheep Sandals
    哔哔鞋_599 = 599,
    // Step-In Boots
    稳步靴_600 = 600,
    // Ghost Boots
    幽灵靴_601 = 601,
    // Iron Geta
    铁屐靴_602 = 602,
    // Strider Boots
    阔步靴_603 = 603,
    // Secret Shoes
    内增高鞋_604 = 604,
    // Wet Boots
    光滑靴_605 = 605,
    // Sneaking Boots
    隐迹靴_606 = 606,
    // Fast Step Boots
    步步运动靴_607 = 607,
    // Water Shoes
    水蜘蛛_608 = 608,
    // Rocket Wing
    火箭之翼_609 = 609,
    // Silver Boots
    银靴_610 = 610,
    // Gold Boots
    金靴_611 = 611,
    // Bone Boots
    骨靴_612 = 612,
    // Fairy Boots
    小仙子靴_613 = 613,
    // Brush
    毛刷_614 = 614,
    // Clippers
    剪毛刀_615 = 615,
    // Empty Bottle
    空瓶_616 = 616,
    // Recovery Potion
    恢复壶_617 = 617,
    // Healing Potion
    疗愈壶_618 = 618,
    // Mystery Potion
    神秘壶_619 = 619,
    // Magnifying Glass
    放大镜_620 = 620,
    // Turnip Seeds
    芜菁种子_621 = 621,
    // Potato Seeds
    马铃薯种子_622 = 622,
    // Cucumber Seeds
    黄瓜种子_623 = 623,
    // Strawberry Seeds
    草莓种子_624 = 624,
    // Cabbage Seeds
    卷心菜种子_625 = 625,
    // Moondrop Seeds
    月落草种子_626 = 626,
    // Toyherb Seeds
    托伊药草种子_627 = 627,
    // Tomato Seeds
    番茄种子_628 = 628,
    // Corn Seeds
    玉米种子_629 = 629,
    // Onion Seeds
    洋葱种子_630 = 630,
    // Pumpkin Seeds
    南瓜种子_631 = 631,
    // Pineapple Seeds
    菠萝种子_632 = 632,
    // Pink Cat Seeds
    粉红猫种子_633 = 633,
    // Eggplant Seeds
    茄子种子_634 = 634,
    // Carrot Seeds
    胡萝卜种子_635 = 635,
    // Yam Seeds
    番薯种子_636 = 636,
    // Spinach Seeds
    菠菜种子_637 = 637,
    // Green Pepper Seeds
    青椒种子_638 = 638,
    // Charm Blue Seeds
    魅蓝草种子_639 = 639,
    // Fodder Seeds
    牧草种子_640 = 640,
    // Cherry Grass Seeds
    樱草种子_641 = 641,
    // Lamp Grass Seeds
    灯草种子_642 = 642,
    // Blue Crystal Seeds
    青水晶花种子_643 = 643,
    // Emery Flower Seeds
    金刚花种子_644 = 644,
    // Ironleaf Seeds
    铁千轮种子_645 = 645,
    // Clover Seeds
    幸运草种子_646 = 646,
    // Fireflower Seeds
    原之焰火种子_647 = 647,
    // Green Crystal Seeds
    绿水晶花种子_648 = 648,
    // Noel Grass Seeds
    树形草种子_649 = 649,
    // Autumn Grass Seeds
    红叶花种子_650 = 650,
    // Pom-Pom Grass Seeds
    剧毒蒲公英种子_651 = 651,
    // Red Crystal Seeds
    红水晶种子_652 = 652,
    // White Crystal Seeds
    白水晶种子_653 = 653,
    // Pink Turnip Seeds
    樱芜菁种子_654 = 654,
    // Radish Seeds
    白萝卜种子_655 = 655,
    // Leek Seeds
    葱种子_656 = 656,
    // Napa Cabbage Seeds
    白菜种子_657 = 657,
    // Golden Cabbage Seeds
    金卷心菜种子_658 = 658,
    // Golden Pumpkin Seeds
    金南瓜种子_659 = 659,
    // Golden Potato Seeds
    金马铃薯种子_660 = 660,
    // Golden Turnip Seeds
    金芜菁种子_661 = 661,
    // Pink Melon Seeds
    少女蜜瓜种子_662 = 662,
    // Hot-Hot Seeds
    火热果实种子_663 = 663,
    // Escape
    瞬移_664 = 664,
    // Fireball
    火球_665 = 665,
    // Inferno
    大火球_666 = 666,
    // Explosion
    爆破_667 = 667,
    // Water Laser
    激光水波_668 = 668,
    // Parallel Laser
    平行激光水波_669 = 669,
    // Delta Laser
    三角激光水波_670 = 670,
    // Rock Screw
    螺旋石_671 = 671,
    // Earth Spike
    地之尖柱_672 = 672,
    // Avenger Rock
    复仇者之石_673 = 673,
    // Sonic Wind
    音速之风_674 = 674,
    // Double Sonic
    双重音波_675 = 675,
    // Sonic Ricochet
    穿透音波_676 = 676,
    // Light Barrier
    光屏障_677 = 677,
    // Shine
    闪光_678 = 678,
    // Prism
    棱镜_679 = 679,
    // Dark Ball
    黑洞_680 = 680,
    // Dark Snake
    黑蛇_681 = 681,
    // Darkness
    暗黑_682 = 682,
    // Cure
    治愈_683 = 683,
    // Cure All
    全体治愈_684 = 684,
    // Master Cure
    大师治疗_685 = 685,
    // Medipoison
    毒素治愈_686 = 686,
    // Mediparalyze
    瘫痪治愈_687 = 687,
    // Mediseal
    封印治愈_688 = 688,
    // Conductivity
    // 超传导魔法格奥斯 = 689,
    // Power Wave
    能量_690 = 690,
    // Dash Slash
    冲击挥砍_691 = 691,
    // Rush Attack
    猛烈攻击_692 = 692,
    // Round Break
    破坏回旋_693 = 693,
    // Mind Thrust
    精神力突进_694 = 694,
    // Gust
    烈空_695 = 695,
    // Storm
    云裂_696 = 696,
    // Blitz
    瞬迅_697 = 697,
    // Twin Attack
    双刺_698 = 698,
    // Rail Strike
    罗闪_699 = 699,
    // Wind Slash
    旋风剑_700 = 700,
    // Flash Strike
    一心一刀_701 = 701,
    // Naive Blade
    无心剑_702 = 702,
    // Steel Heart
    不屈架式_703 = 703,
    // Delta Strike
    行云流水三段_704 = 704,
    // Hurricane
    天空气旋_705 = 705,
    // Reaper Slash
    奋力收割_706 = 706,
    // Millionstrike
    百万打击_707 = 707,
    // Axel Disaster
    灾难行进_708 = 708,
    // Stardust Upper
    星尘飞升_709 = 709,
    // Tornado Swing
    龙卷挥击_710 = 710,
    // Grand Impact
    大地冲击_711 = 711,
    // Giga Swing
    百万挥击_712 = 712,
    // Bonus Concerto
    特技协奏曲_713 = 713,
    // Striking March
    // 进击进行曲 = 714,
    // Queried Waltz
    // 探索华尔兹 = 715,
    // Transform Belt
    变身腰带_716 = 716,
    // Neutraliser
    中和剂_717 = 717,
    // Greenifier
    速速绿_718 = 718,
    // Formula A
    很会长_719 = 719,
    // Formula B
    快速长_720 = 720,
    // Formula C
    极速长_721 = 721,
    // #N/A (Do not select)
    // 无效物品 #N/A = 722,
    // Fodder
    牧草_723 = 723,
    // Fur (S)
    软然毛_724 = 724,
    // Fur (M)
    软绒毛_725 = 725,
    // Fur (L)
    毛茸茸的软软毛_726 = 726,
    // Yarn
    毛线球S_727 = 727,
    // Yarn
    毛线球_728 = 728,
    // Yarn
    毛线球L_729 = 729,
    // Scrap Metal
    废铁_730 = 730,
    // Iron
    铁_731 = 731,
    // Bronze
    铜_732 = 732,
    // Silver
    银_733 = 733,
    // Gold
    金_734 = 734,
    // Platinum
    白金_735 = 735,
    // Diamond
    钻石_736 = 736,
    // Ruby
    红宝石_737 = 737,
    // Emerald
    绿宝石_738 = 738,
    // Sapphire
    蓝宝石_739 = 739,
    // Amethyst
    紫水晶_740 = 740,
    // Aquamarine
    海蓝宝石_741 = 741,
    // Cheap Cloth
    兽人的破布_742 = 742,
    // Quality Cloth
    海盗哥布林的布_743 = 743,
    // Silk Cloth
    丝绸_744 = 744,
    // Arrowhead
    生锈的箭头_745 = 745,
    // Warrior's Proof
    战士之证_746 = 746,
    // Glue
    胶_747 = 747,
    // Old Bandage
    哥布林的绷带_748 = 748,
    // Gunpowder
    射手的引火药_749 = 749,
    // Earth Crystal
    地之结晶_750 = 750,
    // Devil Horn
    魔人的角_751 = 751,
    // Devil Blood
    恶魔之血_752 = 752,
    // Magic Powder
    魔法师的粉末_753 = 753,
    // Magic Crystal
    魔力结晶_754 = 754,
    // Shoulder Piece
    维京肩甲_755 = 755,
    // Giant's Nail
    巨人之爪_756 = 756,
    // Giant's Gloves
    巨人的手套_757 = 757,
    // Hammer Piece
    锤子的碎片_758 = 758,
    // Insect Carapace
    虫皮_759 = 759,
    // Insect Mandibles
    昆虫的下颚_760 = 760,
    // Strong Thread
    强韧的蜘蛛丝_761 = 761,
    // Pretty Thread
    漂亮蜘蛛丝_762 = 762,
    // Pretty Carapace
    漂亮的虫皮_763 = 763,
    // Rigid Horn
    坚固的角_764 = 764,
    // Scorpion Tail
    蝎子的尾巴_765 = 765,
    // Scorpion Pincer
    蝎子的钳子_766 = 766,
    // Panther Claw
    豹爪_767 = 767,
    // Dragon Fang
    龙牙_768 = 768,
    // Wolf Fang
    狼牙_769 = 769,
    // Quality Fur
    优质毛皮_770 = 770,
    // Bull's Horn
    斗牛的角_771 = 771,
    // Bird's Feather
    鸟羽毛_772 = 772,
    // Wind Crystal
    风之结晶_773 = 773,
    // Fur
    毛皮_774 = 774,
    // Root
    植物的根_775 = 775,
    // Spore
    蘑菇孢子_776 = 776,
    // Poison Powder
    毒粉_777 = 777,
    // Plant Stem
    植物的茎_778 = 778,
    // Strong Vine
    结实的藤蔓_779 = 779,
    // Fish Fossil
    古代鱼的骨头_780 = 780,
    // Water Crystal
    水之结晶_781 = 781,
    // Turtle Shell
    龟壳_782 = 782,
    // Fire Crystal
    火之结晶_783 = 783,
    // Ghost Hood
    浮游灵的头巾_784 = 784,
    // Skull
    骷髅_785 = 785,
    // Broken Hilt
    坏掉的柄_786 = 786,
    // Broken Box
    坏掉的木箱_787 = 787,
    // Fairy Dust
    小仙子的粉末_788 = 788,
    // Small Crystal
    小结晶_789 = 789,
    // Wooly Furball
    摩可棉毛_790 = 790,
    // Light Crystal
    光之结晶_791 = 791,
    // Dark Crystal
    暗之结晶_792 = 792,
    // Love Crystal
    爱之结晶_793 = 793,
    // Tanuki Leaf
    狸猫的叶子_794 = 794,
    // Dragon Bones
    龙骨_795 = 795,
    // Red Lion Fur
    狮子的红毛_796 = 796,
    // Blue Lion Fur
    狮子的青毛_797 = 797,
    // Broken Ice Wall
    冰壁的碎片_798 = 798,
    // Ammonite
    菊石_799 = 799,
    // Icy Nose
    冰在鼻_800 = 800,
    // Chest Hair
    究极的胸毛_801 = 801,
    // Chimera Tail
    奇美拉的尾巴_802 = 802,
    // Grimoire Scale
    克里莫亚的鳞片_803 = 803,
    // Legendary Scale
    百鱼王鳞片_804 = 804,
    // Dragon Fin
    水龙羽鳍_805 = 805,
    // Moving Branch
    会动的树枝_806 = 806,
    // Electro Crystal
    电之结晶_807 = 807,
    // Melody Bottle
    会唱歌的小瓶子_808 = 808,
    // Golem Tablet
    石魔人的石板_809 = 809,
    // Earthwyrm Scale
    地龙鳞片_810 = 810,
    // Firewyrm Scale
    火龙鳞片_811 = 811,
    // Battle Turnip
    // 祈祷丰收的芜菁 = 812,
    // Gold Battle Turnip
    // 丰收的芜菁 = 813,
    // Bullet
    // 球 = 814,
    // Engagement Ring
    // 订婚戒指 = 815,
    // Letter
    // 信 = 816,
    // Round Stone
    // 圆石 = 817,
    // Rose Bouquet
    // 一百朵玫瑰的花束 = 818,
    // Shade Stone
    // 遮光石 = 819,
    // Strange Pendant
    // 奇妙坠饰 = 820,
    // Scrap Metal+
    // 惊人的废铁 = 821,
    // Sashimi Knife
    // 刺身刀 = 822,
    // Cactus Flower
    // 仙人掌花 = 823,
    // Foul Dragonbone
    // 腐朽的龙骨 = 824,
    // Ribbon
    // 缎带 = 825,
    // Brooch
    // 胸针 = 826,
    // Bouquet
    // 花束 = 827,
    // Marian's Potion
    // 玛丽安的瓶子 = 828,
    // Light Ore
    // 梅西莱特矿石 = 829,
    // Masu Trout
    石川鲑鱼_830 = 830,
    // Squid
    乌贼_831 = 831,
    // Taimen
    伊富鱼_832 = 832,
    // Sardine
    沙丁鱼_833 = 833,
    // Char
    红点鲑_834 = 834,
    // Chub
    珠星三块鱼_835 = 835,
    // Glitter Snapper
    闪耀鲷鱼_836 = 836,
    // Skipjack
    鲣鱼_837 = 837,
    // Turbot
    鲽鱼_838 = 838,
    // Gibelio
    鲋鱼_839 = 839,
    // Salmon
    钩吻鱼_840 = 840,
    // Mackerel
    青花鱼_841 = 841,
    // Needlefish
    针鱼_842 = 842,
    // Pike
    秋刀鱼_843 = 843,
    // Sunsquid
    斑乌贼_844 = 844,
    // Shrimp
    虾_845 = 845,
    // Snapper
    鲷鱼_846 = 846,
    // Throbby Snapper
    心动鲷鱼_847 = 847,
    // Rainbow Trout
    剧毒虹鳟_848 = 848,
    // Flounder
    比目鱼_849 = 849,
    // Blowfish
    河豚_850 = 850,
    // Yellowtail
    青甘鱼_851 = 851,
    // Crucian Carp
    高身鲫_852 = 852,
    // Tuna
    金枪鱼_853 = 853,
    // Girella
    黑毛鱼_854 = 854,
    // Autumn Flounder
    红叶比目鱼_855 = 855,
    // Cherry Salmon
    山女鳟_856 = 856,
    // Lamp Squid
    封印乌贼_857 = 857,
    // Lobster
    螯龙虾_858 = 858,
    // Pond Smelt
    西太公鱼_859 = 859,
    // Sand Flounder
    沙比目鱼_860 = 860,
    // Rockfish
    岩石鱼_861 = 861,
    // Can
    空罐_862 = 862,
    // Boot
    高筒靴_863 = 863,
    // Rare Can
    罕见的空罐_864 = 864,
    // Rock
    石头_865 = 865,
    // Branch
    树枝_866 = 866,
    // Lumber
    木材_867 = 867,
    // Sword Magic Seed
    剑草种子_868 = 868,
    // Sword Magic Seed-
    干瘪的剑草种子_869 = 869,
    // Iron Magic Seed
    钢身花种子_870 = 870,
    // Iron Magic Seed-
    柔软的的钢身花种子_871 = 871,
    // Wind Magic Seed
    追风草种子_872 = 872,
    // Wind Magic Seed-
    静止的追风草种子_873 = 873,
    // Jack Magic Seed
    杰克南瓜的种子_874 = 874,
    // Jack Magic Seed-
    废柴杰克的种子_875 = 875,
    // Bomb Magic Seed
    蜜瓜炸弹的种子_876 = 876,
    // Bomb Magic Seed-
    坏掉的蜜瓜种子_877 = 877,
    // Cactus Magic Seed
    土俑仙人掌种子_878 = 878,
    // Cactus Magic Seed-
    干枯的土俑仙人掌种子_879 = 879,
    // Water Magic Seed
    水生草的种子_880 = 880,
    // Water Magic Seed-
    干枯的水生草种子_881 = 881,
    // Lilypad Magic Seed
    莲叶船种子_882 = 882,
    // Lilypad Magic Seed-
    折断的莲花种子_883 = 883,
    // Banana Magic Seed
    滑倒香蕉种子_884 = 884,
    // Banana Magic Seed-
    腐烂的香蕉种子_885 = 885,
    // Bean Magic Seed
    魔法蚕豆_886 = 886,
    // Bean Magic Seed-
    别扭的蚕豆种子_887 = 887,
}

struct Trainer {
    显示界面: bool,

    选择的作物: Crop,
    作物列表: &'static [Crop],
    选择的作物等级: CropLevel,
    作物等级列表: &'static [CropLevel],
    选择的作物阶段: CropStage,
    作物阶段列表: &'static [CropStage],

    选择的秒: u8,
    秒列表: Vec<u8>,
    选择的时: u8,
    时列表: Vec<u8>,
    选择的天: u8,
    天列表: Vec<u8>,
    选择的季节: Season,
    季节列表: &'static [Season],
    选择的年: u8,
    年列表: Vec<u8>,
    选择的流速: TimeSpeed,
    时间流速列表: &'static [TimeSpeed],

    选择的物品: Item,
    物品列表: &'static [Item],
    选择的物品数量: u16,
    物品数量: Vec<u16>,
}

pub(crate) struct TrainerHook {
    pub(crate) 金币地址: *mut u32,
    pub(crate) 金币旧值: u32,
    pub(crate) 金币最大开关: bool,

    pub(crate) 木头地址: *mut u16,
    pub(crate) 木头旧值: u16,
    pub(crate) 木头最大开关: bool,

    pub(crate) 居民友谊倍率x100: AsmHook,

    pub(crate) 技能经验倍率x100: AsmHook,

    pub(crate) 战斗经验倍率x100: AsmHook,

    pub(crate) 战斗伤害倍率x100: AsmHook,

    pub(crate) 作物立即成熟: AsmHook,

    pub(crate) 自动钓鱼: AsmHook,
    pub(crate) 自动钓鱼按键: AsmHook,
    pub(crate) 自动钓鱼按键标签: u64,

    pub(crate) 角色穿墙: AsmHook,

    pub(crate) 无限委托: AsmHook,

    pub(crate) 魔物必定驯服: AsmHook,
    pub(crate) 禁止负面状态: AsmHook,

    pub(crate) 农田: AsmHook,

    pub(crate) 土壤质量开关: bool,
    pub(crate) 土壤质量标签: u64,

    pub(crate) 自动耕作开关: bool,
    pub(crate) 自动耕作标签: u64,

    pub(crate) 自动浇水开关: bool,
    pub(crate) 自动浇水标签: u64,

    pub(crate) 自动种植开关: bool,
    pub(crate) 自动种植标签: u64,

    pub(crate) 作物属性: crate::CropProperties,

    pub(crate) 时间: AsmHook,
    pub(crate) 时间指针: *mut Time,

    pub(crate) 物品: AsmHook,
    pub(crate) 物品指针: *mut Items,
}

#[repr(C)]
struct CropProperties {
    类型: u8,
    状态: CropStates,
}

#[repr(C)]
union CropStates {
    阶段: u8,
    等级: u8,
}

#[repr(C)]
struct Time {
    秒: u8,
    __: [u8; 3],
    时: u8,
    ___: [u8; 3],
    天: u8,
    ____: [u8; 3],
    季节: u8,
    _____: [u8; 3],
    年: u8,
    ______: [u8; 3],
    _______: [u8; 32],
    流速: u32,
}

#[repr(C)]
struct Items {
    item: u16,
}

impl ::core::fmt::Display for Season {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Spring => write!(f, "春"),
            Self::Summer => write!(f, "夏"),
            Self::Autumn => write!(f, "秋"),
            Self::Winter => write!(f, "冬"),
        }
    }
}

impl ::core::fmt::Display for TimeSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Puase => write!(f, "暂停"),
            Self::Percent1 => write!(f, "0.01倍"),
            Self::Percent10 => write!(f, "0.1倍"),
            Self::Percent25 => write!(f, "0.25倍"),
            Self::Percent50 => write!(f, "0.5倍"),
            Self::Normal => write!(f, "默认"),
            Self::Percent => write!(f, "1.5倍"),
            Self::Percent200 => write!(f, "2.0倍"),
        }
    }
}

impl ::core::fmt::Display for Crop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::无_0 => write!(f, "无_0"),
            Self::石头_1 => write!(f, "石头_1"),
            Self::岩石_2 => write!(f, "岩石_2"),
            Self::树枝_3 => write!(f, "树枝_3"),
            Self::树桩_4 => write!(f, "树桩_4"),
            Self::木材_5 => write!(f, "木材_5"),
            Self::毒沼_6 => write!(f, "毒沼_6"),
            Self::药草_8 => write!(f, "药草_8"),
            Self::解毒草_9 => write!(f, "解毒草_9"),
            Self::黑草_10 => write!(f, "黑草_10"),
            Self::枯草_11 => write!(f, "枯草_11"),
            Self::黄草_14 => write!(f, "黄草_14"),
            Self::苦橙草_15 => write!(f, "苦橙草_15"),
            Self::杂草_17 => write!(f, "杂草_17"),
            Self::季节岩_18 => write!(f, "季节岩_18"),
            Self::花卉_19 => write!(f, "花卉_19"),
            Self::水晶_20 => write!(f, "水晶_20"),
            Self::草莓_24 => write!(f, "草莓_24"),
            Self::卷心菜_25 => write!(f, "卷心菜_25"),
            Self::樱芜菁_26 => write!(f, "樱芜菁_26"),
            Self::洋葱_27 => write!(f, "洋葱_27"),
            Self::托伊药草_28 => write!(f, "托伊药草_28"),
            Self::月落草_29 => write!(f, "月落草_29"),
            Self::樱草_30 => write!(f, "樱草_30"),
            Self::灯草_31 => write!(f, "灯草_31"),
            Self::金刚花_32 => write!(f, "金刚花_32"),
            Self::青水晶_33 => write!(f, "青水晶_33"),
            Self::金卷心菜_34 => write!(f, "金卷心菜_34"),
            Self::少女蜜瓜_35 => write!(f, "少女蜜瓜_35"),
            Self::竹笋_36 => write!(f, "竹笋_36"),
            Self::南瓜_37 => write!(f, "南瓜_37"),
            Self::黄瓜_38 => write!(f, "黄瓜_38"),
            Self::玉米_39 => write!(f, "玉米_39"),
            Self::番茄_40 => write!(f, "番茄_40"),
            Self::茄子_41 => write!(f, "茄子_41"),
            Self::菠萝_42 => write!(f, "菠萝_42"),
            Self::粉红猫_43 => write!(f, "粉红猫_43"),
            Self::铁千轮_44 => write!(f, "铁千轮_44"),
            Self::四叶草_45 => write!(f, "四叶草_45"),
            Self::原之焰火_46 => write!(f, "原之焰火_46"),
            Self::绿水晶_47 => write!(f, "绿水晶_47"),
            Self::金南瓜_48 => write!(f, "金南瓜_48"),
            Self::蓝草_49 => write!(f, "蓝草_49"),
            Self::绿草_50 => write!(f, "绿草_50"),
            Self::紫草_51 => write!(f, "紫草_51"),
            Self::靛草_52 => write!(f, "靛草_52"),
            Self::番薯_53 => write!(f, "番薯_53"),
            Self::马铃薯_54 => write!(f, "马铃薯_54"),
            Self::胡萝卜_55 => write!(f, "胡萝卜_55"),
            Self::青椒_56 => write!(f, "青椒_56"),
            Self::菠菜_57 => write!(f, "菠菜_57"),
            Self::魅蓝草_58 => write!(f, "魅蓝草_58"),
            Self::红叶花_59 => write!(f, "红叶花_59"),
            Self::剧毒蒲公英_60 => write!(f, "剧毒蒲公英_60"),
            Self::红水晶_61 => write!(f, "红水晶_61"),
            Self::金马铃薯_62 => write!(f, "金马铃薯_62"),
            Self::芜菁_63 => write!(f, "芜菁_63"),
            Self::白萝卜_64 => write!(f, "白萝卜_64"),
            Self::葱_65 => write!(f, "葱_65"),
            Self::白菜_66 => write!(f, "白菜_66"),
            Self::树形草_67 => write!(f, "树形草_67"),
            Self::白水晶_68 => write!(f, "白水晶_68"),
            Self::金芜青_69 => write!(f, "金芜青_69"),
            Self::火热果实_70 => write!(f, "火热果实_70"),
            Self::白草_71 => write!(f, "白草_71"),
        }
    }
}

impl ::core::fmt::Display for CropLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LV1 => write!(f, "LV1"),
            Self::LV2 => write!(f, "LV2"),
            Self::LV3 => write!(f, "LV3"),
            Self::LV4 => write!(f, "LV4"),
            Self::LV5 => write!(f, "LV5"),
            Self::LV6 => write!(f, "LV6"),
            Self::LV7 => write!(f, "LV7"),
            Self::LV8 => write!(f, "LV8"),
            Self::LV9 => write!(f, "LV9"),
            Self::LV10 => write!(f, "LV10"),
        }
    }
}

impl ::core::fmt::Display for CropStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::一阶段 => write!(f, "一阶段"),
            Self::二阶段 => write!(f, "二阶段"),
            Self::三阶段 => write!(f, "三阶段"),
            Self::四阶段 => write!(f, "四阶段"),
            Self::五阶段 => write!(f, "五阶段"),
        }
    }
}

impl ::core::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::卷心菜_1 => write!(f, "卷心菜_1"),
            Self::樱芜菁_2 => write!(f, "樱芜菁_2"),
            Self::少女蜜瓜_3 => write!(f, "少女蜜瓜_3"),
            Self::洋葱_4 => write!(f, "洋葱_4"),
            Self::南瓜_5 => write!(f, "南瓜_5"),
            Self::黄瓜_6 => write!(f, "黄瓜_6"),
            Self::玉米_7 => write!(f, "玉米_7"),
            Self::番茄_8 => write!(f, "番茄_8"),
            Self::茄子_9 => write!(f, "茄子_9"),
            Self::番薯_10 => write!(f, "番薯_10"),
            Self::马铃薯_11 => write!(f, "马铃薯_11"),
            Self::胡萝卜_12 => write!(f, "胡萝卜_12"),
            Self::青椒_13 => write!(f, "青椒_13"),
            Self::菠菜_14 => write!(f, "菠菜_14"),
            Self::芜菁_15 => write!(f, "芜菁_15"),
            Self::白萝卜_16 => write!(f, "白萝卜_16"),
            Self::葱_17 => write!(f, "葱_17"),
            Self::白菜_18 => write!(f, "白菜_18"),
            Self::火热果实_19 => write!(f, "火热果实_19"),
            Self::竹笋_20 => write!(f, "竹笋_20"),
            Self::金卷心菜_21 => write!(f, "金卷心菜_21"),
            Self::金南瓜_22 => write!(f, "金南瓜_22"),
            Self::金马铃薯_23 => write!(f, "金马铃薯_23"),
            Self::金芜菁_24 => write!(f, "金芜菁_24"),
            Self::蘑菇A_25 => write!(f, "蘑菇A_25"),
            Self::蘑菇B_26 => write!(f, "蘑菇B_26"),
            Self::蘑菇C_27 => write!(f, "蘑菇C_27"),
            Self::蘑菇D_28 => write!(f, "蘑菇D_28"),
            Self::蘑菇E_29 => write!(f, "蘑菇E_29"),
            Self::蘑菇F_30 => write!(f, "蘑菇F_30"),
            Self::草莓_31 => write!(f, "草莓_31"),
            Self::菠萝_32 => write!(f, "菠萝_32"),
            Self::葡萄_33 => write!(f, "葡萄_33"),
            Self::苹果_34 => write!(f, "苹果_34"),
            Self::橙子_35 => write!(f, "橙子_35"),
            Self::托伊药草_36 => write!(f, "托伊药草_36"),
            Self::月落花_37 => write!(f, "月落花_37"),
            Self::粉红猫_38 => write!(f, "粉红猫_38"),
            Self::魅蓝草_39 => write!(f, "魅蓝草_39"),
            Self::药草_40 => write!(f, "药草_40"),
            Self::解毒草_41 => write!(f, "解毒草_41"),
            Self::黑草_42 => write!(f, "黑草_42"),
            Self::苦橙草_43 => write!(f, "苦橙草_43"),
            Self::红草_44 => write!(f, "红草_44"),
            Self::黄草_45 => write!(f, "黄草_45"),
            Self::蓝草_46 => write!(f, "蓝草_46"),
            Self::绿草_47 => write!(f, "绿草_47"),
            Self::紫草_48 => write!(f, "紫草_48"),
            Self::靛草_49 => write!(f, "靛草_49"),
            Self::白草_50 => write!(f, "白草_50"),
            Self::杂草_51 => write!(f, "杂草_51"),
            Self::枯草_52 => write!(f, "枯草_52"),
            Self::樱草_53 => write!(f, "樱草_53"),
            Self::灯草_54 => write!(f, "灯草_54"),
            Self::青水花_55 => write!(f, "青水花_55"),
            Self::金刚花_56 => write!(f, "金刚花_56"),
            Self::铁千轮_57 => write!(f, "铁千轮_57"),
            Self::四叶草_58 => write!(f, "四叶草_58"),
            Self::原之焰火_59 => write!(f, "原之焰火_59"),
            Self::绿水晶_60 => write!(f, "绿水晶_60"),
            Self::树形草_61 => write!(f, "树形草_61"),
            Self::红叶花_62 => write!(f, "红叶花_62"),
            Self::剧毒蒲公英_63 => write!(f, "剧毒蒲公英_63"),
            Self::红水晶_64 => write!(f, "红水晶_64"),
            Self::白水晶_65 => write!(f, "白水晶_65"),
            Self::蛋白质_66 => write!(f, "蛋白质_66"),
            Self::智力补品_67 => write!(f, "智力补品_67"),
            Self::生命果冻_68 => write!(f, "生命果冻_68"),
            Self::心灵饮料_69 => write!(f, "心灵饮料_69"),
            Self::解毒药_70 => write!(f, "解毒药_70"),
            Self::解麻药_71 => write!(f, "解麻药_71"),
            Self::解封药_72 => write!(f, "解封药_72"),
            Self::感冒药_73 => write!(f, "感冒药_73"),
            Self::超级营养药_74 => write!(f, "超级营养药_74"),
            Self::爱情饮料_75 => write!(f, "爱情饮料_75"),
            Self::无敌秘药_76 => write!(f, "无敌秘药_76"),
            Self::符文药水_77 => write!(f, "符文药水_77"),
            Self::地狱辣椒_78 => write!(f, "地狱辣椒_78"),
            Self::甘甜粉_79 => write!(f, "甘甜粉_79"),
            Self::酸味饮料_80 => write!(f, "酸味饮料_80"),
            Self::混合香料_81 => write!(f, "混合香料_81"),
            Self::失败之作_82 => write!(f, "失败之作_82"),
            Self::超级失败之作_83 => write!(f, "超级失败之作_83"),
            Self::石川鲑鱼生鱼片_84 => write!(f, "石川鲑鱼生鱼片_84"),
            Self::红点鲑生鱼片_85 => write!(f, "红点鲑生鱼片_85"),
            Self::剧毒虹鳟生鱼片_86 => write!(f, "剧毒虹鳟生鱼片_86"),
            Self::钩吻鲑生鱼片_87 => write!(f, "钩吻鲑生鱼片_87"),
            Self::山女鳟生鱼片_88 => write!(f, "山女鳟生鱼片_88"),
            Self::伊富鱼生鱼片_89 => write!(f, "伊富鱼生鱼片_89"),
            Self::鲷鱼生鱼片_90 => write!(f, "鲷鱼生鱼片_90"),
            Self::闪光鲷鱼生鱼片_91 => write!(f, "闪光鲷鱼生鱼片_91"),
            Self::心动鲷生鱼片_92 => write!(f, "心动鲷生鱼片_92"),
            Self::黑毛鱼生鱼片_93 => write!(f, "黑毛鱼生鱼片_93"),
            Self::鲣鱼生鱼片_94 => write!(f, "鲣鱼生鱼片_94"),
            Self::青甘鱼生鱼片_95 => write!(f, "青甘鱼生鱼片_95"),
            Self::金枪鱼大脂_96 => write!(f, "金枪鱼大脂_96"),
            Self::沙丁鱼生鱼片_97 => write!(f, "沙丁鱼生鱼片_97"),
            Self::针鱼生鱼片_98 => write!(f, "针鱼生鱼片_98"),
            Self::秋刀鱼生鱼片_99 => write!(f, "秋刀鱼生鱼片_99"),
            Self::比目鱼生鱼片_100 => write!(f, "比目鱼生鱼片_100"),
            Self::鲽鱼鲆生鱼片_101 => write!(f, "鲽鱼鲆生鱼片_101"),
            Self::红叶鱼生鱼片_102 => write!(f, "红叶鱼生鱼片_102"),
            Self::乌贼刺身_103 => write!(f, "乌贼刺身_103"),
            Self::斑乌贼刺身_104 => write!(f, "斑乌贼刺身_104"),
            Self::封印乌贼刺身_105 => write!(f, "封印乌贼刺身_105"),
            Self::河豚生鱼片_106 => write!(f, "河豚生鱼片_106"),
            Self::螯龙虾刺身_107 => write!(f, "螯龙虾刺身_107"),
            Self::虾刺身_108 => write!(f, "虾刺身_108"),
            Self::炒蔬菜_109 => write!(f, "炒蔬菜_109"),
            Self::炒饭_110 => write!(f, "炒饭_110"),
            Self::什锦烧_111 => write!(f, "什锦烧_111"),
            Self::炸薯条_112 => write!(f, "炸薯条_112"),
            Self::可乐饼_113 => write!(f, "可乐饼_113"),
            Self::爆米花_114 => write!(f, "爆米花_114"),
            Self::玉米片_115 => write!(f, "玉米片_115"),
            Self::茄子田乐烧_116 => write!(f, "茄子田乐烧_116"),
            Self::玉子烧_117 => write!(f, "玉子烧_117"),
            Self::欧姆蛋_118 => write!(f, "欧姆蛋_118"),
            Self::蛋包饭_119 => write!(f, "蛋包饭_119"),
            Self::烤苹果_120 => write!(f, "烤苹果_120"),
            Self::咖喱面包_121 => write!(f, "咖喱面包_121"),
            Self::法式吐司_122 => write!(f, "法式吐司_122"),
            Self::甜甜圈_123 => write!(f, "甜甜圈_123"),
            Self::炒乌冬面_124 => write!(f, "炒乌冬面_124"),
            Self::天妇罗_125 => write!(f, "天妇罗_125"),
            Self::美式松饼_126 => write!(f, "美式松饼_126"),
            Self::煎饺_127 => write!(f, "煎饺_127"),
            Self::烩饭_128 => write!(f, "烩饭_128"),
            Self::干咖喱料理_129 => write!(f, "干咖喱料理_129"),
            Self::盐烤石川鲑鱼_130 => write!(f, "盐烤石川鲑鱼_130"),
            Self::盐烤红点鲑_131 => write!(f, "盐烤红点鲑_131"),
            Self::烤剧毒虹鳟_132 => write!(f, "烤剧毒虹鳟_132"),
            Self::盐烤山女鳟_133 => write!(f, "盐烤山女鳟_133"),
            Self::盐烤珠星三块鱼_134 => write!(f, "盐烤珠星三块鱼_134"),
            Self::盐烤鲑鱼_135 => write!(f, "盐烤鲑鱼_135"),
            Self::伊富鱼烤全鱼_136 => write!(f, "伊富鱼烤全鱼_136"),
            Self::高身鲫烤全鱼_137 => write!(f, "高身鲫烤全鱼_137"),
            Self::鲋鱼烤全鱼_138 => write!(f, "鲋鱼烤全鱼_138"),
            Self::烤鲷鱼料理_139 => write!(f, "烤鲷鱼料理_139"),
            Self::盐烤黑毛鱼_140 => write!(f, "盐烤黑毛鱼_140"),
            Self::烤闪光鲷鱼_141 => write!(f, "烤闪光鲷鱼_141"),
            Self::烤心动鲷鱼_142 => write!(f, "烤心动鲷鱼_142"),
            Self::土佐风烤鲣鱼_143 => write!(f, "土佐风烤鲣鱼_143"),
            Self::烤青花鱼_144 => write!(f, "烤青花鱼_144"),
            Self::照烧青甘鱼_145 => write!(f, "照烧青甘鱼_145"),
            Self::盐烤西太公鱼_146 => write!(f, "盐烤西太公鱼_146"),
            Self::照烧金枪鱼_147 => write!(f, "照烧金枪鱼_147"),
            Self::沙丁鱼串_148 => write!(f, "沙丁鱼串_148"),
            Self::盐烤针鱼_149 => write!(f, "盐烤针鱼_149"),
            Self::盐烤秋刀鱼_150 => write!(f, "盐烤秋刀鱼_150"),
            Self::烤比目鱼_151 => write!(f, "烤比目鱼_151"),
            Self::烤鲽鱼_152 => write!(f, "烤鲽鱼_152"),
            Self::烤红叶比目鱼_153 => write!(f, "烤红叶比目鱼_153"),
            Self::烤乌贼_154 => write!(f, "烤乌贼_154"),
            Self::烤斑乌贼_155 => write!(f, "烤斑乌贼_155"),
            Self::烤封印乌贼_156 => write!(f, "烤封印乌贼_156"),
            Self::烤河豚_157 => write!(f, "烤河豚_157"),
            Self::螯龙虾烤全虾_158 => write!(f, "螯龙虾烤全虾_158"),
            Self::烤全虾_159 => write!(f, "烤全虾_159"),
            Self::烤沙比目鱼_160 => write!(f, "烤沙比目鱼_160"),
            Self::热牛奶_161 => write!(f, "热牛奶_161"),
            Self::热可可_162 => write!(f, "热可可_162"),
            Self::葡萄酒_163 => write!(f, "葡萄酒_163"),
            Self::炖南瓜_164 => write!(f, "炖南瓜_164"),
            Self::凉拌菠菜_165 => write!(f, "凉拌菠菜_165"),
            Self::水煮蛋_166 => write!(f, "水煮蛋_166"),
            Self::拔丝地瓜_167 => write!(f, "拔丝地瓜_167"),
            Self::水饺_168 => write!(f, "水饺_168"),
            Self::草莓酱_169 => write!(f, "草莓酱_169"),
            Self::苹果酱_170 => write!(f, "苹果酱_170"),
            Self::葡萄酱_171 => write!(f, "葡萄酱_171"),
            Self::柑橘酱_172 => write!(f, "柑橘酱_172"),
            Self::奶酪火锅_173 => write!(f, "奶酪火锅_173"),
            Self::乌冬面_174 => write!(f, "乌冬面_174"),
            Self::咖喱乌冬面_175 => write!(f, "咖喱乌冬面_175"),
            Self::天妇罗乌冬面_176 => write!(f, "天妇罗乌冬面_176"),
            Self::粥_177 => write!(f, "粥_177"),
            Self::牛奶粥_178 => write!(f, "牛奶粥_178"),
            Self::天妇罗盖饭_179 => write!(f, "天妇罗盖饭_179"),
            Self::鸡蛋盖饭_180 => write!(f, "鸡蛋盖饭_180"),
            Self::炖菜_181 => write!(f, "炖菜_181"),
            Self::咖喱饭_182 => write!(f, "咖喱饭_182"),
            Self::究极咖喱_183 => write!(f, "究极咖喱_183"),
            Self::至尊咖喱_184 => write!(f, "至尊咖喱_184"),
            Self::放松茶_185 => write!(f, "放松茶_185"),
            Self::味增田乐烧_186 => write!(f, "味增田乐烧_186"),
            Self::什锦火锅_187 => write!(f, "什锦火锅_187"),
            Self::炖煮岩石鱼_188 => write!(f, "炖煮岩石鱼_188"),
            Self::烤玉米_189 => write!(f, "烤玉米_189"),
            Self::烤饭团_190 => write!(f, "烤饭团_190"),
            Self::烤番薯_191 => write!(f, "烤番薯_191"),
            Self::吐司_192 => write!(f, "吐司_192"),
            Self::果酱面包_193 => write!(f, "果酱面包_193"),
            Self::黄油面包卷_194 => write!(f, "黄油面包卷_194"),
            Self::披萨_195 => write!(f, "披萨_195"),
            Self::海鲜披萨_196 => write!(f, "海鲜披萨_196"),
            Self::多利亚饭_197 => write!(f, "多利亚饭_197"),
            Self::海鲜多利亚饭_198 => write!(f, "海鲜多利亚饭_198"),
            Self::奶油烤菜_199 => write!(f, "奶油烤菜_199"),
            Self::海鲜奶油烤菜_200 => write!(f, "海鲜奶油烤菜_200"),
            Self::蜜番薯_201 => write!(f, "蜜番薯_201"),
            Self::曲奇饼干_202 => write!(f, "曲奇饼干_202"),
            Self::巧克力曲奇_203 => write!(f, "巧克力曲奇_203"),
            Self::蛋糕_204 => write!(f, "蛋糕_204"),
            Self::巧克力蛋糕_205 => write!(f, "巧克力蛋糕_205"),
            Self::奶酪蛋糕_206 => write!(f, "奶酪蛋糕_206"),
            Self::苹果派_207 => write!(f, "苹果派_207"),
            Self::菠萝汁_208 => write!(f, "菠萝汁_208"),
            Self::番茄汁_209 => write!(f, "番茄汁_209"),
            Self::葡萄汁_210 => write!(f, "葡萄汁_210"),
            Self::橘子汁_211 => write!(f, "橘子汁_211"),
            Self::苹果汁_212 => write!(f, "苹果汁_212"),
            Self::草莓牛奶_213 => write!(f, "草莓牛奶_213"),
            Self::复合果汁_214 => write!(f, "复合果汁_214"),
            Self::水果欧蕾_215 => write!(f, "水果欧蕾_215"),
            Self::蔬菜汁_216 => write!(f, "蔬菜汁_216"),
            Self::蔬菜欧蕾_217 => write!(f, "蔬菜欧蕾_217"),
            Self::混合蔬果汁_218 => write!(f, "混合蔬果汁_218"),
            Self::混合蔬果欧蕾_219 => write!(f, "混合蔬果欧蕾_219"),
            Self::番茄酱_220 => write!(f, "番茄酱_220"),
            Self::黄油_221 => write!(f, "黄油_221"),
            Self::黄金果汁_222 => write!(f, "黄金果汁_222"),
            Self::恋爱的预感_223 => write!(f, "恋爱的预感_223"),
            Self::热果汁_224 => write!(f, "热果汁_224"),
            Self::蒸面包_225 => write!(f, "蒸面包_225"),
            Self::奶酪蒸面包_226 => write!(f, "奶酪蒸面包_226"),
            Self::烧麦_227 => write!(f, "烧麦_227"),
            Self::包子_228 => write!(f, "包子_228"),
            Self::咖喱包子_229 => write!(f, "咖喱包子_229"),
            Self::蒸饺_230 => write!(f, "蒸饺_230"),
            Self::蒸卡斯提拉_231 => write!(f, "蒸卡斯提拉_231"),
            Self::蒸蛋糕_232 => write!(f, "蒸蛋糕_232"),
            Self::布丁_233 => write!(f, "布丁_233"),
            Self::南瓜布丁_234 => write!(f, "南瓜布丁_234"),
            Self::团子_235 => write!(f, "团子_235"),
            Self::色拉_236 => write!(f, "色拉_236"),
            Self::三明治_237 => write!(f, "三明治_237"),
            Self::水果三明治_238 => write!(f, "水果三明治_238"),
            Self::腌芜菁_239 => write!(f, "腌芜菁_239"),
            Self::腌黄瓜_240 => write!(f, "腌黄瓜_240"),
            Self::竹笋饭_241 => write!(f, "竹笋饭_241"),
            Self::葡萄干面包_242 => write!(f, "葡萄干面包_242"),
            Self::冰淇淋_243 => write!(f, "冰淇淋_243"),
            Self::放松茶的茶叶_244 => write!(f, "放松茶的茶叶_244"),
            Self::饭团_245 => write!(f, "饭团_245"),
            Self::面包_246 => write!(f, "面包_246"),
            Self::鲑鱼饭团_247 => write!(f, "鲑鱼饭团_247"),
            Self::腌红白芜菁_248 => write!(f, "腌红白芜菁_248"),
            Self::芜菁天堂_249 => write!(f, "芜菁天堂_249"),
            Self::米饭_250 => write!(f, "米饭_250"),
            Self::巧克力_251 => write!(f, "巧克力_251"),
            Self::红酒_252 => write!(f, "红酒_252"),
            Self::艾丽草_253 => write!(f, "艾丽草_253"),
            Self::牛奶_254 => write!(f, "牛奶_254"),
            Self::优质牛奶_255 => write!(f, "优质牛奶_255"),
            Self::钙质满分牛奶_256 => write!(f, "钙质满分牛奶_256"),
            Self::鸡蛋_257 => write!(f, "鸡蛋_257"),
            Self::优质鸡蛋_258 => write!(f, "优质鸡蛋_258"),
            Self::每日健康蛋_259 => write!(f, "每日健康蛋_259"),
            Self::蛋黄酱S_260 => write!(f, "蛋黄酱S_260"),
            Self::蛋黄酱_261 => write!(f, "蛋黄酱_261"),
            Self::蛋黄酱L_262 => write!(f, "蛋黄酱L_262"),
            Self::奶酪S_263 => write!(f, "奶酪S_263"),
            Self::奶酪_264 => write!(f, "奶酪_264"),
            Self::奶酪L_265 => write!(f, "奶酪L_265"),
            Self::酸奶S_266 => write!(f, "酸奶S_266"),
            Self::酸奶_267 => write!(f, "酸奶_267"),
            Self::酸奶L_268 => write!(f, "酸奶L_268"),
            Self::蜂蜜_269 => write!(f, "蜂蜜_269"),
            Self::小麦粉_270 => write!(f, "小麦粉_270"),
            Self::油_271 => write!(f, "油_271"),
            Self::咖喱粉_272 => write!(f, "咖喱粉_272"),
            Self::团子粉_273 => write!(f, "团子粉_273"),
            Self::药学低级配方面包_274 => write!(f, "药学低级配方面包_274"),
            Self::药学高级配方面包_275 => write!(f, "药学高级配方面包_275"),
            Self::料理低级配方面包_276 => write!(f, "料理低级配方面包_276"),
            Self::料理高级配方面包_277 => write!(f, "料理高级配方面包_277"),
            Self::武器低级配方面包_278 => write!(f, "武器低级配方面包_278"),
            Self::武器高级配方面包_279 => write!(f, "武器高级配方面包_279"),
            Self::装饰低级配方面包_280 => write!(f, "装饰低级配方面包_280"),
            Self::装饰高级配方面包_281 => write!(f, "装饰高级配方面包_281"),
            Self::农具低级配方面包_282 => write!(f, "农具低级配方面包_282"),
            Self::农具高级配方面包_283 => write!(f, "农具高级配方面包_283"),
            Self::阔剑_284 => write!(f, "阔剑_284"),
            Self::矿石剑_285 => write!(f, "矿石剑_285"),
            Self::风之剑_286 => write!(f, "风之剑_286"),
            Self::水星剑_287 => write!(f, "水星剑_287"),
            Self::防卫者_288 => write!(f, "防卫者_288"),
            Self::大气锋刃_289 => write!(f, "大气锋刃_289"),
            Self::烈焰剑_290 => write!(f, "烈焰剑_290"),
            Self::红樱_291 => write!(f, "红樱_291"),
            Self::幸运剑_292 => write!(f, "幸运剑_292"),
            Self::白金剑_293 => write!(f, "白金剑_293"),
            Self::胜利之剑_294 => write!(f, "胜利之剑_294"),
            Self::死亡之剑_295 => write!(f, "死亡之剑_295"),
            Self::噬魂剑_296 => write!(f, "噬魂剑_296"),
            Self::粉碎之剑_297 => write!(f, "粉碎之剑_297"),
            Self::屠龙剑_298 => write!(f, "屠龙剑_298"),
            Self::太阳之剑_299 => write!(f, "太阳之剑_299"),
            Self::星辰军刀_300 => write!(f, "星辰军刀_300"),
            Self::盖亚之剑_301 => write!(f, "盖亚之剑_301"),
            Self::格兰泰尔_302 => write!(f, "格兰泰尔_302"),
            Self::混沌剑_303 => write!(f, "混沌剑_303"),
            Self::符文剑_304 => write!(f, "符文剑_304"),
            Self::矿石剑改_305 => write!(f, "矿石剑改_305"),
            Self::白金剑改_306 => write!(f, "白金剑改_306"),
            Self::弯刀_307 => write!(f, "弯刀_307"),
            Self::痒痒挠_308 => write!(f, "痒痒挠_308"),
            Self::迪朗达尔_309 => write!(f, "迪朗达尔_309"),
            Self::古拉迪乌斯_310 => write!(f, "古拉迪乌斯_310"),
            Self::奢华之剑_311 => write!(f, "奢华之剑_311"),
            Self::勺子_312 => write!(f, "勺子_312"),
            Self::符文传说_313 => write!(f, "符文传说_313"),
            Self::蛇腹剑_314 => write!(f, "蛇腹剑_314"),
            Self::蔬菜剑_315 => write!(f, "蔬菜剑_315"),
            Self::隐形剑_316 => write!(f, "隐形剑_316"),
            Self::大剑_317 => write!(f, "大剑_317"),
            Self::双手剑_318 => write!(f, "双手剑_318"),
            Self::烈焰大剑_319 => write!(f, "烈焰大剑_319"),
            Self::气旋剑_320 => write!(f, "气旋剑_320"),
            Self::斩斩舞_321 => write!(f, "斩斩舞_321"),
            Self::巨剑_322 => write!(f, "巨剑_322"),
            Self::天之村云之剑_323 => write!(f, "天之村云之剑_323"),
            Self::大地粉碎者_324 => write!(f, "大地粉碎者_324"),
            Self::苍眼太刀_325 => write!(f, "苍眼太刀_325"),
            Self::毒素剑_326 => write!(f, "毒素剑_326"),
            Self::名刀达忍_327 => write!(f, "名刀达忍_327"),
            Self::焰形剑_328 => write!(f, "焰形剑_328"),
            Self::闪亮剑_329 => write!(f, "闪亮剑_329"),
            Self::大地之影_330 => write!(f, "大地之影_330"),
            Self::生物粉碎者_331 => write!(f, "生物粉碎者_331"),
            Self::制裁者_332 => write!(f, "制裁者_332"),
            Self::海洋切割者_333 => write!(f, "海洋切割者_333"),
            Self::波尔卡农_334 => write!(f, "波尔卡农_334"),
            Self::白雪皇冠_335 => write!(f, "白雪皇冠_335"),
            Self::月影_336 => write!(f, "月影_336"),
            Self::四元素剑_337 => write!(f, "四元素剑_337"),
            Self::双手剑改_338 => write!(f, "双手剑改_338"),
            Self::焰形剑改_339 => write!(f, "焰形剑改_339"),
            Self::斗剑_340 => write!(f, "斗剑_340"),
            Self::巨大匕首_341 => write!(f, "巨大匕首_341"),
            Self::太刀_342 => write!(f, "太刀_342"),
            Self::巴尔蒙克_343 => write!(f, "巴尔蒙克_343"),
            Self::狂乱刃_344 => write!(f, "狂乱刃_344"),
            Self::大鱼_345 => write!(f, "大鱼_345"),
            Self::勇敢之心_346 => write!(f, "勇敢之心_346"),
            Self::别西卜_347 => write!(f, "别西卜_347"),
            Self::白萝卜剑_348 => write!(f, "白萝卜剑_348"),
            Self::王者之剑_349 => write!(f, "王者之剑_349"),
            Self::八岐大蛇_350 => write!(f, "八岐大蛇_350"),
            Self::长枪_351 => write!(f, "长枪_351"),
            Self::骑士枪_352 => write!(f, "骑士枪_352"),
            Self::针枪_353 => write!(f, "针枪_353"),
            Self::斧枪_354 => write!(f, "斧枪_354"),
            Self::水之枪_355 => write!(f, "水之枪_355"),
            Self::血枪_356 => write!(f, "血枪_356"),
            Self::棍棒_357 => write!(f, "棍棒_357"),
            Self::毒枪_358 => write!(f, "毒枪_358"),
            Self::科西嘉枪_359 => write!(f, "科西嘉枪_359"),
            Self::沉默戟刀_360 => write!(f, "沉默戟刀_360"),
            Self::烈焰长枪_361 => write!(f, "烈焰长枪_361"),
            Self::重枪_362 => write!(f, "重枪_362"),
            Self::冰山枪_363 => write!(f, "冰山枪_363"),
            Self::恐惧_364 => write!(f, "恐惧_364"),
            Self::金箍棒_365 => write!(f, "金箍棒_365"),
            Self::骤变者_366 => write!(f, "骤变者_366"),
            Self::贯魔神枪_367 => write!(f, "贯魔神枪_367"),
            Self::羽毛枪_368 => write!(f, "羽毛枪_368"),
            Self::贝鲁伐洛斯_369 => write!(f, "贝鲁伐洛斯_369"),
            Self::剧毒枪_370 => write!(f, "剧毒枪_370"),
            Self::永恒之枪_371 => write!(f, "永恒之枪_371"),
            Self::骑士枪改_372 => write!(f, "骑士枪改_372"),
            Self::科西嘉枪改_373 => write!(f, "科西嘉枪改_373"),
            Self::三叉戟_374 => write!(f, "三叉戟_374"),
            Self::草叉_375 => write!(f, "草叉_375"),
            Self::龙之牙_376 => write!(f, "龙之牙_376"),
            Self::盖伯尔加之枪_377 => write!(f, "盖伯尔加之枪_377"),
            Self::魔法枪_378 => write!(f, "魔法枪_378"),
            Self::感觉不怎么痛的枪_379 => write!(f, "感觉不怎么痛的枪_379"),
            Self::灵枪_380 => write!(f, "灵枪_380"),
            Self::毒后_381 => write!(f, "毒后_381"),
            Self::菠萝棒_382 => write!(f, "菠萝棒_382"),
            Self::五节棍_383 => write!(f, "五节棍_383"),
            Self::破锤子_384 => write!(f, "破锤子_384"),
            Self::青铜锤_385 => write!(f, "青铜锤_385"),
            Self::银锤_386 => write!(f, "银锤_386"),
            Self::金锤_387 => write!(f, "金锤_387"),
            Self::白金锤_388 => write!(f, "白金锤_388"),
            Self::战斗锤_389 => write!(f, "战斗锤_389"),
            Self::战锤_390 => write!(f, "战锤_390"),
            Self::大锤_391 => write!(f, "大锤_391"),
            Self::施纳贝尔_392 => write!(f, "施纳贝尔_392"),
            Self::巨锤_393 => write!(f, "巨锤_393"),
            Self::雷神之锤_394 => write!(f, "雷神之锤_394"),
            Self::钉锤_395 => write!(f, "钉锤_395"),
            Self::烈焰锤_396 => write!(f, "烈焰锤_396"),
            Self::寒冰锤_397 => write!(f, "寒冰锤_397"),
            Self::天空锤_398 => write!(f, "天空锤_398"),
            Self::大地锤_399 => write!(f, "大地锤_399"),
            Self::骨锤_400 => write!(f, "骨锤_400"),
            Self::水晶锤_401 => write!(f, "水晶锤_401"),
            Self::战锤改_402 => write!(f, "战锤改_402"),
            Self::巨锤改_403 => write!(f, "巨锤改_403"),
            Self::觉醒的破锤子_404 => write!(f, "觉醒的破锤子_404"),
            Self::哔哔锤_405 => write!(f, "哔哔锤_405"),
            Self::致命冲击_406 => write!(f, "致命冲击_406"),
            Self::试力石_407 => write!(f, "试力石_407"),
            Self::金刚_408 => write!(f, "金刚_408"),
            Self::球棒_409 => write!(f, "球棒_409"),
            Self::金属球棒_410 => write!(f, "金属球棒_410"),
            Self::飞溅之星_411 => write!(f, "飞溅之星_411"),
            Self::破斧_412 => write!(f, "破斧_412"),
            Self::砍柴用斧_413 => write!(f, "砍柴用斧_413"),
            Self::砍伐用斧_414 => write!(f, "砍伐用斧_414"),
            Self::巡山用斧_415 => write!(f, "巡山用斧_415"),
            Self::奇迹斧_416 => write!(f, "奇迹斧_416"),
            Self::战斧_417 => write!(f, "战斧_417"),
            Self::枪斧_418 => write!(f, "枪斧_418"),
            Self::欧鲁帝鲁_419 => write!(f, "欧鲁帝鲁_419"),
            Self::巨斧_420 => write!(f, "巨斧_420"),
            Self::恶魔斧_421 => write!(f, "恶魔斧_421"),
            Self::偃月斧_422 => write!(f, "偃月斧_422"),
            Self::行刑者_423 => write!(f, "行刑者_423"),
            Self::高热斧_424 => write!(f, "高热斧_424"),
            Self::冰霜斧_425 => write!(f, "冰霜斧_425"),
            Self::投掷战斧_426 => write!(f, "投掷战斧_426"),
            Self::洛克斧_427 => write!(f, "洛克斧_427"),
            Self::双刃斧_428 => write!(f, "双刃斧_428"),
            Self::圣斧_429 => write!(f, "圣斧_429"),
            Self::枪斧改_430 => write!(f, "枪斧改_430"),
            Self::偃月斧改_431 => write!(f, "偃月斧改_431"),
            Self::认真起来的破斧头_432 => write!(f, "认真起来的破斧头_432"),
            Self::棒棒糖_433 => write!(f, "棒棒糖_433"),
            Self::战斗镰刀_434 => write!(f, "战斗镰刀_434"),
            Self::翼蜥之牙_435 => write!(f, "翼蜥之牙_435"),
            Self::恶魔之爪_436 => write!(f, "恶魔之爪_436"),
            Self::破镰刀_437 => write!(f, "破镰刀_437"),
            Self::铁镰刀_438 => write!(f, "铁镰刀_438"),
            Self::优质镰刀_439 => write!(f, "优质镰刀_439"),
            Self::利刃镰刀_440 => write!(f, "利刃镰刀_440"),
            Self::名匠镰鼬_441 => write!(f, "名匠镰鼬_441"),
            Self::破洒水壶_443 => write!(f, "破洒水壶_443"),
            Self::马口铁洒水壶_444 => write!(f, "马口铁洒水壶_444"),
            Self::狮子洒水壶_445 => write!(f, "狮子洒水壶_445"),
            Self::彩虹洒水壶_446 => write!(f, "彩虹洒水壶_446"),
            Self::幸福的洒水壶_447 => write!(f, "幸福的洒水壶_447"),
            Self::破锄头_449 => write!(f, "破锄头_449"),
            Self::坚固的锄头_450 => write!(f, "坚固的锄头_450"),
            Self::熟练的锄头_451 => write!(f, "熟练的锄头_451"),
            Self::闪耀的锄头_452 => write!(f, "闪耀的锄头_452"),
            Self::喜悦的锄头_453 => write!(f, "喜悦的锄头_453"),
            Self::破钓竿_455 => write!(f, "破钓竿_455"),
            Self::初学者钓竿_456 => write!(f, "初学者钓竿_456"),
            Self::中级者钓竿_457 => write!(f, "中级者钓竿_457"),
            Self::名人钓竿_458 => write!(f, "名人钓竿_458"),
            Self::神木钓竿_459 => write!(f, "神木钓竿_459"),
            Self::魔杖_461 => write!(f, "魔杖_461"),
            Self::魔法棒_462 => write!(f, "魔法棒_462"),
            Self::银之杖_463 => write!(f, "银之杖_463"),
            Self::烈焰之杖_464 => write!(f, "烈焰之杖_464"),
            Self::寒冰之杖_465 => write!(f, "寒冰之杖_465"),
            Self::闪电之杖_466 => write!(f, "闪电之杖_466"),
            Self::大地之杖_467 => write!(f, "大地之杖_467"),
            Self::法师之杖_468 => write!(f, "法师之杖_468"),
            Self::魔术手杖_469 => write!(f, "魔术手杖_469"),
            Self::符文之杖_470 => write!(f, "符文之杖_470"),
            Self::魔法手仗_471 => write!(f, "魔法手仗_471"),
            Self::魔法扫把_472 => write!(f, "魔法扫把_472"),
            Self::篮子_473 => write!(f, "篮子_473"),
            Self::魔法注射器_474 => write!(f, "魔法注射器_474"),
            Self::短剑_475 => write!(f, "短剑_475"),
            Self::钢化刃_476 => write!(f, "钢化刃_476"),
            Self::风之刃_477 => write!(f, "风之刃_477"),
            Self::冰霜之刃_478 => write!(f, "冰霜之刃_478"),
            Self::斩钢刃_479 => write!(f, "斩钢刃_479"),
            Self::音速短剑_480 => write!(f, "音速短剑_480"),
            Self::沙罗曼达_481 => write!(f, "沙罗曼达_481"),
            Self::双鬼丸_482 => write!(f, "双鬼丸_482"),
            Self::狂暴_483 => write!(f, "狂暴_483"),
            Self::白金之刃_484 => write!(f, "白金之刃_484"),
            Self::伊弗利特_485 => write!(f, "伊弗利特_485"),
            Self::终极暴雪_486 => write!(f, "终极暴雪_486"),
            Self::暗黑邀约_487 => write!(f, "暗黑邀约_487"),
            Self::力量切割_488 => write!(f, "力量切割_488"),
            Self::龙骑兵之爪_489 => write!(f, "龙骑兵之爪_489"),
            Self::火焰之心_490 => write!(f, "火焰之心_490"),
            Self::沙漠之风_491 => write!(f, "沙漠之风_491"),
            Self::崩坏之墙_492 => write!(f, "崩坏之墙_492"),
            Self::亡神剑_493 => write!(f, "亡神剑_493"),
            Self::混沌之刃_494 => write!(f, "混沌之刃_494"),
            Self::符文之刃_495 => write!(f, "符文之刃_495"),
            Self::钢铁之刃_496 => write!(f, "钢铁之刃_496"),
            Self::绿宝石之刃_497 => write!(f, "绿宝石之刃_497"),
            Self::盗贼匕首_498 => write!(f, "盗贼匕首_498"),
            Self::双重痒痒挠_499 => write!(f, "双重痒痒挠_499"),
            Self::神官之剑_500 => write!(f, "神官之剑_500"),
            Self::诚挚之刃_501 => write!(f, "诚挚之刃_501"),
            Self::超级奢华之剑_502 => write!(f, "超级奢华之剑_502"),
            Self::双过滤勺_503 => write!(f, "双过滤勺_503"),
            Self::双葱_504 => write!(f, "双葱_504"),
            Self::双重正义_505 => write!(f, "双重正义_505"),
            Self::迷你盾_506 => write!(f, "迷你盾_506"),
            Self::铜盾_507 => write!(f, "铜盾_507"),
            Self::圆盾_508 => write!(f, "圆盾_508"),
            Self::白金盾_509 => write!(f, "白金盾_509"),
            Self::重盾_510 => write!(f, "重盾_510"),
            Self::骑士盾_511 => write!(f, "骑士盾_511"),
            Self::符文盾_512 => write!(f, "符文盾_512"),
            Self::魔法盾_513 => write!(f, "魔法盾_513"),
            Self::棱镜盾_514 => write!(f, "棱镜盾_514"),
            Self::元素盾_515 => write!(f, "元素盾_515"),
            Self::混沌盾_516 => write!(f, "混沌盾_516"),
            Self::乌龟盾_517 => write!(f, "乌龟盾_517"),
            Self::骨盾_518 => write!(f, "骨盾_518"),
            Self::鸢盾_519 => write!(f, "鸢盾_519"),
            Self::魔术盾_520 => write!(f, "魔术盾_520"),
            Self::猴子布娃娃_521 => write!(f, "猴子布娃娃_521"),
            Self::伞_522 => write!(f, "伞_522"),
            Self::海蓝宝石戒指_523 => write!(f, "海蓝宝石戒指_523"),
            Self::紫水晶戒指_524 => write!(f, "紫水晶戒指_524"),
            Self::绿宝石戒指_525 => write!(f, "绿宝石戒指_525"),
            Self::蓝宝石戒指_526 => write!(f, "蓝宝石戒指_526"),
            Self::钻石戒指_527 => write!(f, "钻石戒指_527"),
            Self::红宝石戒指_528 => write!(f, "红宝石戒指_528"),
            Self::幸福戒指_529 => write!(f, "幸福戒指_529"),
            Self::暗之戒指_530 => write!(f, "暗之戒指_530"),
            Self::火之戒指_531 => write!(f, "火之戒指_531"),
            Self::风之戒指_532 => write!(f, "风之戒指_532"),
            Self::水之戒指_533 => write!(f, "水之戒指_533"),
            Self::地之戒指_534 => write!(f, "地之戒指_534"),
            Self::银戒指_535 => write!(f, "银戒指_535"),
            Self::金戒指_536 => write!(f, "金戒指_536"),
            Self::白金戒指_537 => write!(f, "白金戒指_537"),
            Self::暴击戒指_538 => write!(f, "暴击戒指_538"),
            Self::静默戒指_539 => write!(f, "静默戒指_539"),
            Self::瘫痪戒指_540 => write!(f, "瘫痪戒指_540"),
            Self::毒素戒指_541 => write!(f, "毒素戒指_541"),
            Self::魔法戒指_542 => write!(f, "魔法戒指_542"),
            Self::廉价手镯_543 => write!(f, "廉价手镯_543"),
            Self::铜手镯_544 => write!(f, "铜手镯_544"),
            Self::银手镯_545 => write!(f, "银手镯_545"),
            Self::金手镯_546 => write!(f, "金手镯_546"),
            Self::白金手镯_547 => write!(f, "白金手镯_547"),
            Self::海蓝宝石胸针_548 => write!(f, "海蓝宝石胸针_548"),
            Self::紫水晶胸针_549 => write!(f, "紫水晶胸针_549"),
            Self::绿宝石胸针_550 => write!(f, "绿宝石胸针_550"),
            Self::蓝宝石胸针_551 => write!(f, "蓝宝石胸针_551"),
            Self::钻石胸针_552 => write!(f, "钻石胸针_552"),
            Self::红宝石胸针_553 => write!(f, "红宝石胸针_553"),
            Self::银坠饰_554 => write!(f, "银坠饰_554"),
            Self::心形坠饰_555 => write!(f, "心形坠饰_555"),
            Self::星形坠饰_556 => write!(f, "星形坠饰_556"),
            Self::太阳坠饰_557 => write!(f, "太阳坠饰_557"),
            Self::草原坠饰_558 => write!(f, "草原坠饰_558"),
            Self::水滴坠饰_559 => write!(f, "水滴坠饰_559"),
            Self::大地坠饰_560 => write!(f, "大地坠饰_560"),
            Self::神圣坠饰_561 => write!(f, "神圣坠饰_561"),
            Self::护符_562 => write!(f, "护符_562"),
            Self::皮腰带_563 => write!(f, "皮腰带_563"),
            Self::暴击之七_564 => write!(f, "暴击之七_564"),
            Self::辟邪符_565 => write!(f, "辟邪符_565"),
            Self::冠军腰带_566 => write!(f, "冠军腰带_566"),
            Self::结实的手套_567 => write!(f, "结实的手套_567"),
            Self::拼命三郎的劳作手套_568 => write!(f, "拼命三郎的劳作手套_568"),
            Self::赠品徽章_569 => write!(f, "赠品徽章_569"),
            Self::力量手套_570 => write!(f, "力量手套_570"),
            Self::魔斗的咒符_571 => write!(f, "魔斗的咒符_571"),
            Self::盾戒_572 => write!(f, "盾戒_572"),
            Self::停战的祈祷_573 => write!(f, "停战的祈祷_573"),
            Self::勇气徽章_574 => write!(f, "勇气徽章_574"),
            Self::豪杰之证_575 => write!(f, "豪杰之证_575"),
            Self::贤者之证_576 => write!(f, "贤者之证_576"),
            Self::手织围巾_577 => write!(f, "手织围巾_577"),
            Self::毛茸茸围巾_578 => write!(f, "毛茸茸围巾_578"),
            Self::必杀的秘诀_579 => write!(f, "必杀的秘诀_579"),
            Self::钢铁的秘诀_580 => write!(f, "钢铁的秘诀_580"),
            Self::魔法之道的秘诀_581 => write!(f, "魔法之道的秘诀_581"),
            Self::银发簪_582 => write!(f, "银发簪_582"),
            Self::金发簪_583 => write!(f, "金发簪_583"),
            Self::专心耳饰_584 => write!(f, "专心耳饰_584"),
            Self::时尚帽子_585 => write!(f, "时尚帽子_585"),
            Self::魔女耳饰_586 => write!(f, "魔女耳饰_586"),
            Self::胜负头巾_587 => write!(f, "胜负头巾_587"),
            Self::羽毛帽子_588 => write!(f, "羽毛帽子_588"),
            Self::名牌眼镜_589 => write!(f, "名牌眼镜_589"),
            Self::防灾头巾_590 => write!(f, "防灾头巾_590"),
            Self::魔法石耳饰_591 => write!(f, "魔法石耳饰_591"),
            Self::手工针织帽_592 => write!(f, "手工针织帽_592"),
            Self::羽毛长靴_593 => write!(f, "羽毛长靴_593"),
            Self::重靴_594 => write!(f, "重靴_594"),
            Self::皮长靴_595 => write!(f, "皮长靴_595"),
            Self::骑士靴_596 => write!(f, "骑士靴_596"),
            Self::雪靴_597 => write!(f, "雪靴_597"),
            Self::溜冰鞋_598 => write!(f, "溜冰鞋_598"),
            Self::哔哔鞋_599 => write!(f, "哔哔鞋_599"),
            Self::稳步靴_600 => write!(f, "稳步靴_600"),
            Self::幽灵靴_601 => write!(f, "幽灵靴_601"),
            Self::铁屐靴_602 => write!(f, "铁屐靴_602"),
            Self::阔步靴_603 => write!(f, "阔步靴_603"),
            Self::内增高鞋_604 => write!(f, "内增高鞋_604"),
            Self::光滑靴_605 => write!(f, "光滑靴_605"),
            Self::隐迹靴_606 => write!(f, "隐迹靴_606"),
            Self::步步运动靴_607 => write!(f, "步步运动靴_607"),
            Self::水蜘蛛_608 => write!(f, "水蜘蛛_608"),
            Self::火箭之翼_609 => write!(f, "火箭之翼_609"),
            Self::银靴_610 => write!(f, "银靴_610"),
            Self::金靴_611 => write!(f, "金靴_611"),
            Self::骨靴_612 => write!(f, "骨靴_612"),
            Self::小仙子靴_613 => write!(f, "小仙子靴_613"),
            Self::毛刷_614 => write!(f, "毛刷_614"),
            Self::剪毛刀_615 => write!(f, "剪毛刀_615"),
            Self::空瓶_616 => write!(f, "空瓶_616"),
            Self::恢复壶_617 => write!(f, "恢复壶_617"),
            Self::疗愈壶_618 => write!(f, "疗愈壶_618"),
            Self::神秘壶_619 => write!(f, "神秘壶_619"),
            Self::放大镜_620 => write!(f, "放大镜_620"),
            Self::芜菁种子_621 => write!(f, "芜菁种子_621"),
            Self::马铃薯种子_622 => write!(f, "马铃薯种子_622"),
            Self::黄瓜种子_623 => write!(f, "黄瓜种子_623"),
            Self::草莓种子_624 => write!(f, "草莓种子_624"),
            Self::卷心菜种子_625 => write!(f, "卷心菜种子_625"),
            Self::月落草种子_626 => write!(f, "月落草种子_626"),
            Self::托伊药草种子_627 => write!(f, "托伊药草种子_627"),
            Self::番茄种子_628 => write!(f, "番茄种子_628"),
            Self::玉米种子_629 => write!(f, "玉米种子_629"),
            Self::洋葱种子_630 => write!(f, "洋葱种子_630"),
            Self::南瓜种子_631 => write!(f, "南瓜种子_631"),
            Self::菠萝种子_632 => write!(f, "菠萝种子_632"),
            Self::粉红猫种子_633 => write!(f, "粉红猫种子_633"),
            Self::茄子种子_634 => write!(f, "茄子种子_634"),
            Self::胡萝卜种子_635 => write!(f, "胡萝卜种子_635"),
            Self::番薯种子_636 => write!(f, "番薯种子_636"),
            Self::菠菜种子_637 => write!(f, "菠菜种子_637"),
            Self::青椒种子_638 => write!(f, "青椒种子_638"),
            Self::魅蓝草种子_639 => write!(f, "魅蓝草种子_639"),
            Self::牧草种子_640 => write!(f, "牧草种子_640"),
            Self::樱草种子_641 => write!(f, "樱草种子_641"),
            Self::灯草种子_642 => write!(f, "灯草种子_642"),
            Self::青水晶花种子_643 => write!(f, "青水晶花种子_643"),
            Self::金刚花种子_644 => write!(f, "金刚花种子_644"),
            Self::铁千轮种子_645 => write!(f, "铁千轮种子_645"),
            Self::幸运草种子_646 => write!(f, "幸运草种子_646"),
            Self::原之焰火种子_647 => write!(f, "原之焰火种子_647"),
            Self::绿水晶花种子_648 => write!(f, "绿水晶花种子_648"),
            Self::树形草种子_649 => write!(f, "树形草种子_649"),
            Self::红叶花种子_650 => write!(f, "红叶花种子_650"),
            Self::剧毒蒲公英种子_651 => write!(f, "剧毒蒲公英种子_651"),
            Self::红水晶种子_652 => write!(f, "红水晶种子_652"),
            Self::白水晶种子_653 => write!(f, "白水晶种子_653"),
            Self::樱芜菁种子_654 => write!(f, "樱芜菁种子_654"),
            Self::白萝卜种子_655 => write!(f, "白萝卜种子_655"),
            Self::葱种子_656 => write!(f, "葱种子_656"),
            Self::白菜种子_657 => write!(f, "白菜种子_657"),
            Self::金卷心菜种子_658 => write!(f, "金卷心菜种子_658"),
            Self::金南瓜种子_659 => write!(f, "金南瓜种子_659"),
            Self::金马铃薯种子_660 => write!(f, "金马铃薯种子_660"),
            Self::金芜菁种子_661 => write!(f, "金芜菁种子_661"),
            Self::少女蜜瓜种子_662 => write!(f, "少女蜜瓜种子_662"),
            Self::火热果实种子_663 => write!(f, "火热果实种子_663"),
            Self::瞬移_664 => write!(f, "瞬移_664"),
            Self::火球_665 => write!(f, "火球_665"),
            Self::大火球_666 => write!(f, "大火球_666"),
            Self::爆破_667 => write!(f, "爆破_667"),
            Self::激光水波_668 => write!(f, "激光水波_668"),
            Self::平行激光水波_669 => write!(f, "平行激光水波_669"),
            Self::三角激光水波_670 => write!(f, "三角激光水波_670"),
            Self::螺旋石_671 => write!(f, "螺旋石_671"),
            Self::地之尖柱_672 => write!(f, "地之尖柱_672"),
            Self::复仇者之石_673 => write!(f, "复仇者之石_673"),
            Self::音速之风_674 => write!(f, "音速之风_674"),
            Self::双重音波_675 => write!(f, "双重音波_675"),
            Self::穿透音波_676 => write!(f, "穿透音波_676"),
            Self::光屏障_677 => write!(f, "光屏障_677"),
            Self::闪光_678 => write!(f, "闪光_678"),
            Self::棱镜_679 => write!(f, "棱镜_679"),
            Self::黑洞_680 => write!(f, "黑洞_680"),
            Self::黑蛇_681 => write!(f, "黑蛇_681"),
            Self::暗黑_682 => write!(f, "暗黑_682"),
            Self::治愈_683 => write!(f, "治愈_683"),
            Self::全体治愈_684 => write!(f, "全体治愈_684"),
            Self::大师治疗_685 => write!(f, "大师治疗_685"),
            Self::毒素治愈_686 => write!(f, "毒素治愈_686"),
            Self::瘫痪治愈_687 => write!(f, "瘫痪治愈_687"),
            Self::封印治愈_688 => write!(f, "封印治愈_688"),
            Self::能量_690 => write!(f, "能量_690"),
            Self::冲击挥砍_691 => write!(f, "冲击挥砍_691"),
            Self::猛烈攻击_692 => write!(f, "猛烈攻击_692"),
            Self::破坏回旋_693 => write!(f, "破坏回旋_693"),
            Self::精神力突进_694 => write!(f, "精神力突进_694"),
            Self::烈空_695 => write!(f, "烈空_695"),
            Self::云裂_696 => write!(f, "云裂_696"),
            Self::瞬迅_697 => write!(f, "瞬迅_697"),
            Self::双刺_698 => write!(f, "双刺_698"),
            Self::罗闪_699 => write!(f, "罗闪_699"),
            Self::旋风剑_700 => write!(f, "旋风剑_700"),
            Self::一心一刀_701 => write!(f, "一心一刀_701"),
            Self::无心剑_702 => write!(f, "无心剑_702"),
            Self::不屈架式_703 => write!(f, "不屈架式_703"),
            Self::行云流水三段_704 => write!(f, "行云流水三段_704"),
            Self::天空气旋_705 => write!(f, "天空气旋_705"),
            Self::奋力收割_706 => write!(f, "奋力收割_706"),
            Self::百万打击_707 => write!(f, "百万打击_707"),
            Self::灾难行进_708 => write!(f, "灾难行进_708"),
            Self::星尘飞升_709 => write!(f, "星尘飞升_709"),
            Self::龙卷挥击_710 => write!(f, "龙卷挥击_710"),
            Self::大地冲击_711 => write!(f, "大地冲击_711"),
            Self::百万挥击_712 => write!(f, "百万挥击_712"),
            Self::特技协奏曲_713 => write!(f, "特技协奏曲_713"),
            Self::变身腰带_716 => write!(f, "变身腰带_716"),
            Self::中和剂_717 => write!(f, "中和剂_717"),
            Self::速速绿_718 => write!(f, "速速绿_718"),
            Self::很会长_719 => write!(f, "很会长_719"),
            Self::快速长_720 => write!(f, "快速长_720"),
            Self::极速长_721 => write!(f, "极速长_721"),
            Self::牧草_723 => write!(f, "牧草_723"),
            Self::软然毛_724 => write!(f, "软然毛_724"),
            Self::软绒毛_725 => write!(f, "软绒毛_725"),
            Self::毛茸茸的软软毛_726 => write!(f, "毛茸茸的软软毛_726"),
            Self::毛线球S_727 => write!(f, "毛线球S_727"),
            Self::毛线球_728 => write!(f, "毛线球_728"),
            Self::毛线球L_729 => write!(f, "毛线球L_729"),
            Self::废铁_730 => write!(f, "废铁_730"),
            Self::铁_731 => write!(f, "铁_731"),
            Self::铜_732 => write!(f, "铜_732"),
            Self::银_733 => write!(f, "银_733"),
            Self::金_734 => write!(f, "金_734"),
            Self::白金_735 => write!(f, "白金_735"),
            Self::钻石_736 => write!(f, "钻石_736"),
            Self::红宝石_737 => write!(f, "红宝石_737"),
            Self::绿宝石_738 => write!(f, "绿宝石_738"),
            Self::蓝宝石_739 => write!(f, "蓝宝石_739"),
            Self::紫水晶_740 => write!(f, "紫水晶_740"),
            Self::海蓝宝石_741 => write!(f, "海蓝宝石_741"),
            Self::兽人的破布_742 => write!(f, "兽人的破布_742"),
            Self::海盗哥布林的布_743 => write!(f, "海盗哥布林的布_743"),
            Self::丝绸_744 => write!(f, "丝绸_744"),
            Self::生锈的箭头_745 => write!(f, "生锈的箭头_745"),
            Self::战士之证_746 => write!(f, "战士之证_746"),
            Self::胶_747 => write!(f, "胶_747"),
            Self::哥布林的绷带_748 => write!(f, "哥布林的绷带_748"),
            Self::射手的引火药_749 => write!(f, "射手的引火药_749"),
            Self::地之结晶_750 => write!(f, "地之结晶_750"),
            Self::魔人的角_751 => write!(f, "魔人的角_751"),
            Self::恶魔之血_752 => write!(f, "恶魔之血_752"),
            Self::魔法师的粉末_753 => write!(f, "魔法师的粉末_753"),
            Self::魔力结晶_754 => write!(f, "魔力结晶_754"),
            Self::维京肩甲_755 => write!(f, "维京肩甲_755"),
            Self::巨人之爪_756 => write!(f, "巨人之爪_756"),
            Self::巨人的手套_757 => write!(f, "巨人的手套_757"),
            Self::锤子的碎片_758 => write!(f, "锤子的碎片_758"),
            Self::虫皮_759 => write!(f, "虫皮_759"),
            Self::昆虫的下颚_760 => write!(f, "昆虫的下颚_760"),
            Self::强韧的蜘蛛丝_761 => write!(f, "强韧的蜘蛛丝_761"),
            Self::漂亮蜘蛛丝_762 => write!(f, "漂亮蜘蛛丝_762"),
            Self::漂亮的虫皮_763 => write!(f, "漂亮的虫皮_763"),
            Self::坚固的角_764 => write!(f, "坚固的角_764"),
            Self::蝎子的尾巴_765 => write!(f, "蝎子的尾巴_765"),
            Self::蝎子的钳子_766 => write!(f, "蝎子的钳子_766"),
            Self::豹爪_767 => write!(f, "豹爪_767"),
            Self::龙牙_768 => write!(f, "龙牙_768"),
            Self::狼牙_769 => write!(f, "狼牙_769"),
            Self::优质毛皮_770 => write!(f, "优质毛皮_770"),
            Self::斗牛的角_771 => write!(f, "斗牛的角_771"),
            Self::鸟羽毛_772 => write!(f, "鸟羽毛_772"),
            Self::风之结晶_773 => write!(f, "风之结晶_773"),
            Self::毛皮_774 => write!(f, "毛皮_774"),
            Self::植物的根_775 => write!(f, "植物的根_775"),
            Self::蘑菇孢子_776 => write!(f, "蘑菇孢子_776"),
            Self::毒粉_777 => write!(f, "毒粉_777"),
            Self::植物的茎_778 => write!(f, "植物的茎_778"),
            Self::结实的藤蔓_779 => write!(f, "结实的藤蔓_779"),
            Self::古代鱼的骨头_780 => write!(f, "古代鱼的骨头_780"),
            Self::水之结晶_781 => write!(f, "水之结晶_781"),
            Self::龟壳_782 => write!(f, "龟壳_782"),
            Self::火之结晶_783 => write!(f, "火之结晶_783"),
            Self::浮游灵的头巾_784 => write!(f, "浮游灵的头巾_784"),
            Self::骷髅_785 => write!(f, "骷髅_785"),
            Self::坏掉的柄_786 => write!(f, "坏掉的柄_786"),
            Self::坏掉的木箱_787 => write!(f, "坏掉的木箱_787"),
            Self::小仙子的粉末_788 => write!(f, "小仙子的粉末_788"),
            Self::小结晶_789 => write!(f, "小结晶_789"),
            Self::摩可棉毛_790 => write!(f, "摩可棉毛_790"),
            Self::光之结晶_791 => write!(f, "光之结晶_791"),
            Self::暗之结晶_792 => write!(f, "暗之结晶_792"),
            Self::爱之结晶_793 => write!(f, "爱之结晶_793"),
            Self::狸猫的叶子_794 => write!(f, "狸猫的叶子_794"),
            Self::龙骨_795 => write!(f, "龙骨_795"),
            Self::狮子的红毛_796 => write!(f, "狮子的红毛_796"),
            Self::狮子的青毛_797 => write!(f, "狮子的青毛_797"),
            Self::冰壁的碎片_798 => write!(f, "冰壁的碎片_798"),
            Self::菊石_799 => write!(f, "菊石_799"),
            Self::冰在鼻_800 => write!(f, "冰在鼻_800"),
            Self::究极的胸毛_801 => write!(f, "究极的胸毛_801"),
            Self::奇美拉的尾巴_802 => write!(f, "奇美拉的尾巴_802"),
            Self::克里莫亚的鳞片_803 => write!(f, "克里莫亚的鳞片_803"),
            Self::百鱼王鳞片_804 => write!(f, "百鱼王鳞片_804"),
            Self::水龙羽鳍_805 => write!(f, "水龙羽鳍_805"),
            Self::会动的树枝_806 => write!(f, "会动的树枝_806"),
            Self::电之结晶_807 => write!(f, "电之结晶_807"),
            Self::会唱歌的小瓶子_808 => write!(f, "会唱歌的小瓶子_808"),
            Self::石魔人的石板_809 => write!(f, "石魔人的石板_809"),
            Self::地龙鳞片_810 => write!(f, "地龙鳞片_810"),
            Self::火龙鳞片_811 => write!(f, "火龙鳞片_811"),
            Self::石川鲑鱼_830 => write!(f, "石川鲑鱼_830"),
            Self::乌贼_831 => write!(f, "乌贼_831"),
            Self::伊富鱼_832 => write!(f, "伊富鱼_832"),
            Self::沙丁鱼_833 => write!(f, "沙丁鱼_833"),
            Self::红点鲑_834 => write!(f, "红点鲑_834"),
            Self::珠星三块鱼_835 => write!(f, "珠星三块鱼_835"),
            Self::闪耀鲷鱼_836 => write!(f, "闪耀鲷鱼_836"),
            Self::鲣鱼_837 => write!(f, "鲣鱼_837"),
            Self::鲽鱼_838 => write!(f, "鲽鱼_838"),
            Self::鲋鱼_839 => write!(f, "鲋鱼_839"),
            Self::钩吻鱼_840 => write!(f, "钩吻鱼_840"),
            Self::青花鱼_841 => write!(f, "青花鱼_841"),
            Self::针鱼_842 => write!(f, "针鱼_842"),
            Self::秋刀鱼_843 => write!(f, "秋刀鱼_843"),
            Self::斑乌贼_844 => write!(f, "斑乌贼_844"),
            Self::虾_845 => write!(f, "虾_845"),
            Self::鲷鱼_846 => write!(f, "鲷鱼_846"),
            Self::心动鲷鱼_847 => write!(f, "心动鲷鱼_847"),
            Self::剧毒虹鳟_848 => write!(f, "剧毒虹鳟_848"),
            Self::比目鱼_849 => write!(f, "比目鱼_849"),
            Self::河豚_850 => write!(f, "河豚_850"),
            Self::青甘鱼_851 => write!(f, "青甘鱼_851"),
            Self::高身鲫_852 => write!(f, "高身鲫_852"),
            Self::金枪鱼_853 => write!(f, "金枪鱼_853"),
            Self::黑毛鱼_854 => write!(f, "黑毛鱼_854"),
            Self::红叶比目鱼_855 => write!(f, "红叶比目鱼_855"),
            Self::山女鳟_856 => write!(f, "山女鳟_856"),
            Self::封印乌贼_857 => write!(f, "封印乌贼_857"),
            Self::螯龙虾_858 => write!(f, "螯龙虾_858"),
            Self::西太公鱼_859 => write!(f, "西太公鱼_859"),
            Self::沙比目鱼_860 => write!(f, "沙比目鱼_860"),
            Self::岩石鱼_861 => write!(f, "岩石鱼_861"),
            Self::空罐_862 => write!(f, "空罐_862"),
            Self::高筒靴_863 => write!(f, "高筒靴_863"),
            Self::罕见的空罐_864 => write!(f, "罕见的空罐_864"),
            Self::石头_865 => write!(f, "石头_865"),
            Self::树枝_866 => write!(f, "树枝_866"),
            Self::木材_867 => write!(f, "木材_867"),
            Self::剑草种子_868 => write!(f, "剑草种子_868"),
            Self::干瘪的剑草种子_869 => write!(f, "干瘪的剑草种子_869"),
            Self::钢身花种子_870 => write!(f, "钢身花种子_870"),
            Self::柔软的的钢身花种子_871 => write!(f, "柔软的的钢身花种子_871"),
            Self::追风草种子_872 => write!(f, "追风草种子_872"),
            Self::静止的追风草种子_873 => write!(f, "静止的追风草种子_873"),
            Self::杰克南瓜的种子_874 => write!(f, "杰克南瓜的种子_874"),
            Self::废柴杰克的种子_875 => write!(f, "废柴杰克的种子_875"),
            Self::蜜瓜炸弹的种子_876 => write!(f, "蜜瓜炸弹的种子_876"),
            Self::坏掉的蜜瓜种子_877 => write!(f, "坏掉的蜜瓜种子_877"),
            Self::土俑仙人掌种子_878 => write!(f, "土俑仙人掌种子_878"),
            Self::干枯的土俑仙人掌种子_879 => write!(f, "干枯的土俑仙人掌种子_879"),
            Self::水生草的种子_880 => write!(f, "水生草的种子_880"),
            Self::干枯的水生草种子_881 => write!(f, "干枯的水生草种子_881"),
            Self::莲叶船种子_882 => write!(f, "莲叶船种子_882"),
            Self::折断的莲花种子_883 => write!(f, "折断的莲花种子_883"),
            Self::滑倒香蕉种子_884 => write!(f, "滑倒香蕉种子_884"),
            Self::腐烂的香蕉种子_885 => write!(f, "腐烂的香蕉种子_885"),
            Self::魔法蚕豆_886 => write!(f, "魔法蚕豆_886"),
            Self::别扭的蚕豆种子_887 => write!(f, "别扭的蚕豆种子_887"),
        }
    }
}

impl CropProperties {
    fn 设置作物类型(&mut self, v: Crop) {
        if v as u8 == 0 {
            self.类型 = 0;
        }

        self.类型 = (v as u8) << 1;
    }

    unsafe fn 设置作物阶段(&mut self, v: CropStage) {
        self.状态.阶段 &= 0b0000_1111;

        self.状态.阶段 |= (v as u8) << 4;
    }

    unsafe fn 设置作物等级(&mut self, v: CropLevel) {
        self.状态.等级 &= 0b0111_0000;

        self.状态.等级 |= v as u8;
    }
}

impl Items {
    unsafe fn 设置(&mut self, index: u16, quantity: u16) {
        let high_bits = self.item & 0b1100_0000_0000_0000;
        let bits_1_10 = index & 0b11_1111_1111;
        let bits_11_14 = (quantity & 0b1111) << 10;

        self.item = high_bits | bits_11_14 | bits_1_10
    }
}
