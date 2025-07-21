# ioctl-term-light

Lightweight crate to retrieve the dimensions (.i.e.: width/height) of
a UNIX terminal in terms of rows/columns.

## Examples

```rust
use ioctl_term_light::{cols, rows};

assert_eq!(cols(), 130)
assert_eq!(rows(), 33)
```

```rust
use ioctl_term_light::{tuple_from_raw_fd, Stdout};

assert_eq!(tuple_from_raw_fd(Stdout), (130, 33));
```



## Crate Features

#### `fd`

Enables the [`FileDescriptor`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/enum.FileDescriptor.html) enum.


#### `any-fd`

Enables the `Any(i32)` variant in the [`FileDescriptor`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/enum.FileDescriptor.html) enum.


#### `stdio`

Enables the modules:

- [`ioctl_term_light::stderr`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/stderr/index.html)
- [`ioctl_term_light::stdin`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/stdin/index.html)
- [`ioctl_term_light::stdout`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/stdout/index.html)


#### `stderr`

Enables the module [`ioctl_term_light::stderr`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/stderr/index.html).

#### `stdin`

Enables the module [`ioctl_term_light::stdin`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/stdin/index.html).

#### `stdout`

Enables the module [`ioctl_term_light::stdout`](https://docs.rs/ioctl-term-light/latest/ioctl_term_light/stdout/index.html).


#### `u8`

Enables conversion from u16 to u8 within the dimensions module
