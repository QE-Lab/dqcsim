use serde::Serialize;
use std::fmt::Debug;

/// Marker trait for to support multiple send channels in LogProxy.
pub trait Sender {
    type Item;
    type Error: Debug;
    fn send(&self, item: Self::Item) -> Result<(), Self::Error>;
}

/// crossbeam_channel::Sender implements the Sender trait with the non-blocking
/// try_send.
impl<T> Sender for crossbeam_channel::Sender<T> {
    type Item = T;
    type Error = crossbeam_channel::TrySendError<Self::Item>;
    fn send(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.try_send(item)
    }
}

/// ipc_channel::ipc::IpcSender<T> requires T to be Serialize.
/// Implements Sender with a blocking send.
impl<T: Serialize> Sender for ipc_channel::ipc::IpcSender<T> {
    type Item = T;
    type Error = ipc_channel::Error;
    fn send(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.send(item)
    }
}
