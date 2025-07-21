#![allow(unused)]

use std::os::fd::AsRawFd;

use crate::c::{c_int, ioctl_get_dimensions};
use crate::Dimensions;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FileDescriptor {
    Stdin,
    Stdout,
    Stderr,
    #[cfg(feature = "any-fd")]
    Any(i32),
}
impl FileDescriptor {
    pub fn no(self) -> i32 {
        match self {
            FileDescriptor::Stdin => 0,
            FileDescriptor::Stdout => 1,
            FileDescriptor::Stderr => 2,
            #[cfg(feature = "any-fd")]
            FileDescriptor::Any(no) => no,
        }
    }
    pub fn get_dimensions(self) -> Option<(u16, u16)> {
        let ws = unsafe { ioctl_get_dimensions(self.no()) };
        if ws.ws_col == 0 || ws.ws_row == 0 {
            None
        } else {
            Some((ws.ws_col as u16, ws.ws_row as u16))
        }
    }
    pub fn dimensions(self) -> Dimensions {
        Dimensions::from(self.get_dimensions())
    }
}

impl AsRawFd for FileDescriptor {
    fn as_raw_fd(&self) -> c_int {
        self.no()
    }
}

#[cfg(test)]
mod tests {
    use super::FileDescriptor;
    use crate::Dimensions;

    #[test]
    fn test_file_descriptor_stdout_get_dimensions() {
        let dimensions = FileDescriptor::Stdout.get_dimensions();
        assert_ne!(
            dimensions, None,
            "expected FileDescriptor::Stdout::get_dimensions() to not return None"
        );
        assert_ne!(
            dimensions,
            Some((0, 0)),
            "expected FileDescriptor::Stdout::get_dimensions() to not return Some((0, 0))"
        );
    }

    #[test]
    fn test_file_descriptor_stdin_get_dimensions() {
        let dimensions = FileDescriptor::Stdin.get_dimensions();
        assert_ne!(
            dimensions, None,
            "expected FileDescriptor::Stdin::get_dimensions() to not return None"
        );
        assert_ne!(
            dimensions,
            Some((0, 0)),
            "expected FileDescriptor::Stdin::get_dimensions() to not return Some((0, 0))"
        );
    }
    #[test]
    fn test_file_descriptor_stderr_get_dimensions() {
        let dimensions = FileDescriptor::Stderr.get_dimensions();
        assert_ne!(
            dimensions, None,
            "expected FileDescriptor::Stderr::get_dimensions() to not return None"
        );
        assert_ne!(
            dimensions,
            Some((0, 0)),
            "expected FileDescriptor::Stderr::get_dimensions() to not return Some((0, 0))"
        );
    }

    #[test]
    fn test_file_descriptor_stdout_dimensions() {
        let dimensions = FileDescriptor::Stdout.dimensions();
        let unexpected = Dimensions::from((0u16, 0u16));
        assert_ne!(
            dimensions, unexpected,
            "expected FileDescriptor::Stdout::get_dimensions() to not return {unexpected:#?}"
        );
    }
    #[test]
    fn test_file_descriptor_stdin_dimensions() {
        let dimensions = FileDescriptor::Stdin.dimensions();
        let unexpected = Dimensions::from((0u16, 0u16));
        assert_ne!(
            dimensions, unexpected,
            "expected FileDescriptor::Stdin::get_dimensions() to not return {unexpected:#?}"
        );
    }
    #[test]
    fn test_file_descriptor_stderr_dimensions() {
        let dimensions = FileDescriptor::Stderr.dimensions();
        let unexpected = Dimensions::from((0u16, 0u16));
        assert_ne!(
            dimensions, unexpected,
            "expected FileDescriptor::Stderr::get_dimensions() to not return {unexpected:#?}"
        );
    }
}
