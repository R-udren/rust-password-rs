//! Embeds Windows branding and version information in the executable.

const PRODUCT: &str = "Rust Pass Reveal";
const DESCRIPTION: &str = "Rust game Codelock Revealer.";
const COMPANY: &str = ".insoulglobal";
const COPYRIGHT: &str = "Copyright © 2026 .insoulglobal";
const EXE_NAME: &str = "rust-password.exe";

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use winresource::{VersionInfo, WindowsResource};

    let version = env!("CARGO_PKG_VERSION");
    let mut resource = WindowsResource::new();
    resource
        .set_icon("assets/icon.ico")
        .set("FileDescription", DESCRIPTION)
        .set("ProductName", PRODUCT)
        .set("CompanyName", COMPANY)
        .set("LegalCopyright", COPYRIGHT)
        .set("OriginalFilename", EXE_NAME)
        .set("FileVersion", version)
        .set("ProductVersion", version)
        .set_version_info(VersionInfo::FILEVERSION, numeric_version()?)
        .set_version_info(VersionInfo::PRODUCTVERSION, numeric_version()?);
    resource.compile()?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn numeric_version() -> Result<u64, std::num::ParseIntError> {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u64>()?;
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u64>()?;
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u64>()?;
    Ok((major << 48) | (minor << 32) | (patch << 16))
}
