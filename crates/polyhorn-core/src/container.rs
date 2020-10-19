use super::Platform;

pub trait Container<P>
where
    P: Platform + ?Sized,
{
    fn mount(&mut self, container: &mut P::Container, environment: &mut P::Environment);

    fn unmount(&mut self);
}
