use super::Platform;

pub trait Container<P>
where
    P: Platform + ?Sized,
{
    fn mount(&mut self, container: &mut P::Container);

    fn unmount(&mut self);
}
