use super::Platform;

pub trait Builtin<P>: Send + Sync
where
    P: Platform + ?Sized,
{
    fn instantiate(
        &self,
        parent: &mut P::Container,
        environment: &mut P::Environment,
    ) -> P::Container;

    fn update(&self, container: &mut P::Container, environment: &mut P::Environment);
}
