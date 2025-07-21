use crate::impl_std_fd;

#[cfg(any(feature = "stdio", feature = "stdin"))]
impl_std_fd!(stdin, 0);
#[cfg(any(feature = "stdio", feature = "stdout"))]
impl_std_fd!(stdout, 1);
#[cfg(any(feature = "stdio", feature = "stderr"))]
impl_std_fd!(stderr, 2);

#[cfg(any(
    feature = "stdio",
    feature = "stdin",
    feature = "stdout",
    feature = "stderr"
))]
#[macro_export]
macro_rules! impl_std_fd {
    ($name:ident, $fd:literal) => {
        pub mod $name {
            use $crate::c::{c_int, ioctl_get_dimensions};
            use $crate::Dimensions;
            pub const FILENO: c_int = $fd;

            /// Attempts to obtain a tuple with (rows, columns) of the current process'es stdout, stdin or stderr.
            ///
            /// Examples
            ///
            /// ```
            /// use ioctl_term_light::stdout;
            /// assert_eq!(stdout::get_dimensions(), None);
            /// ```
            ///
            /// ```
            /// use ioctl_term_light::stdin;
            /// assert_eq!(stdin::get_dimensions(), None);
            /// ```
            ///
            /// ```
            /// use ioctl_term_light::stderr;
            /// assert_eq!(stderr::get_dimensions(), None);
            /// ```
            pub fn get_dimensions() -> Option<(u16, u16)> {
                let ws = unsafe { ioctl_get_dimensions(FILENO) };
                if ws.ws_col == 0 || ws.ws_row == 0 {
                    None
                } else {
                    Some((ws.ws_col as u16, ws.ws_row as u16))
                }
            }
            /// Returns [Dimensions] process'es stdout, stdin or stderr.
            ///
            /// Examples
            ///
            /// ```
            /// use ioctl_term_light::Dimensions;
            /// use ioctl_term_light::stdout;
            ///
            /// assert_eq!(stdout::dimensions(), Dimensions::from((0u16, 0u16)));
            /// ```
            ///
            /// ```
            /// use ioctl_term_light::Dimensions;
            /// use ioctl_term_light::stdout;
            ///
            /// assert_eq!(stdout::dimensions(), Dimensions::from((0u16, 0u16)));
            /// ```
            ///
            /// ```
            /// use ioctl_term_light::Dimensions;
            /// use ioctl_term_light::stdout;
            ///
            /// assert_eq!(stdout::dimensions(), Dimensions::from((0u16, 0u16)));
            /// ```
            pub fn dimensions() -> Dimensions {
                Dimensions::from(get_dimensions())
            }

            #[cfg(test)]
            mod tests {
                use super::{dimensions, get_dimensions};
                use $crate::Dimensions;

                #[test]
                fn test_get_dimensions() {
                    let dimensions = get_dimensions();
                    assert_ne!(
                        dimensions, None,
                        "expected get_dimensions() to not return None"
                    );
                    assert_ne!(
                        dimensions,
                        Some((0, 0)),
                        "expected get_dimensions() to not return Some((0, 0))"
                    );
                }

                #[test]
                fn test_dimensions() {
                    let dimensions = dimensions();
                    let unexpected = Dimensions::from((0u16, 0u16));
                    assert_ne!(
                        dimensions, unexpected,
                        "expected get_dimensions() to not return {unexpected:#?}"
                    );
                }
            }
        }
    };
}
