use fsize::fsize;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct UIEdgeInsets {
    pub top: fsize,
    pub left: fsize,
    pub bottom: fsize,
    pub right: fsize,
}
