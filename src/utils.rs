use plotters::{coord::Shift, prelude::*};
use std::{collections::HashMap, fs, path::Path, str::FromStr};

/// 将 HEX 颜色或 RGB 颜色字符串转换为 RGBColor
/// 支持以下格式：
/// - 3位HEX: #RGB 或 RGB
/// - 6位HEX: #RRGGBB 或 RRGGBB
/// - RGB: rgb(r, g, b) 或 rgba(r, g, b, a)
pub fn parse_color(color_str: &str) -> Result<RGBColor, Box<dyn std::error::Error>> {
    let trimmed = color_str.trim();

    // 处理 rgb() 或 rgba() 格式
    if trimmed.starts_with("rgb(") && trimmed.ends_with(")") {
        let inner = &trimmed[4..trimmed.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

        if parts.len() < 3 {
            return Err("RGB格式错误，需要至少3个组件".into());
        }

        let r = u8::from_str(parts[0])?;
        let g = u8::from_str(parts[1])?;
        let b = u8::from_str(parts[2])?;

        return Ok(RGBColor(r, g, b));
    }

    // 处理 HEX 格式
    let hex = trimmed.trim_start_matches('#');

    match hex.len() {
        // 3位HEX格式，如 #RGB -> #RRGGBB
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16)? * 17; // 乘以17相当于重复一位，如F -> FF
            let g = u8::from_str_radix(&hex[1..2], 16)? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16)? * 17;
            Ok(RGBColor(r, g, b))
        }
        // 6位HEX格式，如 #RRGGBB
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16)?;
            let g = u8::from_str_radix(&hex[2..4], 16)?;
            let b = u8::from_str_radix(&hex[4..6], 16)?;
            Ok(RGBColor(r, g, b))
        }
        _ => Err(format!("HEX颜色必须是3位或6位，当前长度: {}", hex.len()).into()),
    }
}

/// 绘制文本
pub fn draw_text<T: DrawingBackend>(
    root: &DrawingArea<T, Shift>,
    text: &str,
    x: u32,
    y: u32,
    size: u32,
    color: &str,
) -> () {
    let _ = root.draw_text(
        &text,
        &TextStyle::from(("sans-serif", size).into_font())
            .color(&parse_color(color).unwrap_or(RGBColor(255, 255, 255))),
        (x as i32, y as i32),
    );
}

/// 绘制资源文本
pub fn draw_res_text<T: DrawingBackend>(
    root: &DrawingArea<T, Shift>,
    res_type: &str,
    x: u32,
    y: u32,
    color: &str,
) -> () {
    let _ = draw_text(root, res_type, x, y, 14, color);
}

/// 绘制资源
pub fn draw_res<T: DrawingBackend>(
    root: &DrawingArea<T, Shift>,
    res_color_map: &HashMap<&str, &str>,
    name: &str,
    number: &i32,
    x: u32,
    y: u32,
) -> () {
    draw_res_text(&root, name, x, y, res_color_map.get(name).unwrap());
    draw_res_text(
        &root,
        &format_number(*number),
        x,
        y + 14,
        res_color_map.get(name).unwrap(),
    );
}

/// 将所有shard的资源统计合在一起
pub fn merge_res(res_map: &HashMap<String, HashMap<String, i32>>) -> HashMap<String, i32> {
    let mut res_sum = HashMap::new();
    for (_, res) in res_map {
        for (res_name, res_number) in res {
            *res_sum.entry(res_name.to_string()).or_insert(0) += res_number;
        }
    }
    res_sum
}

/// 创建数据文件夹
pub fn create_data_dir() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir(data_dir)?;
    }
    Ok(())
}

/// 千分位分割数字
pub fn format_number(num: i32) -> String {
    let num_str = num.to_string();
    
    // 处理负数情况
    let (prefix, digits) = if num_str.starts_with('-') {
        ("-", &num_str[1..])
    } else {
        ("", &num_str[..])
    };
    
    let len = digits.len();
    let mut result = String::from(prefix);
    
    for (i, ch) in digits.chars().enumerate() {
        // 计算当前字符后是否需要添加逗号
        // 从右边数起，每三位数字后添加一个逗号
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(12), "12");
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(12345), "12,345");
        assert_eq!(format_number(123456), "123,456");
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(12345678), "12,345,678");
        assert_eq!(format_number(123456789), "123,456,789");
        assert_eq!(format_number(-1234567), "-1,234,567");
        assert_eq!(format_number(-1234), "-1,234");
    }
}
