use std::collections::HashMap;

/// 基础资源
pub const BASE_RES: [&str; 9] = ["energy", "U", "L", "K", "Z", "X", "O", "H", "G"];

/// 能量资源
pub const POWER_RES: [&str; 2] = ["power", "ops"];

/// 条形资源
pub const BARS_RES: [&str; 9] = [
    "battery",
    "utrium_bar",
    "lemergium_bar",
    "keanium_bar",
    "zynthium_bar",
    "purifier",
    "oxidant",
    "reductant",
    "ghodium_melt",
];

/// 灰色化合物资源
pub const C_GREY_RES: [&str; 3] = ["composite", "crystal", "liquid"];

/// 蓝色化合物资源
pub const C_BLUE_RES: [&str; 7] = [
    "silicon",
    "wire",
    "switch",
    "transistor",
    "microchip",
    "circuit",
    "device",
];

/// 黄色化合物资源
pub const C_YELLOW_RES: [&str; 7] = [
    "metal",
    "alloy",
    "tube",
    "fixtures",
    "frame",
    "hydraulics",
    "machine",
];

/// 粉色化合物资源
pub const C_PINK_RES: [&str; 7] = [
    "mist",
    "condensate",
    "concentrate",
    "extract",
    "spirit",
    "emanation",
    "essence",
];

/// 绿色化合物资源
pub const C_GREEN_RES: [&str; 7] = [
    "biomass", "cell", "phlegm", "tissue", "muscle", "organoid", "organism",
];

/// 灰色基础资源
pub const B_GREY_RES: [&str; 4] = ["OH", "ZK", "UL", "G"];

/// 蓝色基础资源
pub const B_BLUE_RES: [&str; 7] = ["UH", "UH2O", "XUH2O", "UO", "UHO2", "XUHO2", "utrium"];

/// 黄色基础资源
pub const B_YELLOW_RES: [&str; 7] = ["ZH", "ZH2O", "XZH2O", "ZO", "ZHO2", "XZHO2", "zynthium"];

/// 粉色基础资源
pub const B_PINK_RES: [&str; 7] = ["KH", "KH2O", "XKH2O", "KO", "KHO2", "XKHO2", "keanium"];

/// 绿色基础资源
pub const B_GREEN_RES: [&str; 7] = ["LH", "LH2O", "XLH2O", "LO", "LHO2", "XLHO2", "lemergium"];

/// 白色基础资源
pub const B_WHITE_RES: [&str; 7] = ["GH", "GH2O", "XGH2O", "GO", "GHO2", "XGHO2", "ghodium"];

