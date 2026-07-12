pub const PRODUCT: &str = "Rust Pass Reveal";

pub const ICON_SIZE: u32 = 32;

pub fn icon_rgba() -> Vec<u8> {
    include_bytes!("assets/icon.rgba").to_vec()
}
