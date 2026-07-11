pub const PRODUCT: &str = "Rust Password";
pub const DESCRIPTION: &str = "Rust registry and Steam status viewer";
pub const COMPANY: &str = ".insoulglobal";
pub const COPYRIGHT: &str = "Copyright © 2026 .insoulglobal";
pub const EXE_NAME: &str = "rust-password.exe";

pub const ICON_SIZE: u32 = 32;

pub fn icon_rgba() -> Vec<u8> {
    include_bytes!("assets/icon.rgba").to_vec()
}
