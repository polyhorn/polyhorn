mod queue;

pub use queue::QueueBound;

// use super::Instance;
// use std::sync::{Arc, Mutex};
// use tokio::stream::StreamExt;

// #[repr(C)]
// pub struct Springboard {
//     main: fn(),
//     application_did_finish_launching: fn(),
// }

// enum Message {
//     ApplicationDidFinishLaunching,
//     WorkItem(Box<dyn FnOnce() + Send + Sync>),
// }

// fn main() {
//     std::thread::Builder::new()
//         .name("com.glacyr.Polyhorn".to_owned())
//         .spawn(|| {
//             let mut runtime = tokio::runtime::Runtime::new().unwrap();
//             runtime.block_on(async {
//                 let instance = Arc::new(Mutex::new(None));
//                 let mut receiver = CHANNEL.1.lock().unwrap().take().unwrap();
//                 while let Some(message) = receiver.next().await {
//                     match message {
//                         Message::ApplicationDidFinishLaunching => {
//                             extern "C" {
//                                 #[allow(improper_ctypes)]
//                                 fn __poly_main() -> Arc<Instance>;
//                             }

//                             let instance = instance.clone();
//                             parachute(move || {
//                                 instance
//                                     .lock()
//                                     .unwrap()
//                                     .replace(Some(unsafe { __poly_main() }));
//                             })
//                         }
//                         Message::WorkItem(item) => item(),
//                     }
//                 }
//             });
//         })
//         .unwrap();
// }

// fn application_did_finish_launching() {
//     let _ = CHANNEL
//         .0
//         .lock()
//         .unwrap()
//         .try_send(Message::ApplicationDidFinishLaunching);
// }

// pub const SPRINGBOARD: Springboard = Springboard {
//     main,
//     application_did_finish_launching,
// };
