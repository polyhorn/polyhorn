#![allow(non_camel_case_types)]

use libc::pipe;
use std::marker::PhantomData;
use std::mem::{size_of, MaybeUninit};
use std::os::raw::{c_int, c_void};

type ALooper_callbackFunc = extern "C" fn(fd: c_int, events: c_int, data: *mut c_void) -> c_int;

extern "C" {
    fn ALooper_forThread() -> *mut c_void;
    fn ALooper_acquire(looper: *mut c_void);
    fn ALooper_addFd(
        looper: *mut c_void,
        fd: c_int,
        ident: c_int,
        events: c_int,
        callback: ALooper_callbackFunc,
        data: *mut c_void,
    ) -> c_int;
    fn ALooper_release(looper: *mut c_void);
}

pub struct Looper<T>
where
    T: Send + 'static,
{
    sender: c_int,
    marker: PhantomData<T>,
}

impl<T> Looper<T>
where
    T: Send + 'static,
{
    pub fn new<F>(callback: F) -> Looper<T>
    where
        F: FnMut(T) + 'static,
    {
        extern "C" fn looper_callback(fd: c_int, _events: c_int, data: *mut c_void) -> c_int {
            log::error!("Got looper_callback!");
            let data = data as *mut TypeErasedLooperReceiverWrapper;

            unsafe {
                // From the docs: implementations should return 1 to continue
                // receiving callbacks, or 0 to have this file descriptor and
                // callback unregistered from the looper.
                if data.as_mut().unwrap().receiver.process_callback(fd) {
                    return 1;
                } else {
                    let _ = Box::from_raw(data);
                    return 0;
                }
            }
        }

        unsafe {
            // Get a reference to this thread's looper.
            let looper = ALooper_forThread();
            ALooper_acquire(looper);

            // Create a new send-receive pipe.
            let mut message_pipe = [c_int::default(); 2];
            pipe(message_pipe.as_mut_ptr());

            ALooper_addFd(
                looper,
                message_pipe[0],
                0,
                1,
                looper_callback,
                Box::into_raw(Box::new(TypeErasedLooperReceiverWrapper {
                    receiver: Box::new(LooperReceiver {
                        looper,
                        callback,
                        marker: PhantomData,
                    }),
                })) as *mut _,
            );

            Looper {
                sender: message_pipe[1],
                marker: PhantomData,
            }
        }
    }

    pub fn send(&self, message: T) {
        self.send_raw(Box::new(Some(message)));
    }

    fn send_raw(&self, message: Box<Option<T>>) {
        unsafe {
            let pointer = Box::into_raw(message);
            libc::write(
                self.sender,
                &pointer as *const *mut Option<T> as *const _,
                size_of::<*mut Option<T>>(),
            )
        };
    }
}

impl<T> Clone for Looper<T>
where
    T: Send + 'static,
{
    fn clone(&self) -> Self {
        Looper {
            sender: self.sender,
            marker: self.marker,
        }
    }
}

impl<T> Drop for Looper<T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        self.send_raw(Box::new(None))
    }
}

trait TypeErasedLooperReceiver {
    fn process_callback(&mut self, fd: c_int) -> bool;
}

struct TypeErasedLooperReceiverWrapper {
    receiver: Box<dyn TypeErasedLooperReceiver>,
}

struct LooperReceiver<T, F>
where
    F: FnMut(T),
{
    looper: *mut c_void,
    callback: F,
    marker: PhantomData<T>,
}

impl<T, F> TypeErasedLooperReceiver for LooperReceiver<T, F>
where
    F: FnMut(T),
{
    fn process_callback(&mut self, fd: c_int) -> bool {
        unsafe {
            let mut data = MaybeUninit::<*mut Option<T>>::uninit();
            let size = size_of::<*mut Option<T>>();

            if libc::read(fd, data.as_mut_ptr() as _, size) != size as isize {
                panic!("Could not read enough bytes from looper channel.");
            }

            match *Box::from_raw(data.assume_init()) {
                Some(data) => {
                    (self.callback)(data);
                    true
                }
                None => {
                    ALooper_release(self.looper);
                    false
                }
            }
        }
    }
}
