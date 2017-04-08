//! Transport and I/O primitives for libp2p.

extern crate futures;
extern crate tokio_io;

/// Multi-address re-export.
pub extern crate multiaddr;

use futures::*;
use std::io::Error as IoError;
use tokio_io::{AsyncRead, AsyncWrite};

// Something more strongly-typed?
pub type ProtocolId = String;
pub type PeerId = String;

/// A logical wire between us and a peer. We can read and write through this asynchronously.
///
/// You can have multiple `Socket`s between you and any given peer.
pub trait Socket: AsyncRead + AsyncWrite { 
    /// Get the protocol ID this socket uses.
    fn protocol_id(&self) -> ProtocolId;

    /// Access the underlying connection.
    fn conn(&self) -> &Conn<Socket=Self>;
}

/// A connection between you and a peer.
pub trait Conn {
    /// The socket type this connection manages.
    type Socket;

    /// Initiate a socket between you and the peer on the given protocol.
    fn make_socket(&self, proto: ProtocolId) -> BoxFuture<Self::Socket, IoError>;
}

pub struct MultiAddr; // stub for multiaddr crate type.

/// A transport is a stream producing incoming connections.
/// These are transports or wrappers around them.
pub trait Transport {
    /// The raw connection.
    type RawConn: AsyncRead + AsyncWrite;

    /// The listener produces incoming connections.
    type Listener: Stream<Item=Self::RawConn>;

    /// A future which indicates currently dialing to a peer.
    type Dial: IntoFuture<Item=Self::RawConn, Error=IoError>;

    /// Listen on the given multi-addr.
    /// Returns the address back if it isn't supported.
    fn listen_on(&mut self, addr: MultiAddr) -> Result<Self::Listener, MultiAddr>;

    /// Dial to the given multi-addr.
    /// Returns either a future which may resolve to a connection,
    /// or gives back the multiaddress.
    fn dial(&mut self, addr: MultiAddr) -> Result<Self::Dial, MultiAddr>;
}