#[path = "branding.rs"]
mod branding;

#[cfg(windows)]
fn main() {
    use std::env;
    use winresource::{VersionInfo, WindowsResource};

    let version = env!("CARGO_PKG_VERSION");
    let mut resource = WindowsResource::new();
    resource
        .set_icon("assets/icon.ico")
        .set("FileDescription", branding::DESCRIPTION)
        .set("ProductName", branding::PRODUCT)
        .set("CompanyName", branding::COMPANY)
        .set("LegalCopyright", branding::COPYRIGHT)
        .set("OriginalFilename", branding::EXE_NAME)
        .set("FileVersion", version)
        .set("ProductVersion", version)
        .set_version_info(VersionInfo::FILEVERSION, numeric_version(version))
        .set_version_info(VersionInfo::PRODUCTVERSION, numeric_version(version));
    resource
        .compile()
        .expect("compile Windows executable resources");
}

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn numeric_version(version: &str) -> u64 {
    let mut parts = version.split('.').map(|part| {
        part.parse::<u16>()
            .expect("Cargo package version components must be numeric")
    });
    let major = parts.next().expect("package version has a major component") as u64;
    let minor = parts.next().unwrap_or_default() as u64;
    let patch = parts.next().unwrap_or_default() as u64;
    (major << 48) | (minor << 32) | (patch << 16)
}
