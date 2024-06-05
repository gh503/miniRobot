// 将字节大小转换为易读格式的函数
pub fn format_size(size: u64) -> String {
    let units = ["bytes", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.0} {}", size, units[unit])
}