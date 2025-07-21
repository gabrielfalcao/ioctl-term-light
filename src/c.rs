#![allow(non_camel_case_types)]
use std::mem::zeroed;
pub(crate) use std::os::raw::c_int;
pub(crate) use std::os::raw::c_ushort;
pub(crate) use std::os::raw::c_ulong;
// pub type c_int = i32;
// pub type c_ulong = u64;
// pub type c_ushort = u16;

#[repr(C)]
pub(crate) struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

unsafe extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) static TIOCGWINSZ: c_ulong = 0x5413;

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub(crate) static TIOCGWINSZ: c_ulong = 0x40087468;

#[cfg(target_os = "solaris")]
pub(crate) static TIOCGWINSZ: c_ulong = 0x5468;

pub(crate) unsafe fn ioctl_get_dimensions(fd: c_int) -> winsize {
    let mut window: winsize = unsafe { zeroed() };
    let result = unsafe { ioctl(fd, TIOCGWINSZ, &mut window) };

    if result != -1 {
        return window;
    }
    unsafe { zeroed() }
}
