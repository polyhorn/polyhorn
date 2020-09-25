#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Rect<T> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Insets<T> {
    pub top: T,
    pub trailing: T,
    pub bottom: T,
    pub leading: T,
}
