//! Types to run a test feedback server.

use futures::channel::oneshot;
use futures::FutureExt;
use serde::Deserialize;
use simctl::Device;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use warp::Filter;

pub mod snapshot;

/// Disposable that can be dropped to stop a test feedback server.
pub struct Disposable(Option<oneshot::Sender<mpsc::Sender<()>>>);

/// Message that is sent to the test feedback server.
#[derive(Debug, Deserialize)]
pub enum Message {
    /// Creates a screenshot of the device and assigns it to the given test and
    /// snapshot names.
    Snapshot {
        /// Test name to attach to the screenshot.
        test_name: String,

        /// Name to attach to the snapshot. This field can be used to
        /// distinguish between multiple snapshots within the same test.
        snapshot_name: String,
    },

    /// Opens the given URL on the device.
    OpenURL(String),
}

impl Drop for Disposable {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let (tx, rx) = mpsc::channel();
            let _ = sender.send(tx);
            rx.recv().unwrap();
        }
    }
}

fn handler(device: Device) -> impl warp::Filter<Extract = impl warp::Reply> + Clone {
    let output = Arc::new(Mutex::new(snapshot::Output::new(
        "target/polyhorn-snapshots",
    )));

    warp::path!("polyhorn" / "tests" / String)
        .and(warp::post())
        .and(warp::body::json())
        .map(move |_id, message: Message| {
            match message {
                Message::OpenURL(url) => {
                    device.open_url(&url).unwrap();
                }
                Message::Snapshot {
                    test_name,
                    snapshot_name,
                } => {
                    let screenshot = device
                        .io()
                        .screenshot(
                            simctl::io::ImageType::Png,
                            simctl::io::Display::Internal,
                            simctl::io::Mask::Ignored,
                        )
                        .unwrap();

                    output.lock().unwrap().store(
                        snapshot::Metadata {
                            device: Some(device.name.to_owned()),
                            os: Some("iOS".to_owned()),
                            os_version: Some("14.0".to_owned()),
                            os_appearance: Some("light".to_owned()),
                            test_name: Some(test_name),
                            snapshot_name: Some(snapshot_name),
                        },
                        screenshot,
                    );
                }
            }

            "Ok"
        })
}

/// Starts a test feedback server for the given device. This function returns
/// the address (incl. port) that the server is bound to, and a disposable that
/// can be dropped to stop the server.
pub fn serve(device: Device) -> (SocketAddr, Disposable) {
    let (sender, receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            let (drop_sender, drop_receiver) = oneshot::channel();

            let server = warp::serve(handler(device));

            let (addr, runloop) =
                server.bind_ephemeral(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0));

            sender.send((addr, Disposable(Some(drop_sender)))).unwrap();

            let mut drop_receiver = drop_receiver.fuse();
            let mut runloop = runloop.fuse();

            futures::select! {
                tx = drop_receiver => {
                    std::mem::drop(runloop);

                    tx.unwrap().send(()).unwrap();
                },
                _ = runloop => {}
            };
        });
    });

    receiver.recv().unwrap()
}
