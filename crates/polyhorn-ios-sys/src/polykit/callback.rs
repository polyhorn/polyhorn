use objc::runtime::*;
use objc::*;
use std::marker::PhantomData;

use crate::Raw;

/// Callback that can be invoked from within Objective-C.
pub struct PLYCallback<T>
where
    T: Raw,
{
    object: *mut Object,
    marker: PhantomData<T>,
}

struct PLYCallbackData<T, F>
where
    F: FnMut(T) + Send + Sync,
    T: Raw,
{
    closure: F,
    marker: PhantomData<T>,
}

impl<T, F> UITypeErasedCallbackData for PLYCallbackData<T, F>
where
    F: FnMut(T) + Send + Sync,
    T: Raw,
{
    fn invoke(&mut self, object: *mut Object) {
        (self.closure)(unsafe { T::from_raw(object) })
    }
}

trait UITypeErasedCallbackData {
    fn invoke(&mut self, object: *mut Object);
}

struct UITypeErasedCallbackDataWrapper(Box<dyn UITypeErasedCallbackData>);

impl<T> PLYCallback<T>
where
    T: Raw,
{
    /// Initializes a newly allocated callback with the given closure.
    pub fn new<F>(value: F) -> Self
    where
        F: FnMut(T) + Send + Sync + 'static,
        T: Raw + 'static,
    {
        let data = UITypeErasedCallbackDataWrapper(Box::new(PLYCallbackData {
            closure: value,
            marker: PhantomData,
        }));

        unsafe {
            unsafe fn hook(data: *mut UITypeErasedCallbackDataWrapper, argument: *mut Object) {
                data.as_mut().unwrap().0.invoke(argument);
            }

            unsafe fn free(data: *mut UITypeErasedCallbackDataWrapper) {
                Box::from_raw(data);
            }

            let data = Box::into_raw(Box::new(data));

            let mut object: *mut Object = msg_send![class!(PLYCallback), alloc];

            object = msg_send![object, initWithHook: hook as usize
                                               free: free as usize
                                               data: data as usize];

            PLYCallback::from_raw(object)
        }
    }
}

impl<T> Raw for PLYCallback<T>
where
    T: Raw,
{
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYCallback {
            object,
            marker: PhantomData,
        }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl<T> Drop for PLYCallback<T>
where
    T: Raw,
{
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
