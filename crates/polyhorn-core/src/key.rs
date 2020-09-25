use std::cmp::PartialEq;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Key is a dynamically typed wrapper around any type that implements `Any + Eq
/// + Hash` (note that `Any` implies `'static`). Key acts as a transparent
/// wrapper for these traits, where `Eq` and `PartialEq` in particular perform
/// dynamic type checks before determining value equality.
#[derive(Clone, Debug)]
pub struct Key(Rc<dyn machinery::Keyable>);

impl Key {
    /// This function returns a new key with the given value. The prescribed
    /// trait bound on `machinery::Keyable` refers to a hidden trait that is
    /// automatically implemented for types that implement `Any + Eq + Hash`.
    /// This trait is hidden because it should not be implemented manually.
    pub fn new<T>(value: T) -> Key
    where
        T: machinery::Keyable + 'static,
    {
        Key(Rc::new(value))
    }
}

impl<T> From<Rc<T>> for Key
where
    T: machinery::Keyable + 'static,
{
    fn from(value: Rc<T>) -> Self {
        Key(value)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        self.0.eq(other.0.as_any())
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state)
    }
}

mod machinery {
    use std::any::Any;
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::hash::{Hash, Hasher};

    pub trait Keyable: Any + Debug {
        fn eq(&self, other: &dyn Any) -> bool;
        fn as_any(&self) -> &dyn Any;
        fn hash(&self, state: &mut dyn Hasher);
    }

    impl<T> Keyable for T
    where
        T: Debug + Eq + Hash + 'static,
    {
        fn eq(&self, other: &dyn Any) -> bool {
            match other.downcast_ref::<Self>() {
                Some(other) => PartialEq::eq(self, other),
                _ => false,
            }
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn hash(&self, mut state: &mut dyn Hasher) {
            T::hash(self, &mut state)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Key;

    #[test]
    fn test_key() {
        assert_eq!(Key::new((1, "Hello")), Key::new((1, "Hello")));
        assert_ne!(Key::new((1, "Hello")), Key::new((2, "Hello")));
    }
}
