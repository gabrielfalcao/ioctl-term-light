#![allow(unused)]
use crate::c::ioctl_get_dimensions;
use crate::impl_raw_fd;
use crate::Dimensions;
use std::os::fd::AsRawFd;

impl_raw_fd!(Stdin, 0);
impl_raw_fd!(Stdout, 1);
impl_raw_fd!(Stderr, 2);

/// Attempts to obtain a tuple with (rows, columns) of the given [std::os::fd::AsRawFd] implementor.
///
/// ```
/// use ioctl_term_light::tuple_from_raw_fd;
///
/// assert_eq!(tuple_from_raw_fd(ioctl_term_light::Stdout), None);
///
/// assert_eq!(tuple_from_raw_fd(ioctl_term_light::Stdin), None);
///
/// assert_eq!(tuple_from_raw_fd(ioctl_term_light::Stderr), None);
/// ```
pub fn tuple_from_raw_fd<FileDescriptor: AsRawFd>(fd: FileDescriptor) -> Option<(u16, u16)> {
    let no = fd.as_raw_fd();
    let ws = unsafe { ioctl_get_dimensions(no) };
    if ws.ws_col == 0 || ws.ws_row == 0 {
        None
    } else {
        Some((ws.ws_col as u16, ws.ws_row as u16))
    }
}

/// Returns a tuple with (rows, columns) of the current process'es (stdout, stdin and stderr) sequentially.
///
/// ```
/// use ioctl_term_light::tuple;
///
/// assert_eq!(tuple(), (0, 0));
/// ```
pub fn tuple() -> (u16, u16) {
    None::<(u16, u16)>
        .or_else(|| tuple_from_raw_fd(Stdout))
        .or_else(|| tuple_from_raw_fd(Stdin))
        .or_else(|| tuple_from_raw_fd(Stderr))
        .unwrap_or_else(|| (0u16, 0u16))
}

/// Returns the number of columns in the current terminal via [tuple](crate::tuple)
///
/// ```
/// use ioctl_term_light::cols;
///
/// assert_eq!(cols(), 0);
/// ```
pub fn cols() -> u16 {
    tuple().0
}

/// Returns the number of rows in the current terminal via [tuple](crate::tuple)
///
/// ```
/// use ioctl_term_light::rows;
///
/// assert_eq!(rows(), 0);
/// ```
pub fn rows() -> u16 {
    tuple().1
}

#[cfg(test)]
mod tests {
    use super::{cols, rows, tuple, tuple_from_raw_fd, Stderr, Stdin, Stdout};

    #[test]
    fn test_tuple_from_raw_fd_stdout() {
        assert_ne!(
            tuple_from_raw_fd(Stdout),
            None,
            "expected tuple_from_raw_fd(Stdout) to not be None"
        );
        assert_ne!(
            tuple_from_raw_fd(Stdout),
            Some((0, 0)),
            "expected tuple_from_raw_fd(Stdout) to not be Some((0, 0))"
        );
    }

    #[test]
    fn test_tuple_from_raw_fd_stdin() {
        assert_ne!(
            tuple_from_raw_fd(Stdin),
            None,
            "expected tuple_from_raw_fd(Stdin) to not be None"
        );
        assert_ne!(
            tuple_from_raw_fd(Stdin),
            Some((0, 0)),
            "expected tuple_from_raw_fd(Stdin) to not be Some((0, 0))"
        );
    }

    #[test]
    fn test_tuple_from_raw_fd_stderr() {
        assert_ne!(
            tuple_from_raw_fd(Stderr),
            None,
            "expected tuple_from_raw_fd(Stderr) to not be None"
        );
        assert_ne!(
            tuple_from_raw_fd(Stderr),
            Some((0, 0)),
            "expected tuple_from_raw_fd(Stderr) to not be Some((0, 0))"
        );
    }

    #[test]
    fn test_cols() {
        let result = cols();
        assert_ne!(result, 0, "expected cols() to not be 0 (zero)");
    }

    #[test]
    fn test_rows() {
        let result = rows();
        assert_ne!(result, 0, "expected rows() to not be 0 (zero)");
    }

    #[test]
    fn test_tuple() {
        let result = tuple();
        assert_ne!(result, (0, 0), "expected tuple() to not be (0, 0)");
    }
}

#[macro_export]
macro_rules! impl_raw_fd {
    ($name:ident, $fd:literal) => {
        /// Represents the standard file descriptor (.i.e.: stdout, stdin or stderr), equivalent to [`std::io::stdout`](std::io::stdout), [`std::io::stdin`](std::io::stdin) or [`std::io::stderr`](std::io::stderr) (respectively)
        #[derive(Clone, Copy, Debug)]
        pub struct $name;
        impl std::os::fd::AsRawFd for $name {
            fn as_raw_fd(&self) -> std::os::fd::RawFd {
                $fd
            }
        }
    };
}
