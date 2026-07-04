pub struct SizeFormatter;

impl SizeFormatter {
    pub fn format(bytes: u64) -> String {
        let units = ["B", "KB", "MB", "GB", "TB"];

        let mut value = bytes as f64;
        let mut unit_index = 0;

        while value >= 1024.0 && unit_index < units.len() - 1 {
            value /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, units[unit_index])
        } else {
            format!("{:.2} {}", value, units[unit_index])
        }
    }
}
