use super::Platform;
use derivative::Derivative;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Left
    }
}

#[derive(Derivative)]
#[derivative(Clone, Debug, Default)]
pub struct TextStyle<P>
where
    P: Platform + ?Sized,
{
    pub font: Option<P::Font>,
    pub color: Option<P::Color>,
    pub alignment: TextAlignment,
}
