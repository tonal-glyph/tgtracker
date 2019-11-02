//@ # graph editor
//@
//@ The "graph editor" is a node graph like the one BuzzTracker made famous. Conrod and dsp-chain both use daggy, which is based on petgraph. Conrod has a node graph example in its codebase.
//@
//@ - [ ] Node - something that generates, processes, routes audio
//@     - [ ] Sound file ("phrase", recording, "sample")
//@     - [ ] VST plugin (instrument/effect)
//@     - [ ] AudioUnit
//@     - [ ] LV2 plugin
//@     - [ ] Ladspa plugin
//@     - [ ] ChucK code
//@     - [ ] FAUST code
//@ - [ ] Socket/port
//@     - [ ] Input, may be mono or stereo, colored red
//@     - [ ] Output, may be mono or stereo, colored green
//@ - [ ] Connection/Direction (edge)
//@     - [ ] Send targets
//@     - [ ] Receive targets

#![allow(dead_code)]
pub enum Channel {
    Mono,
    Stereo,
}
pub struct Socket {
    mode: Mode,
    channel: Channel,
}
pub struct Node {
    edges: Vec<Edge>,
    sockets: Vec<Socket>,
}
pub struct NodeId {
    id: u32,
}
pub enum Mode {
    Send,
    Receive,
}
pub struct Edge {
    mode: Mode,
    channel: Channel,
}
pub struct NodeGraph {
    pub graph: Vec<Node>,
}
