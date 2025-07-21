#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris"
))]
fn supports_os() -> bool {
    true
}
#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris"
)))]
fn supports_os() -> bool {
    false
}

fn main() -> Result<(), String> {
    if supports_os() {
        Ok(())
    } else {
        let os = std::env::consts::OS.to_string();
        Err(format!("OS `{os}' is not supported by this crate"))
    }
}
