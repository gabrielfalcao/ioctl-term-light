#![allow(unused)]
pub(crate) mod c;

pub(crate) mod default;
#[doc(inline)]
pub use default::{cols, rows, tuple, tuple_from_raw_fd, Stdout, Stdin, Stderr};

pub mod dimensions;
#[doc(inline)]
pub use dimensions::Dimensions;

#[cfg(feature = "fd")]
pub(crate) mod fd;
#[cfg(feature = "fd")]
#[doc(inline)]
pub(crate) use fd::FileDescriptor;

#[cfg(any(
    feature = "stdio",
    feature = "stdin",
    feature = "stdout",
    feature = "stderr"
))]
#[cfg(any(
    feature = "stdio",
    feature = "stdin",
    feature = "stdout",
    feature = "stderr"
))]
pub mod stdio;

#[cfg(feature = "stdio")]
#[doc(inline)]
pub use stdio::{stderr, stdin, stdout};

#[cfg(not(feature = "stdio"))]
#[cfg(feature = "stdin")]
#[doc(inline)]
pub use stdio::stdin;

#[cfg(not(feature = "stdio"))]
#[cfg(feature = "stdout")]
#[doc(inline)]
pub use stdio::stdout;

#[cfg(not(feature = "stdio"))]
#[cfg(feature = "stderr")]
#[doc(inline)]
pub use stdio::stderr;
