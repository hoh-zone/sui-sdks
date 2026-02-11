pub const MAX_U8: u8 = u8::MAX;

pub fn create_full_id(package_id: &str, id: &str) -> String {
    let clean = package_id.strip_prefix("0x").unwrap_or(package_id);
    format!("{}{}", clean, id)
}
