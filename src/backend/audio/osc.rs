use rosc::OscPacket;
// use std::env;
use std::net::SocketAddrV4;
use std::str::FromStr;
use yansi::Paint;

/// OSC packet handler
pub fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", Paint::blue(msg.addr));
            match msg.args {
                Some(args) => {
                    println!("OSC arguments: {:?}", Paint::blue(args));
                }
                None => println!("No arguments in message."),
            }
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", Paint::blue(bundle));
        }
    }
}

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

// serde_osc stuff

use serde_bytes::ByteBuf;
use serde_osc::{de, ser};

/// Struct we'll serialize.
/// This represents a single OSC message with three arguments:
///   one of type 'i', 'f' and 'b', encoded in the order they appear in the struct.
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    address: String,
    // ByteBuf is the object we use for OSC "blobs".
    // It's a thin wrapper over Vec<u8> provided by Serde that allows
    // for more computationally-efficient serialization/deserialization.
    args: (i32, f32, ByteBuf),
}

/// Initialize OSC system
pub fn init() {
    let message = Message {
        address: "/audio/play".to_owned(),
        args: (1, 44100.0f32, ByteBuf::from(vec![0xde, 0xad, 0xbe, 0xef])),
    };
    println!("Serializing {:?}", message);
    // Serialize the message to an OSC packet stored in a Vec<u8>
    let as_vec = ser::to_vec(&message).unwrap();
    println!("Serialied to: {:?}", as_vec);
    // Deserialize an OSC packet contained in a Vec<u8> into the Message struct
    let received: Message = de::from_slice(&as_vec).unwrap();
    println!("Received: {:?}", Paint::blue(received));
}
