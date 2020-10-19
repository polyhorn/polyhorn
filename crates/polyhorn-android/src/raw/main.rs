#![allow(non_snake_case)]

use polyhorn_android_sys::{Activity, Bridged, Object, View};

// pub unsafe fn main(env: *mut std::ffi::c_void, object: *mut std::ffi::c_void) {
//     let mut activity = Activity::from_object(Object::new(env as *mut _, object as *mut _));

//     let mut parent = View::new(&activity);
//     parent.set_background_color(0, 255, 255, 1.0);

//     let mut a = View::new(&activity);
//     a.set_background_color(255, 0, 0, 1.0);
//     a.set_frame(20.0, 30.0, 200.0, 400.0);

//     let mut b = View::new(&activity);
//     b.set_background_color(0, 255, 0, 1.0);
//     b.set_frame(80.0, 40.0, 200.0, 200.0);

//     parent.add_view(&a);
//     parent.add_view(&b);

//     activity.set_content_view(&parent);
// }