/// 资源颜色映射
pub fn res_color_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("empty", "rgba(0,0,0,0)");
    map.insert("energy", "rgb(255,242,0)");
    map.insert("battery", "rgb(255,242,0)");
    map.insert("Z", "rgb(247, 212, 146)");
    map.insert("L", "rgb(108, 240, 169)");
    map.insert("U", "rgb(76, 167, 229)");
    map.insert("K", "rgb(218, 107, 245)");
    map.insert("X", "rgb(255, 192, 203)");
    map.insert("G", "rgb(255,255,255)");
    map.insert("zynthium_bar", "rgb(247, 212, 146)");
    map.insert("lemergium_bar", "rgb(108, 240, 169)");
    map.insert("utrium_bar", "rgb(76, 167, 229)");
    map.insert("keanium_bar", "rgb(218, 107, 245)");
    map.insert("purifier", "rgb(255, 192, 203)");
    map.insert("ghodium_melt", "rgb(255,255,255)");
    map.insert("power", "rgb(224,90,90)");
    map.insert("ops", "rgb(224,90,90)");
    map.insert("composite", "#ccc");
    map.insert("crystal", "#ccc");
    map.insert("liquid", "#ccc");
    map.insert("device", "rgb(76, 167,229)");
    map.insert("circuit", "rgb(76, 167,229)");
    map.insert("microchip", "rgb(76, 167,229)");
    map.insert("transistor", "rgb(76, 167,229)");
    map.insert("switch", "rgb(76, 167,229)");
    map.insert("wire", "rgb(76, 167,229)");
    map.insert("silicon", "rgb(76, 167,229)");
    map.insert("machine", "rgb(247,212,146)");
    map.insert("hydraulics", "rgb(247,212,146)");
    map.insert("frame", "rgb(247,212,146)");
    map.insert("fixtures", "rgb(247,212,146)");
    map.insert("tube", "rgb(247,212,146)");
    map.insert("alloy", "rgb(247,212,146)");
    map.insert("metal", "rgb(247,212,146)");
    map.insert("essence", "rgb(218,107,245)");
    map.insert("emanation", "rgb(218,107,245)");
    map.insert("spirit", "rgb(218,107,245)");
    map.insert("extract", "rgb(218,107,245)");
    map.insert("concentrate", "rgb(218,107,245)");
    map.insert("condensate", "rgb(218,107,245)");
    map.insert("mist", "rgb(218,107,245)");
    map.insert("organism", "rgb(108,240,169)");
    map.insert("organoid", "rgb(108,240,169)");
    map.insert("muscle", "rgb(108,240,169)");
    map.insert("tissue", "rgb(108,240,169)");
    map.insert("phlegm", "rgb(108,240,169)");
    map.insert("cell", "rgb(108,240,169)");
    map.insert("biomass", "rgb(108,240,169)");
    map.insert("OH", "#ccc");
    map.insert("ZK", "#ccc");
    map.insert("UL", "#ccc");
    map.insert("UH", "rgb(76, 167,229)");
    map.insert("UH2O", "rgb(76, 167,229)");
    map.insert("XUH2O", "rgb(76, 167,229)");
    map.insert("UO", "rgb(76, 167,229)");
    map.insert("UHO2", "rgb(76, 167,229)");
    map.insert("XUHO2", "rgb(76, 167,229)");
    map.insert("ZH", "rgb(247,212,146)");
    map.insert("ZH2O", "rgb(247,212,146)");
    map.insert("XZH2O", "rgb(247,212,146)");
    map.insert("ZO", "rgb(247,212,146)");
    map.insert("ZHO2", "rgb(247,212,146)");
    map.insert("XZHO2", "rgb(247,212,146)");
    map.insert("KH", "rgb(218,107,245)");
    map.insert("KH2O", "rgb(218,107,245)");
    map.insert("XKH2O", "rgb(218,107,245)");
    map.insert("KO", "rgb(218,107,245)");
    map.insert("KHO2", "rgb(218,107,245)");
    map.insert("XKHO2", "rgb(218,107,245)");
    map.insert("LH", "rgb(108,240,169)");
    map.insert("LH2O", "rgb(108,240,169)");
    map.insert("XLH2O", "rgb(108,240,169)");
    map.insert("LO", "rgb(108,240,169)");
    map.insert("LHO2", "rgb(108,240,169)");
    map.insert("XLHO2", "rgb(108,240,169)");
    map.insert("GH", "rgb(255,255,255)");
    map.insert("GH2O", "rgb(255,255,255)");
    map.insert("XGH2O", "rgb(255,255,255)");
    map.insert("GO", "rgb(255,255,255)");
    map.insert("GHO2", "rgb(255,255,255)");
    map.insert("XGHO2", "rgb(255,255,255)");
    map.insert("H", "#ccc");
    map.insert("O", "#ccc");
    map.insert("oxidant", "#ccc");
    map.insert("reductant", "#ccc");
    map.insert("utrium", "rgb(76, 167,229)");
    map.insert("lemergium", "rgb(108, 240, 169)");
    map.insert("keanium", "rgb(218, 107, 245)");
    map.insert("zynthium", "rgb(247, 212, 146)");
    map.insert("ghodium", "rgb(255,255,255)");
    map
}
