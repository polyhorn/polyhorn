use super::{Element, Manager, Platform};

pub trait Component<P>: Clone
where
    P: Platform + ?Sized,
{
    fn render(&self, manager: &mut Manager<P>) -> Element<P>;
}
