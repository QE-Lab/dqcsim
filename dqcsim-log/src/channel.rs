//! Channel abstraction.

use serde::Serialize;
use std::fmt::Debug;

/// Marker trait for to support multiple channels in a [`LogProxy`].
///
/// [`LogProxy`]: ../proxy/struct.LogProxy.html
pub trait Sender {
    /// The message type of the channel.
    type Item;
    /// The error type of the Result returned by the send function.
    type Error: Debug;
    fn send(&self, item: Self::Item) -> Result<(), Self::Error>;
}

/// [`crossbeam_channel::Sender<T>`] implements the [`Sender`] trait with the
/// non-blocking [`try_send`].
///
/// [`Sender`]: ./trait.Sender.html
/// [`try_send`]: ../../crossbeam_channel/struct.Sender.html#method.try_send
/// [`crossbeam_channel::Sender<T>`]: ../../crossbeam_channel/struct.Sender.html
impl<T> Sender for crossbeam_channel::Sender<T> {
    type Item = T;
    type Error = crossbeam_channel::TrySendError<Self::Item>;
    fn send(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.try_send(item)
    }
}

/// [`ipc_channel::ipc::IpcSender<T>`] requires `T` to be [`Serialize`]. Implements
/// [`Sender`] with the blocking [`send`].
///
/// [`Sender`]: ./trait.Sender.html
/// [`Serialize`]: ../../serde/trait.Serialize.html
/// [`send`]: ../../ipc_channel/ipc/struct.IpcSender.html#method.send
/// [`ipc_channel::ipc::IpcSender<T>`]: ../../ipc_channel/ipc/struct.IpcSender.html
impl<T: Serialize> Sender for ipc_channel::ipc::IpcSender<T> {
    type Item = T;
    type Error = ipc_channel::Error;
    fn send(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.send(item)
    }
}
