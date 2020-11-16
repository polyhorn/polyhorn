use std::future::Future;
use std::pin::Pin;

use super::Automator;

pub trait Test<'a> {
    fn call(&self, automator: &'a mut Automator) -> Pin<Box<dyn Future<Output = ()> + 'a>>;
}

impl<'a, T, F> Test<'a> for T
where
    T: Fn(&'a mut Automator) -> F,
    F: Future<Output = ()> + 'a,
{
    fn call(&self, automator: &'a mut Automator) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin((self)(automator))
    }
}

static mut TESTS: Option<Vec<(&str, &dyn for<'a> Test<'a>)>> = None;

/// Registers a unit test for the given function.
pub fn register(name: &'static str, test: &'static dyn for<'a> Test<'a>) {
    unsafe { TESTS.get_or_insert_with(|| vec![]).push((name, test)) }
}

/// Returns all registered unit tests.
pub fn all() -> &'static [(&'static str, &'static dyn for<'a> Test<'a>)] {
    unsafe { TESTS.get_or_insert_with(|| vec![]).as_slice() }
}

pub use ctor::ctor;

/// Registers a unit test for a function with the given name.
#[macro_export]
macro_rules! register {
    ($name:expr, $id:expr) => {
        #[$crate::inventory::ctor]
        fn register() {
            $crate::inventory::register($name, &$id);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        static mut CALLED: bool = false;

        async fn test_example(_automator: &mut Automator) {
            unsafe {
                CALLED = true;
            }
        }

        register!("test_example", test_example);

        // let (sender, _) = futures::channel::mpsc::channel(16);
        // let mut automator = Automator::new(sender);

        // all().iter().for_each(|f| f(&mut automator));
        // assert!(unsafe { CALLED });
    }
}
