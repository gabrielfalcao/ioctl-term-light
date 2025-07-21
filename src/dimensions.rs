#![allow(unused)]
use crate::c::{c_int, ioctl_get_dimensions};
use crate::default::{tuple, tuple_from_raw_fd};
use std::os::fd::AsRawFd;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}
impl Dimensions {
    pub fn to_tuple<Size: From<u16>>(&self) -> (Size, Size) {
        (Size::from(self.width), Size::from(self.height))
    }
    pub fn to_slice<Size: From<u16>>(&self) -> [Size; 2] {
        [Size::from(self.width), Size::from(self.height)]
    }

    pub fn to_u16_tuple(&self) -> (u16, u16) {
        self.to_tuple::<u16>().into()
    }
    pub fn to_u16_slice(&self) -> (u16, u16) {
        self.to_slice::<u16>().into()
    }

    pub fn to_usize_tuple(&self) -> (usize, usize) {
        self.to_tuple::<usize>().into()
    }
    pub fn to_usize_slice(&self) -> (usize, usize) {
        self.to_slice::<usize>().into()
    }

    pub fn from_raw_fd<FileDescriptor: AsRawFd>(fd: FileDescriptor) -> Dimensions {
        let tuple = tuple_from_raw_fd(fd);
        Dimensions::from(tuple)
    }
    pub fn get() -> Dimensions {
        let tuple = tuple();
        Dimensions::from(tuple)
    }
}
impl<Size> Into<(Size, Size)> for Dimensions
where
    Size: From<u16>,
{
    fn into(self) -> (Size, Size) {
        self.to_tuple()
    }
}
impl<Size> Into<[Size; 2]> for Dimensions
where
    Size: From<u16>,
{
    fn into(self) -> [Size; 2] {
        self.to_slice()
    }
}

impl From<(u16, u16)> for Dimensions {
    fn from(dim: (u16, u16)) -> Dimensions {
        let (width, height) = dim;
        Dimensions { width, height }
    }
}

impl From<Option<(u16, u16)>> for Dimensions {
    fn from(dim: Option<(u16, u16)>) -> Dimensions {
        let (width, height) = match dim {
            Some(dim) => dim,
            None => (0, 0),
        };
        Dimensions { width, height }
    }
}

#[cfg(feature = "u8")]
mod dimensions_u8 {
    use super::Dimensions;
    impl From<(u8, u8)> for Dimensions {
        fn from(dim: (u8, u8)) -> Dimensions {
            let (width, height) = dim;
            let width = width as u16;
            let height = height as u16;
            Dimensions { width, height }
        }
    }

    impl From<Option<(u8, u8)>> for Dimensions {
        fn from(dim: Option<(u8, u8)>) -> Dimensions {
            let (width, height) = match dim {
                Some(dim) => dim,
                None => (0, 0),
            };
            let width = width as u16;
            let height = height as u16;

            Dimensions { width, height }
        }
    }
}
