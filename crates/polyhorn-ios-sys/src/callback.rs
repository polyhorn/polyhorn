use objc::runtime::*;
use objc::*;
use std::marker::PhantomData;

use super::Raw;

pub struct UICallback<T>
where
    T: Raw,
{
    object: *mut Object,
    marker: PhantomData<T>,
}

struct UICallbackData<T, F>
where
    F: FnMut(T) + Send + Sync,
    T: Raw,
{
    closure: F,
    marker: PhantomData<T>,
}

impl<T, F> UITypeErasedCallbackData for UICallbackData<T, F>
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

impl<T> UICallback<T>
where
    T: Raw,
{
    pub fn new<F>(value: F) -> Self
    where
        F: FnMut(T) + Send + Sync + 'static,
        T: Raw + 'static,
    {
        let data = UITypeErasedCallbackDataWrapper(Box::new(UICallbackData {
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

            let object: *mut Object = msg_send![class!(PLYCallback), alloc];

            let _: () = msg_send![object, initWithHook: hook as usize
                                                  free: free as usize
                                                  data: data as usize];

            UICallback::from_raw(object)
        }
    }
}

impl<T> Raw for UICallback<T>
where
    T: Raw,
{
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UICallback {
            object,
            marker: PhantomData,
        }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl<T> Drop for UICallback<T>
where
    T: Raw,
{
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
