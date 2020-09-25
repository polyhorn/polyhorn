use derivative::Derivative;
use std::pin::Pin;
use std::rc::Rc;

/// Wraps a closure into a event listener that events can be emitted to. It acts
/// mostly like an optional reference-counted closure, but it is easier to work
/// with. Most importantly, the `emit` function silently ignores events emitted
/// to unbound event listeners.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Default(bound = ""))]
pub struct EventListener<T> {
    callback: Option<Pin<Rc<dyn Fn(T)>>>,
}

impl<T> EventListener<T> {
    /// Returns a new event listener that invokes the given callback when an
    /// event is emitted.
    pub fn new<F>(callback: F) -> EventListener<T>
    where
        F: Fn(T) + 'static,
    {
        EventListener {
            callback: Some(Rc::pin(callback)),
        }
    }

    /// Returns a boolean that indicates if this event listener is currently
    /// bound to a closure.
    pub fn is_some(&self) -> bool {
        self.callback.is_some()
    }

    /// Returns a boolean that indicates if this event listener is currently
    /// not bound to a closure.
    pub fn is_none(&self) -> bool {
        self.callback.is_none()
    }

    /// Emits the given event to this event listener.
    pub fn emit(&self, event: T) {
        match &self.callback {
            Some(callback) => callback(event),
            None => {}
        }
    }
}

impl<F, T> From<F> for EventListener<T>
where
    F: Fn(T) + 'static,
{
    fn from(value: F) -> Self {
        EventListener::new(value)
    }
}
