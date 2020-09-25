use fsize::fsize;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGPoint {
    pub x: fsize,
    pub y: fsize,
}

impl CGPoint {
    pub fn new(x: fsize, y: fsize) -> CGPoint {
        CGPoint { x, y }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGSize {
    pub width: fsize,
    pub height: fsize,
}

impl CGSize {
    pub fn new(width: fsize, height: fsize) -> CGSize {
        CGSize { width, height }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    pub fn new(x: fsize, y: fsize, width: fsize, height: fsize) -> CGRect {
        CGRect {
            origin: CGPoint::new(x, y),
            size: CGSize::new(width, height),
        }
    }
}
