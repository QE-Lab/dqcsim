//! Channel abstractions.
//!
//! Defines a [`Channel`] trait based on a [`Sender`] and [`Receiver`] trait
//! combination to abstract over different channel types.
//!
//! [`Channel`]: ./trait.Channel.html
//! [`Sender`]: ./trait.Sender.html
//! [`Receiver`]: ./trait.Receiver.html

use crate::common::protocol::{GatestreamDown, GatestreamUp, PluginToSimulator, SimulatorToPlugin};
use ipc_channel::ipc;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Channel abstraction.
pub trait Channel {
    /// The message type of the sender.
    type SenderItem: Into<Self::ReceiverItem>;
    /// The message type of the receiver.
    type ReceiverItem: From<Self::SenderItem>;
    /// Sender side.
    type Sender: Sender<Item = Self::SenderItem>;
    /// Receiver side;
    type Receiver: Receiver<Item = Self::ReceiverItem>;
}

pub type CrossbeamChannel<T, U> = (crossbeam_channel::Sender<T>, crossbeam_channel::Receiver<U>);
pub type IpcChannel<T, U> = (ipc::IpcSender<T>, ipc::IpcReceiver<U>);

pub type SimulatorChannel = IpcChannel<SimulatorToPlugin, PluginToSimulator>;
pub type PluginChannel = IpcChannel<PluginToSimulator, SimulatorToPlugin>;
pub type UpstreamChannel = IpcChannel<GatestreamUp, GatestreamDown>;
pub type DownstreamChannel = IpcChannel<GatestreamDown, GatestreamUp>;

impl<T, U> Channel for CrossbeamChannel<T, U>
where
    T: Into<U>,
    U: From<T>,
{
    type SenderItem = T;
    type ReceiverItem = U;
    type Sender = crossbeam_channel::Sender<T>;
    type Receiver = crossbeam_channel::Receiver<U>;
}

impl<T, U> Channel for IpcChannel<T, U>
where
    T: Serialize + Into<U>,
    U: Serialize + for<'de> Deserialize<'de> + From<T>,
{
    type SenderItem = T;
    type ReceiverItem = U;
    type Sender = ipc::IpcSender<T>;
    type Receiver = ipc::IpcReceiver<U>;
}

/// Sender side of a channel.
pub trait Sender {
    /// The message type of the channel.
    type Item;
    /// The error type of the Result returned by the send function.
    type Error: Debug;

    /// Send a message.
    fn send(&self, item: Self::Item) -> Result<(), Self::Error>;
}

/// Receiver side of a channel.
pub trait Receiver {
    /// The message type of the channel.
    type Item;
    /// The error type of the Result returned by the recv function.
    type Error: Debug;

    /// Receive a message.
    fn recv(&self) -> Result<Self::Item, Self::Error>;
}

impl<T> Sender for crossbeam_channel::Sender<T> {
    type Item = T;
    type Error = crossbeam_channel::SendError<Self::Item>;

    fn send(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.send(item)
    }
}

impl<T: Serialize> Sender for ipc::IpcSender<T> {
    type Item = T;
    type Error = ipc_channel::Error;

    fn send(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.send(item)
    }
}

impl<T> Receiver for crossbeam_channel::Receiver<T> {
    type Item = T;
    type Error = crossbeam_channel::RecvError;

    fn recv(&self) -> Result<Self::Item, Self::Error> {
        self.recv()
    }
}

impl<T> Receiver for ipc::IpcReceiver<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Item = T;
    type Error = ipc_channel::ipc::IpcError;

    fn recv(&self) -> Result<Self::Item, Self::Error> {
        self.recv()
    }
}
