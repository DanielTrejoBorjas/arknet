use bincode::{Encode, Decode};

/// Packets sent from the CLIENT to the server.
#[derive(Encode, Decode, Debug)]
pub enum ClientPacketType {
    /// Request to initiate a connection to the server.
    ConnectionRequest,
    /// Input commands such as movement or actions.
    InputCommand,
    /// Ping packet to measure latency.
    Ping,
    /// Acknowledgment confirming receipt of server packets.
    Ack,
    /// Event messages like chat or non-physical interactions.
    EventMessage,
}

/// Packets sent from the SERVER to the client.
#[derive(Encode, Decode, Debug)]
pub enum ServerPacketType {
    /// Acceptance of a connection request.
    ConnectionAccept,
    /// Rejection of a connection request with a reason.
    ConnectionReject,
    /// Updates on the world state (positions, health, etc.).
    StateUpdate,
    /// Streaming of resources/assets to the client.
    ResourceStream,
    /// Response to a Ping packet.
    Pong,
    /// Acknowledgment confirming receipt of client packets.
    Ack,
    /// Messages or global events broadcasted to clients.
    EventMessage,
    /// Packet to inform about errors or validation failures.
    Error,
}

/// Packets that can be sent by BOTH CLIENT and SERVER.
#[derive(Encode, Decode, Debug)]
pub enum BothPacketType {
    /// Notification of voluntary or forced disconnection.
    Disconnect,
    /// Heartbeat packet to keep the connection alive.
    Heartbeat,
}

/// High-level packet type for serialization and networking.
#[derive(Encode, Decode, Debug)]
pub enum PacketType {
    Client(ClientPacketType),
    Server(ServerPacketType),
    Both(BothPacketType),
}

#[derive(Encode, Decode, Debug)]
pub struct NetworkPacket {
    pub packet_type: PacketType,
    pub payload: Vec<u8>,
}
