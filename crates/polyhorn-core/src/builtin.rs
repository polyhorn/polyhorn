use super::Platform;

pub trait Builtin<P>: Send + Sync
where
    P: Platform + ?Sized,
{
    fn instantiate(&self, environment: &mut P::Environment) -> P::Container;
}
